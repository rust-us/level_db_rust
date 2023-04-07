use std::borrow::BorrowMut;
use std::cmp::max;
use std::usize::MAX;
use crate::traits::coding_trait::CodingTrait;
use crate::traits::filter_policy_trait::FilterPolicy;
use crate::util::coding::Coding;
use crate::util::hash::Hash;
use crate::util::slice::Slice;

/// 内部使用。专门用于测试用例的  FilterPolicy
pub struct TestHashFilter {
    //.
}

impl TestHashFilter {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl FilterPolicy for TestHashFilter {
    fn name(&self) -> String {
        String::from("TestHashFilter")
    }

    fn create_filter(&self, keys: Vec<&Slice>) -> Slice {
        // 每个 key 都会 hash_code 转为 u32， 所以 * 4
        let mut len: usize = keys.len() * 4;

        self.create_filter_with_len(len, keys)
    }

    fn create_filter_with_len(&self, capacity: usize, keys: Vec<&Slice>) -> Slice {
        // Actually capacity
        let mut len: usize = capacity;

        let need_capacity = keys.len() * 4;
        // 指定大小和 need_capacity 取最大值
        len = max(len, need_capacity);

        let mut dst_chars = vec![0; len];
        let bloom_filter = dst_chars.borrow_mut();

        let mut offset: usize = 0;
        // for [0, len)
        for i in 0..keys.len() {
            let h = Hash::hash_code(keys[i].as_ref(), 1); // seed 固定为 1
            offset = Coding::put_fixed32(bloom_filter, offset, h);
        }

        Slice::from_buf(bloom_filter)
    }

