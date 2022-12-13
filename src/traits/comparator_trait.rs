use std::cmp::Ordering;
use crate::util::slice::Slice;

/// 比较器
pub trait ComparatorTrait {

    ///  Slice 的大小比较, 按字典逐字节序进行比较
    ///
    /// # Arguments
    ///
    /// * `a`: 参与排序的 Slice
    /// * `b`: 参与排序的 Slice
    ///
    /// returns: Option<Ordering>
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cmp::Ordering;
    ///
    ///  let comp = BytewiseComparatorImpl::default();
    ///  optionVal = comp.compare(&Slice::from("a"), &Slice::from("ab"));
    ///  assert_eq!(optionVal.unwrap(), Ordering::Less);
    ///
    ///  let comp = BytewiseComparatorImpl::default();
    ///  let optionVal = comp.compare(&Slice::from("b"), &Slice::from("abcd"));
    ///  assert_eq!(optionVal.unwrap(), Ordering::Greater);
    /// ```
    fn compare(&self, a: &Slice, b: &Slice) -> Option<Ordering>;

    /// 返回comparator的名字
    fn get_name() -> String;

    /// 函数：用于减少像index blocks这样的内部数据结构占用的空间, 其中的*start和*key参数都是IN OUT的。
    ///
    /// 这个函数的作用就是：
    /// 如果*start < limit，就在[startlimit,)中找到一个短字符串，并赋给*start返回. 如“helloworld”和”hellozoomer”之间最短的key可以是”hellox”。
    /// 简单的comparator实现可能不改变*start(start==limit)，这也是正确的
    ///
    /// # Arguments
    ///
    /// * `start`:
    /// * `limit`:
    ///
    /// returns: String
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn find_shortest_separator(&self, start: &String, limit:&Slice) -> String;

    /// 减少像index blocks这样的内部数据结构占用的空间
    /// 将 key变成一个比原*key大的短字符串，并赋值给 key返回。
    fn find_short_successor(&self, key: &String) -> String;

}