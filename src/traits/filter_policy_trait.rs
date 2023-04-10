use std::sync::Arc;
use crate::util::slice::Slice;


/// FilterPolicy 的 `Arc<Box<dyn FilterPolicy>>` 别名
pub type FilterPolicyPtr = Arc<Box<dyn FilterPolicy>>;

/// 用于key过滤，可以快速的排除不存在的key
pub trait FilterPolicy {

    ///
    /// filter的名字
    /// Return the name of this policy.  Note that if the filter encoding
    /// changes in an incompatible way, the name returned by this method
    /// must be changed.  Otherwise, old incompatible filters may be
    /// passed to methods of this type.
    ///
    fn name(&self) -> String;

    fn create_filter(&self, keys: Vec<&Slice>) -> Slice;

    ///
    /// 使用一系列key来创建一个 bloom filter，并返回 bloom filter
    ///
    /// 有n个整数set，以及一个m位的bit数组，以及k个哈希函数。m[i]表示访问第i个bit位。
    ///
    /// # Arguments
    ///
    /// * `capacity`: 构造的 BloomFilter 的长度
    /// * `keys`: 创建过滤器的数据清单
    ///
    /// returns: bloom filter Slice
    ///
    /// # Examples
    ///
    /// ```
    ///     use level_db_rust::util::filter_policy_bloom::BloomFilterPolicy;
    ///     use level_db_rust::util::slice::Slice;
    ///
    ///     let mut keys : Vec<&Slice>  = Vec::new();
    ///     keys.push(&Slice::try_from(String::from("hello")).unwrap());
    ///     keys.push(&Slice::try_from(String::from("world")).unwrap());
    ///
    ///     let policy = BloomFilterPolicy::new();
    ///     let bloom_filter: Slice = policy.create_filter(keys);
    /// ```
    fn create_filter_with_len(&self, capacity: usize, keys: Vec<&Slice>) -> Slice;

    ///
    ///
    /// # Arguments
    ///
    /// * `key`:
    /// * `bloom_filter`:
    ///
    /// returns: bool
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn key_may_match(&self, key: &Slice, bloom_filter: &Slice) -> bool;
}