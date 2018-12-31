# rsbloom &emsp; [![Build Status]][travis] [![Latest Version]][crates.io] [![Rustc Version]][rustc] [![License]][crates.io]

[Build Status]: https://travis-ci.org/alexanderbez/rust-bloom.svg?branch=master
[travis]: https://travis-ci.org/alexanderbez/rust-bloom
[Latest Version]: https://img.shields.io/crates/v/rsbloom.svg
[crates.io]: https://crates.io/crates/rsbloom
[Rustc Version]: https://img.shields.io/badge/rustc-1.31+-blue.svg
[rustc]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[License]: https://img.shields.io/crates/l/rsbloom.svg

A simple implementation of a Bloom filter, a space-efficient probabilistic data
structure.

## Bloom Filters

A Bloom filter is a space-efficient probabilistic data structure that is
used to test whether an element is a member of a set. It allows for queries
to return: "possibly in set" or "definitely not in set". Elements can be
added to the set, but not removed; the more elements that are added to the
set, the larger the probability of false positives. It has been shown that
fewer than 10 bits per element are required for a 1% false positive
probability, independent of the size or number of elements in the set.

The provided implementation allows you to create a Bloom filter specifying
the approximate number of items expected to inserted and an optional false
positive probability. It also allows you to approximate the total number of
items in the filter.

## Enhanced Double Hashing

Enhanced double hashing is used to set bit positions within a bit vector.
The choice for double hashing was shown to be effective without any loss in
the asymptotic false positive probability, leading to less computation and
potentially less need for randomness in practice by Adam Kirsch and
Michael Mitzenmacher in [Less Hashing, Same Performance: Building a Better Bloom Filter](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.152.579&rep=rep1&type=pdf).

The enhanced double hash takes the form of the following formula:

g<sub>i</sub>(x) = (H<sub>1</sub>(x) + iH<sub>2</sub>(x) + f(i)) mod m, where

H<sub>1</sub>
is Murmur3 128-bit, H<sub>2</sub> is xxHash 64-bit, and f(i) = i<sup>3</sup>

## Usage

Add the `rsbloom` dependency to your `Cargo.toml`:

```toml
[dependencies]
rsbloom = "0.1.0"
```

## Example

```rust
use rsbloom::BloomFilter;

fn main() {
  let approx_items = 100;
  let mut bf = BloomFilter::new(approx_items);

  bf.set(&"foo");
  bf.set(&"bar");

  bf.has(&"foo"); // true
  bf.has(&"bar"); // true
  bf.has(&"baz"); // false

  bf.num_items_approx(); // 2
}
```

## Tests

```shell
make test
```

## Benchmarks

```shell
make bench
```
