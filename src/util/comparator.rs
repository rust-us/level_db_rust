
use std::cmp::Ordering;
use crate::traits::comparator_trait::{ComparatorTrait};
use crate::util::slice::Slice;

pub struct BytewiseComparatorImpl {}

impl ComparatorTrait for BytewiseComparatorImpl {
    fn compare(&self, a: &Slice, b: &Slice) -> Option<Ordering> {
        todo!()
    }

    fn get_name() -> String {
        String::from("leveldb.BytewiseComparator")
    }

    fn find_shortest_separator(&self, start: &String, limit: &Slice) -> String {
        todo!()
    }

    fn find_short_successor(&self, key: &String) -> String {
        todo!()
    }
}

pub struct InternalKeyComparator {
    // fn user_comparator(&self) -> Box<Comparator> {
    //     todo!()
    // }

    // fn Compare(InternalKey, InternalKey)
}

/// InternalKeyComparator 比较器: 用来比较内部键（Internal Key）。
/// 内部键值是为了方便处理，将原普通键、序列号和值类型组成的新键。
impl ComparatorTrait for InternalKeyComparator {
    // todo  InternalKeyComparator 的构造方法

    fn compare(&self, a: &Slice, b: &Slice) -> Option<Ordering> {
        a.partial_cmp(b)
    }

    fn get_name() -> String {
        String::from("leveldb.InternalKeyComparator")
    }

    fn find_shortest_separator(&self, start: &String, limit: &Slice) -> String {
        todo!()
    }

    fn find_short_successor(&self, key: &String) -> String {
        todo!()
    }
}

pub struct ReverseKeyComparator {

}

impl ComparatorTrait for ReverseKeyComparator {
    fn compare(&self, a: &Slice, b: &Slice) -> Option<Ordering> {
        todo!()
    }

    fn get_name() -> String {
        String::from("leveldb.ReverseBytewiseComparator")
    }

    fn find_shortest_separator(&self, start: &String, limit: &Slice) -> String {
        todo!()
    }

    fn find_short_successor(&self, key: &String) -> String {
        todo!()
    }
}