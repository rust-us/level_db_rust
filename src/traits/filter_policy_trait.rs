use crate::util::slice::Slice;

/// 用于key过滤，可以快速的排除不存在的key
pub trait FilterPolicy {

    /// filter的名字
    /// Return the name of this policy.  Note that if the filter encoding
    /// changes in an incompatible way, the name returned by this method
    /// must be changed.  Otherwise, old incompatible filters may be
    /// passed to methods of this type.
    fn name() -> String;

    fn create_filter(&self, keys: Slice, n: u32) -> String;

    fn key_may_match(key: &Slice, filter: &Slice) -> bool;
}