use std::ptr::null;
use crate::util::bloom_filter;
use crate::util::filter_policy::BloomFilterPolicy;

#[test]
fn test_new() {
    let bloom_filter = BloomFilterPolicy::new(8);
    println!("hash:{}", "a");
    // assert_eq!(bloom_filter, null());

    let bloom_filter = BloomFilterPolicy::new(800);
    println!("hash:{}", "a");

}