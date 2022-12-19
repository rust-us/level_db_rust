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
    /// 作用是，
    /// 如果start < limit,就把start修改为 start和limit的共同前缀, 后面多一个字符加1
    ///
    /// 例如：
    /// start:    helloWorld
    /// limit:     helloZookeeper
    /// 由于  start < limit, 所以调用 FindShortSuccessor(start, limit)之后，
    /// start变成： helloX (保留前缀，第一个不相同的字符+1)
    ///
    /// # Arguments
    ///
    /// * `start`:  String
    /// * `limit`:  Slice
    ///
    /// returns: String
    ///
    /// # Examples
    ///
    /// ```
    ///         let comp = BytewiseComparatorImpl::default();
    ///         let find_shortest_separator_val = comp.find_shortest_separator(
    ///             &String::from("abcdefghijklimA"),
    ///             &Slice::from("abcdefghijklimNy"));
    ///         /// A < N
    ///         assert_eq!(find_shortest_separator_val, "abcdefghijklimB");
    ///
    ///         let comp = BytewiseComparatorImpl::default();
    ///         let find_shortest_separator_val = comp.find_shortest_separator(
    ///             &String::from("abcdefghijklima"),
    ///             &Slice::from("abcdefghijklimNy"));
    ///         /// a > N
    ///         assert_eq!(find_shortest_separator_val, "abcdefghijklima");
    /// ```
    fn find_shortest_separator(&self, start: &String, limit:&Slice) -> String;

    /// 用于找到比key大的最短字符串，如传入“helloworld”，返回的key可能是“i”
    ///
    /// 找一个 >= key的短字符串, key变成一个比原key大的短字符串，并返回。
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