    fn key_may_match(&self, key: &Slice, bloom_filter: &Slice) -> bool {
        let h = Hash::hash_code(key.to_vec().as_ref(), 1);

        let bloom_filter_data: &[u8] = bloom_filter.as_ref();
        let len = bloom_filter_data.len();

        let mut pos = 0;
        while pos < len {
            let buf = &bloom_filter_data[pos..(pos+4)];

            let h_bl = Coding::decode_fixed32(buf);
            if h == h_bl {
                return true
            }

            pos += 4;
        }

        false
    }
}

// ####################  FilterPolicy test
#[test]
fn test_create_filter() {
    let policy = TestHashFilter::new();

    // 如下三个值， 存放在 BloomFilter 中
    let s1 = Slice::try_from(String::from("hello")).unwrap();
    let s2 = Slice::try_from("world").unwrap();
    let s3 = Slice::try_from("hello world").unwrap();

    let mut keys : Vec<&Slice>  = Vec::new();
    keys.push(&s1);
    keys.push(&s2);
    keys.push(&s3);

    let bloom_filter: Slice = policy.create_filter(keys);

    // 验证通过
    let mut key_may_match = policy.key_may_match(
        &Slice::try_from("hello").unwrap(),
        &bloom_filter);
    assert!(key_may_match);

    // 验证通过
    key_may_match = policy.key_may_match(&Slice::try_from("world").unwrap(),
                                         &bloom_filter);
    assert!(key_may_match);

    // 因为不存在，所以验证不通过
    let key_not_match = policy.key_may_match(&Slice::try_from("helloworld").unwrap(),
                                         &bloom_filter);
    assert!(!key_not_match);

    // 因为存在，所以验证通过
    let key_may_match = policy.key_may_match(&Slice::try_from("hello world").unwrap(),
                                         &bloom_filter);
    assert!(key_may_match);

    // 因为不存在，所以验证不通过
    let key_not_match = policy.key_may_match(&Slice::try_from("foo").unwrap(),
                                         &bloom_filter);
    assert!(!key_not_match);

    // 验证通过
    let mut key_may_match = policy.key_may_match(
        &Slice::try_from("hello").unwrap(),
        &bloom_filter);
    assert!(key_may_match);

    // 因为不存在，所以验证不通过
    let mut key_not_match = policy.key_may_match(&Slice::try_from("x").unwrap(),
                                                 &bloom_filter);
    assert!(!key_not_match);
}

/// 指定超长长度。可以超过放置的值
#[test]
fn test_create_filter_with_long_len() {
    let policy = TestHashFilter::new();

    // 如下三个值， 存放在 BloomFilter 中
    let s1 = Slice::try_from(String::from("hello")).unwrap();
    let s2 = Slice::try_from("world").unwrap();
    let s3 = Slice::try_from("hello world").unwrap();

    let mut keys : Vec<&Slice>  = Vec::new();
    keys.push(&s1);
    keys.push(&s2);
    keys.push(&s3);

    let bloom_filter: Slice = policy.create_filter_with_len(500, keys);

    // 验证通过
    let mut key_may_match = policy.key_may_match(
        &Slice::try_from("hello").unwrap(),
        &bloom_filter);
    assert!(key_may_match);

    // 验证通过
    key_may_match = policy.key_may_match(&Slice::try_from("world").unwrap(),
                                         &bloom_filter);
    assert!(key_may_match);

    // 因为不存在，所以验证不通过
    let key_not_match = policy.key_may_match(&Slice::try_from("helloworld").unwrap(),
                                             &bloom_filter);
    assert!(!key_not_match);

    // 因为存在，所以验证通过
    let key_may_match = policy.key_may_match(&Slice::try_from("hello world").unwrap(),
                                             &bloom_filter);
    assert!(key_may_match);

    // 因为不存在，所以验证不通过
    let key_not_match = policy.key_may_match(&Slice::try_from("foo").unwrap(),
                                             &bloom_filter);
    assert!(!key_not_match);

    // 验证通过
    let mut key_may_match = policy.key_may_match(
        &Slice::try_from("hello").unwrap(),
        &bloom_filter);
    assert!(key_may_match);

    // 因为不存在，所以验证不通过
    let mut key_not_match = policy.key_may_match(&Slice::try_from("x").unwrap(),
                                                 &bloom_filter);
    assert!(!key_not_match);
}

/// 指定端长度。放不开放置的值。 此时需要扩容
#[test]
fn test_create_filter_with_short_len() {
    let policy = TestHashFilter::new();

    // 如下三个值， 存放在 BloomFilter 中
    let s1 = Slice::try_from(String::from("hello")).unwrap();
    let s2 = Slice::try_from("world").unwrap();
    let s3 = Slice::try_from("hello world").unwrap();

    let mut keys : Vec<&Slice>  = Vec::new();
    keys.push(&s1);
    keys.push(&s2);
    keys.push(&s3);

    let bloom_filter: Slice = policy.create_filter_with_len(5, keys);

    // 验证通过
    let mut key_may_match = policy.key_may_match(
        &Slice::try_from("hello").unwrap(),
        &bloom_filter);
    assert!(key_may_match);

    // 验证通过
    key_may_match = policy.key_may_match(&Slice::try_from("world").unwrap(),
                                         &bloom_filter);
    assert!(key_may_match);

    // 因为不存在，所以验证不通过
    let key_not_match = policy.key_may_match(&Slice::try_from("helloworld").unwrap(),
                                             &bloom_filter);
    assert!(!key_not_match);

    // 因为存在，所以验证通过
    let key_may_match = policy.key_may_match(&Slice::try_from("hello world").unwrap(),
                                             &bloom_filter);
    assert!(key_may_match);

    // 因为不存在，所以验证不通过
    let key_not_match = policy.key_may_match(&Slice::try_from("foo").unwrap(),
                                             &bloom_filter);
    assert!(!key_not_match);

    // 验证通过
    let mut key_may_match = policy.key_may_match(
        &Slice::try_from("hello").unwrap(),
        &bloom_filter);
    assert!(key_may_match);

    // 因为不存在，所以验证不通过
    let mut key_not_match = policy.key_may_match(&Slice::try_from("x").unwrap(),
                                                 &bloom_filter);
    assert!(!key_not_match);
}
