use crate::util::slice::Slice;

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

    /// 根据 keys 创建过滤器，并返回 bloom_filter Slice
    ///
    /// # Arguments
    ///
    /// * `keys`:  创建过滤器的数据清单
    ///
    /// returns: bloom_filter Slice
    ///
    /// # Examples
    ///
    /// ```
    ///    use crate::util::slice::Slice;
    ///
    ///    let mut keys : Vec<Slice>  = Vec::new();
    ///     keys.push(Slice::try_from(String::from("hello")).unwrap());
    ///     keys.push(Slice::try_from(String::from("world")).unwrap());
    ///
    ///     let policy = BloomFilterPolicy::new(800);
    ///     let bloom_filter: Slice = policy.create_filter(keys);
    /// ```
    fn create_filter(&self, keys: Vec<Slice>) -> Slice;

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