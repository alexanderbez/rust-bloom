//! A simple example showing the use of a Bloom filter.

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