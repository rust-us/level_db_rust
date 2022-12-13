use std::cmp::Ordering;
use crate::util::slice::Slice;

/// A Comparator object provides a total order across slices that are
/// used as keys in an sstable or a database.  A Comparator implementation
/// must be thread-safe since leveldb may invoke its methods concurrently
/// from multiple threads.
pub trait ComparatorTrait {

    fn compare(&self, a: &Slice, b: &Slice) -> Option<Ordering>;

    /// 返回comparator的名字
    fn get_name() -> String;

    /// 减少像index blocks这样的内部数据结构占用的空间
    /// 这两个函数作用是减少像index blocks这样的内部数据结构占用的空间。
    /// 如果 start < limit，就在[start,limit)中找到一个短字符串，并赋给*start返回。
    /// 当然返回的*start可能没变化（start==limit），此时这个函数相当于啥都没干，这也是正确的。
    fn find_shortest_separator(&self, start: &String, limit:&Slice) -> String;

    /// 减少像index blocks这样的内部数据结构占用的空间
    /// 将 key变成一个比原*key大的短字符串，并赋值给 key返回。
    fn find_short_successor(&self, key: &String) -> String;

}