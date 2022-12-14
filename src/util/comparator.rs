
use std::cmp::{min, Ordering};
use crate::traits::comparator_trait::{ComparatorTrait};
use crate::util::slice::Slice;

///  BytewiseComparatorImpl是按字典逐字节序进行比较
///  也就是说 i>helloworld，因为先比较i和h，i>h，比较直接结束
pub struct BytewiseComparatorImpl {}

impl ComparatorTrait for BytewiseComparatorImpl {

    fn compare(&self, a: &Slice, b: &Slice) -> Option<Ordering> {
        a.partial_cmp(b)
    }

    fn get_name() -> String {
        String::from("leveldb.BytewiseComparator")
    }

    fn find_shortest_separator(&self, start: &String, limit: &Slice) -> String {
        // 首先计算共同前缀字符串的长度
        let min_lengrh: usize = min(start.len(), limit.len());

        // let mut diff_index: usize = 0;
        // let start_char_vec: Vec<char> = start.chars().collect::<Vec<_>>();
        // let limit_char_vec: Vec<char> = limit.chars().collect::<Vec<_>>();
        // while (
        //     (diff_index < min_lengt) &&
        //         (start_char_vec[diff_index] == limit_char_vec[diff_index])
        // ){
        //     diff_index = diff_index + 1;
        // }

        String::from("")
    }

    fn find_short_successor(&self, key: &String) -> String {
        todo!()
    }
}

impl Default for BytewiseComparatorImpl {
    fn default() -> Self {
        Self{}
    }
}

// /// InternalKeyComparator
// pub struct InternalKeyComparator {
//     // fn user_comparator(&self) -> Box<Comparator> {
//     //     todo!()
//     // }
//
//     // fn Compare(InternalKey, InternalKey)
// }
//
// /// InternalKeyComparator 比较器: 用来比较内部键（Internal Key）。
// /// 内部键值是为了方便处理，将原普通键、序列号和值类型组成的新键。
// impl ComparatorTrait for InternalKeyComparator {
//     // todo  InternalKeyComparator 的构造方法
//
//     fn compare(&self, a: &Slice, b: &Slice) -> Option<Ordering> {
//         todo!()
//     }
//
//     fn get_name() -> String {
//         String::from("leveldb.InternalKeyComparator")
//     }
//
//     fn find_shortest_separator(&self, start: &String, limit: &Slice) -> String {
//         todo!()
//     }
//
//     fn find_short_successor(&self, key: &String) -> String {
//         todo!()
//     }
// }
