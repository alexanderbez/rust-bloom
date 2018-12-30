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

//! A simple implementation of a Bloom filter, a space-efficient probabilistic
//! data structure.
//!
//! # Bloom Filters
//!
//! A Bloom filter is a space-efficient probabilistic data structure that is
//! used to test whether an element is a member of a set. It allows for queries
//! to return: "possibly in set" or "definitely not in set". Elements can be
//! added to the set, but not removed; the more elements that are added to the
//! set, the larger the probability of false positives. It has been shown that
//! fewer than 10 bits per element are required for a 1% false positive
//! probability, independent of the size or number of elements in the set.
//!
//! The provided implementation allows you to create a Bloom filter specifying
//! the approximate number of items expected to inserted and an optional false
//! positive probability. It also allows you to approximate the total number of
//! items in the filter.
//!
//! # Enhanced Double Hashing
//!
//! Enhanced double hashing is used to set bit positions within a bit vector.
//! The choice for double hashing was shown to be effective without any loss in
//! the asymptotic false positive probability, leading to less computation and
//! potentially less need for randomness in practice by Adam Kirsch and
//! Michael Mitzenmacher in
//! [Less Hashing, Same Performance: Building a Better Bloom Filter](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.152.579&rep=rep1&type=pdf).
//!
//! The enhanced double hash takes the form of the following formula:
//!
//! g<sub>i</sub>(x) = (H<sub>1</sub>(x) + iH<sub>2</sub>(x) + f(i)) mod m, where
//!
//! H<sub>1</sub>
//! is Murmur3 128-bit, H<sub>2</sub> is xxHash 64-bit, and f(i) = i<sup>3</sup>
//!
//!
//! # Example
//!
//! ```rust
//! use rsbloom::BloomFilter;
//!
//! fn main() {
//!   let approx_items = 100;
//!   let mut bf = BloomFilter::new(approx_items);
//!
//!   bf.set(&"foo");
//!   bf.set(&"bar");
//!
//!   bf.has(&"foo"); // true
//!   bf.has(&"bar"); // true
//!   bf.has(&"baz"); // false
//!
//!   bf.num_items_approx(); // 2
//! }
//! ```
#![crate_type = "lib"]
#![crate_name = "rsbloom"]
#![warn(missing_docs)]

// import library modules
pub mod bloom;

// re-export library modules
pub use self::bloom::BloomFilter;
