use std::ptr::null;
use crate::traits::filter_policy_trait::FilterPolicy;
use crate::util::bloom_filter;
use crate::util::filter_policy::{BloomFilterPolicy, FromPolicy};
use crate::util::slice::Slice;

#[test]
fn test_bloom_hash() {
    let val = "aabbccd";
    let slice: Slice = Slice::from_buf(val.as_bytes());

    let hash_val = BloomFilterPolicy::bloom_hash(slice);
    assert_eq!(hash_val, 2085241752);
}

#[test]
fn test_new() {
    let bloom_filter: BloomFilterPolicy = BloomFilterPolicy::new(8);
    assert_eq!(bloom_filter.from_bits_per_key(), 8);
    assert_eq!(bloom_filter.from_k(), 6);

    let bloom_filter = BloomFilterPolicy::new(800);
    assert_eq!(bloom_filter.from_bits_per_key(), 800);
    assert_eq!(bloom_filter.from_k(), 30);

}

// #[test]
// fn test_create_filter() {
//     let bloom_filter: BloomFilterPolicy = BloomFilterPolicy::create_filter(8);
//     println!("{}", "aa")
//
// }