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

    /// 函数：找到start、limit之间最短的字符串，如“helloworld”和”hellozoomer”之间最短的key可以是”hellox”
    ///
    /// 作用是：
    /// 作用是，如果start < limit,就把start修改为*start和limit的共同前缀后面多一个字符加1
    /// 例如：
    /// start:    helloWorld
    /// limit:     helloZookeeper
    /// 由于 *start < limit, 所以调用 FindShortSuccessor(start, limit)之后，start变成： helloX (保留前缀，第一个不相同的字符+1)
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

    /// 找一个 >= key的短字符串, key变成一个比原*key大的短字符串，并返回。
    /// 简单的comparator实现可能不改变 key，这也是正确的
    ///
    /// # Arguments
    ///
    /// * `key`:
    ///
    /// returns: String
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn find_short_successor(&self, key: &String) -> String;

}