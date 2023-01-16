use std::ptr::null;
use crate::traits::filter_policy_trait::FilterPolicy;
use crate::util::bloom_filter;
use crate::util::filter_policy::{BloomFilterPolicy, FromPolicy};
use crate::util::slice::Slice;

// ####################  BloomFilterPolicy test
#[test]
fn test_bloom_hash() {
    let val = "aabbccd";
    let slice: Slice = Slice::from_buf(val.as_bytes());

    let hash_val = BloomFilterPolicy::bloom_hash(&slice);
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

// ####################  FilterPolicy test
#[test]
fn test_create_filter() {
    let policy = BloomFilterPolicy::new(800);

    let mut keys : Vec<Slice>  = Vec::new();
    keys.push(Slice::try_from(String::from("hello")).unwrap());
    keys.push(Slice::try_from(String::from("world")).unwrap());

    let bloom_filter: Slice = policy.create_filter(keys);

    let mut key_may_match = policy.key_may_match(
        &Slice::try_from(String::from("hello")).unwrap(),
        &bloom_filter);
    assert!(key_may_match);

    // key_may_match = policy.key_may_match(&Slice::try_from(String::from("world")).unwrap(),
    //                                      &bloom_filter);
    // assert!(key_may_match);

    // let mut un_key_may_match = policy.key_may_match(&Slice::try_from(String::from("x")).unwrap(),
    //                                      &bloom_filter);
    // assert!(!un_key_may_match);
    //
    // un_key_may_match = policy.key_may_match(&Slice::try_from(String::from("foo")).unwrap(),
    //                                         &bloom_filter);
    // assert!(!un_key_may_match);
}

// a(&policy1);
// fn a(a: &dyn FilterPolicy) {
//    //.
// }