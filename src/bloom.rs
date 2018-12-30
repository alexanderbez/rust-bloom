// The MIT License
// Copyright (c) 2018 Aleksandr Bezobchuk
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! A simple and intuitive implementation of a Bloom filter using enhanced
//! double hashing.

use bit_vec::BitVec;
use fasthash::RandomState;
use fasthash::{murmur3, xx};
use std::hash::{BuildHasher, Hash, Hasher};

const LN_SQR: f64 = core::f64::consts::LN_2 * core::f64::consts::LN_2;
const SET_BIT: bool = true;
const UNSET_BIT: bool = false;

/// The default false positive probability value which is 1%.
pub const DEFAULT_FALSE_POS: f64 = 0.01;

/// A Bloom filter implementation that tracks the total number of set bits along
/// with the underlying bit vector and hashing functions, Murmur3 and xxHash.
pub struct BloomFilter<R: BuildHasher, S: BuildHasher> {
  bit_vec: BitVec,
  num_hashes: u64,
  set_bits: u64,
  murmur_hasher: R,
  xx_hasher: S,
}

impl BloomFilter<RandomState<murmur3::Murmur3_x64_128>, RandomState<xx::XXHash64>> {
  /// Return a new Bloom filter with a given number of approximate items to set.
  /// The default false positive probability is set and defined by DEFAULT_FALSE_POS.
  pub fn new(approx_items: u64) -> Self {
    BloomFilter::new_with_rate(approx_items, DEFAULT_FALSE_POS)
  }

  /// Return a new Bloom filter with a given number of approximate items to set
  /// and a desired false positive probability.
  pub fn new_with_rate(approx_items: u64, fp_prob: f64) -> Self {
    let num_bits = optimal_num_bits(approx_items, fp_prob);
    let num_hashes = optimal_num_hashes(num_bits, approx_items);

    BloomFilter {
      bit_vec: BitVec::from_elem(num_bits as usize, UNSET_BIT),
      num_hashes: num_hashes,
      set_bits: 0,
      murmur_hasher: RandomState::<murmur3::Murmur3_x64_128>::new(),
      xx_hasher: RandomState::<xx::XXHash64>::new(),
    }
  }
}

impl<R, S> BloomFilter<R, S>
where
  R: BuildHasher,
  S: BuildHasher,
{
  /// Set an object in the Bloom filter. This operation is idempotent in regards
  /// to each unique object. Each object must implement the Hash trait.
  pub fn set<T: Hash>(&mut self, obj: &T) {
    let mut hasher_one = self.murmur_hasher.build_hasher();
    let mut hasher_two = self.xx_hasher.build_hasher();

    obj.hash(&mut hasher_one);
    obj.hash(&mut hasher_two);

    let h1 = hasher_one.finish();
    let h2 = hasher_two.finish();

    for i in 0..self.num_hashes {
      let bit_idx = self.enhanced_double_hash(h1, h2, i) as usize;

      // Unwrap option<bool> and if the bit is unset then we increment the set
      // bits.
      //
      // NOTE: We should not panic here as enhanced_double_hash ensures the
      // index is within bounds via modulo bit vector table size.
      if self.bit_vec.get(bit_idx).unwrap() == UNSET_BIT {
        self.set_bits += 1;
      }

      self.bit_vec.set(bit_idx, SET_BIT);
    }
  }

  /// Returns a bool reflecting if a given object is 'most likely' in the Bloom
  /// filter or not. There is a possibility for a false positive with the
  /// probability being under the Bloom filter's p value, but a false negative
  /// will never occur.
  pub fn has<T: Hash>(&self, obj: &T) -> Option<bool> {
    let mut hasher_one = self.murmur_hasher.build_hasher();
    let mut hasher_two = self.xx_hasher.build_hasher();

    obj.hash(&mut hasher_one);
    obj.hash(&mut hasher_two);

    let h1 = hasher_one.finish();
    let h2 = hasher_two.finish();

    for i in 0..self.num_hashes {
      let bit_idx = self.enhanced_double_hash(h1, h2, i) as usize;

      // Unwrap option<bool> and if the bit is not set, then we short-circuit
      // and return false.
      //
      // NOTE: We should not panic here as enhanced_double_hash ensures the
      // index is within bounds via modulo bit vector table size.
      if self.bit_vec.get(bit_idx).unwrap() != SET_BIT {
        return Some(false);
      }
    }

    Some(true)
  }

  /// Returns the approximate total number of objects set in the Bloom filter.
  pub fn num_items_approx(&self) -> u64 {
    let m = self.bit_vec.len() as f64;
    let k = self.num_hashes as f64;
    let x = self.set_bits as f64;
    (-(m / k) * (1.0 - (x / m)).ln()) as u64
  }

  fn enhanced_double_hash(&self, h1: u64, h2: u64, i: u64) -> u64 {
    let r = h1.wrapping_add(i.wrapping_mul(h2)).wrapping_add(i.pow(3));
    r % self.bit_vec.len() as u64
  }
}

/// Return the optimal bit vector size for a Bloom filter given an approximate
/// size approx_items and a desired false positive probability fp_prob.
fn optimal_num_bits(approx_items: u64, fp_prob: f64) -> u64 {
  (-((fp_prob.ln() * (approx_items as f64)) / LN_SQR)).ceil() as u64
}

/// Return the optimal number of hash 'functions' for a Bloom filter given a
/// bit vector size num_bits and an approximate set size approx_items.
fn optimal_num_hashes(num_bits: u64, approx_items: u64) -> u64 {
  (((num_bits / approx_items) as f64) * core::f64::consts::LN_2).ceil() as u64
}

#[cfg(test)]
mod tests {
  use super::*;
  use rand::distributions::Alphanumeric;
  use rand::{thread_rng, Rng};
  use std::collections::HashSet;

  fn random_str(len: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(len).collect()
  }

  #[test]
  fn test_bloom_filter() {
    let n = 1000;
    let mut items = HashSet::<String>::new();

    // generate random strings to insert
    for _ in 0..n {
      items.insert(random_str(30));
    }

    let mut bf = BloomFilter::new(items.len() as u64);

    // test inclusion
    for item in items.iter() {
      bf.set(item);

      let exists = bf.has(item).unwrap();
      assert_eq!(
        exists, true,
        "item {} should result in a positive inclusion",
        item,
      );
    }

    // test false negatives
    for _ in 0..n {
      let item = random_str(30);
      let exists = bf.has(&item).unwrap();

      if items.contains(&item) {
        assert_eq!(exists, true, "item {} resulted in a false negative", item);
      }
    }
  }

  #[test]
  fn test_optimal_num_bits() {
    assert_eq!(optimal_num_bits(10, 0.04), 67);
    assert_eq!(optimal_num_bits(5000, 0.01), 47926);
    assert_eq!(optimal_num_bits(100000, 0.01), 958506);
  }

  #[test]
  fn test_optimal_num_hashes() {
    assert_eq!(optimal_num_hashes(67, 10), 5);
    assert_eq!(optimal_num_hashes(47926, 5000), 7);
    assert_eq!(optimal_num_hashes(958506, 100000), 7);
  }
}
