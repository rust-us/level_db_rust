use crate::util::slice::Slice;

/// 用于key过滤，可以快速的排除不存在的key
pub trait FilterPolicy {

    /// filter的名字
    /// Return the name of this policy.  Note that if the filter encoding
    /// changes in an incompatible way, the name returned by this method
    /// must be changed.  Otherwise, old incompatible filters may be
    /// passed to methods of this type.
    fn name() -> String;

    /// 根据指定的参数创建过滤器，并返回结果， 结果为dst的原始内容 + append结果。
    /// 参数keys[0,n-1]包含依据用户提供的comparator排序的key列表--可重复，
    /// 并把根据这些key创建的filter追加到 dst中。
    ///
    /// # Arguments
    ///
    /// * `keys`:
    /// * `n`:
    /// * `dst`:
    ///
    /// returns: String
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn create_filter(&self, keys: Slice, n: u32, dst: String) -> String;

    fn key_may_match(key: &Slice, filter: &Slice) -> bool;
}