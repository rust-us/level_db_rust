use crate::traits::filter_policy_trait::FilterPolicy;
use crate::util::filter_policy::{AsBloomHash, FromPolicy};
use crate::util::filter_policy_bloom::BloomFilterPolicy;
use crate::util::hash::ToHash;
use crate::util::slice::Slice;

// ####################  BloomFilterPolicy test
#[test]
fn test_bloom_hash() {
    let val = "aabbccd";
    let slice: Slice = Slice::from_buf(val.as_bytes());

    let hash_val = BloomFilterPolicy::bloom_hash(&slice);
    let hash_val_1 = slice.bloom_hash();
    assert_eq!(hash_val, hash_val_1);
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

    // 如下三个值， 存放在 BloomFilter 中
    let s1 = Slice::try_from(String::from("hello")).unwrap();
    let s2 = Slice::try_from(String::from("world")).unwrap();
    let s3 = Slice::try_from(String::from("hello world")).unwrap();

    let mut keys : Vec<&Slice>  = Vec::new();
    keys.push(&s1);
    keys.push(&s2);
    keys.push(&s3);

    let bloom_filter: Slice = policy.create_filter(keys);

    // 验证通过
    let mut key_may_match = policy.key_may_match(
        &Slice::try_from(String::from("hello")).unwrap(),
        &bloom_filter);
    assert!(key_may_match);

    // 验证通过
    key_may_match = policy.key_may_match(&Slice::try_from(String::from("world")).unwrap(),
                                         &bloom_filter);
    assert!(key_may_match);

    // 因为不存在，所以验证不通过
    let mut key_not_match = policy.key_may_match(&Slice::try_from(String::from("x")).unwrap(),
                                                 &bloom_filter);
    assert!(!key_not_match);

    // 因为不存在，所以验证不通过
    key_not_match = policy.key_may_match(&Slice::try_from(String::from("helloworld")).unwrap(),
                                         &bloom_filter);
    assert!(!key_not_match);

    // 因为存在，所以验证通过
    let key_match = policy.key_may_match(&Slice::try_from(String::from("hello world")).unwrap(),
                                         &bloom_filter);
    assert!(key_match);

    // 因为不存在，所以验证不通过
    key_not_match = policy.key_may_match(&Slice::try_from(String::from("foo")).unwrap(),
                                         &bloom_filter);
    assert!(!key_not_match);

    // 验证通过
    let mut key_may_match = policy.key_may_match(
        &Slice::try_from(String::from("hello")).unwrap(),
        &bloom_filter);
    assert!(key_may_match);
}

/// 指定超长长度。可以超过放置的值
#[test]
fn test_create_filter_with_long_len(){
    let policy = BloomFilterPolicy::new(800);

    // 如下三个值， 存放在 BloomFilter 中
    let s1 = Slice::try_from(String::from("hello")).unwrap();
    let s2 = Slice::try_from(String::from("world")).unwrap();
    let s3 = Slice::try_from(String::from("hello world")).unwrap();

    let mut keys : Vec<&Slice>  = Vec::new();
    keys.push(&s1);
    keys.push(&s2);
    keys.push(&s3);

    let bloom_filter: Slice = policy.create_filter_with_len(600, keys);

    // 验证通过
    let mut key_may_match = policy.key_may_match(
        &Slice::try_from(String::from("hello")).unwrap(),
        &bloom_filter);
    assert!(key_may_match);

    // 验证通过
    key_may_match = policy.key_may_match(&Slice::try_from(String::from("world")).unwrap(),
                                         &bloom_filter);
    assert!(key_may_match);

    // 因为不存在，所以验证不通过
    let mut key_not_match = policy.key_may_match(&Slice::try_from(String::from("x")).unwrap(),
                                                 &bloom_filter);
    assert!(!key_not_match);

    // 因为不存在，所以验证不通过
    key_not_match = policy.key_may_match(&Slice::try_from(String::from("helloworld")).unwrap(),
                                         &bloom_filter);
    assert!(!key_not_match);

    // 因为存在，所以验证通过
    let key_match = policy.key_may_match(&Slice::try_from(String::from("hello world")).unwrap(),
                                         &bloom_filter);
    assert!(key_match);

    // 因为不存在，所以验证不通过
    key_not_match = policy.key_may_match(&Slice::try_from(String::from("foo")).unwrap(),
                                         &bloom_filter);
    assert!(!key_not_match);

    // 验证通过
    let mut key_may_match = policy.key_may_match(
        &Slice::try_from(String::from("hello")).unwrap(),
        &bloom_filter);
    assert!(key_may_match);
}

/// 指定端长度。放不开放置的值。 此时对于 BloomFilterPolicy 来讲不需要扩容
#[test]
fn test_create_filter_with_short_len(){
    let policy = BloomFilterPolicy::new(800);

    // 如下三个值， 存放在 BloomFilter 中
    let s1 = Slice::try_from(String::from("hello")).unwrap();
    let s2 = Slice::try_from(String::from("world")).unwrap();
    let s3 = Slice::try_from(String::from("hello world")).unwrap();

    let mut keys : Vec<&Slice>  = Vec::new();
    keys.push(&s1);
    keys.push(&s2);
    keys.push(&s3);

    let bloom_filter: Slice = policy.create_filter_with_len(2, keys);

    // 验证通过
    let mut key_may_match = policy.key_may_match(
        &Slice::try_from(String::from("hello")).unwrap(),
        &bloom_filter);
    assert!(key_may_match);

    // 验证通过
    key_may_match = policy.key_may_match(&Slice::try_from(String::from("world")).unwrap(),
                                         &bloom_filter);
    assert!(key_may_match);

    // 因为不存在，所以验证不通过
    let mut key_not_match = policy.key_may_match(&Slice::try_from(String::from("x")).unwrap(),
                                                 &bloom_filter);
    assert!(!key_not_match);

    // 因为不存在，所以验证不通过
    key_not_match = policy.key_may_match(&Slice::try_from(String::from("helloworld")).unwrap(),
                                         &bloom_filter);
    assert!(!key_not_match);

    // 因为存在，所以验证通过
    let key_match = policy.key_may_match(&Slice::try_from(String::from("hello world")).unwrap(),
                                         &bloom_filter);
    assert!(key_match);

    // 因为不存在，所以验证不通过
    key_not_match = policy.key_may_match(&Slice::try_from(String::from("foo")).unwrap(),
                                         &bloom_filter);
    assert!(!key_not_match);

    // 验证通过
    let mut key_may_match = policy.key_may_match(
        &Slice::try_from(String::from("hello")).unwrap(),
        &bloom_filter);
    assert!(key_may_match);
}