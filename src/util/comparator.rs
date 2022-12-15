
use std::cmp::{min, Ordering};
use crate::traits::comparator_trait::{ComparatorTrait};
use crate::util::slice::Slice;

pub struct BytewiseComparatorImpl {}

///
/// compare:
///   按字典逐字节序进行比较
///   也就是说 i>helloworld，因为先比较i和h，i>h，比较直接结束
impl Default for BytewiseComparatorImpl {
    fn default() -> Self {
        Self{}
    }
}

impl ComparatorTrait for BytewiseComparatorImpl {

    fn compare(&self, a: &Slice, b: &Slice) -> Option<Ordering> {
        a.partial_cmp(b)
    }

    fn get_name() -> String {
        String::from("leveldb.BytewiseComparator")
    }

    fn find_shortest_separator(&self, start: &String, limit: &Slice) -> String {
        // 首先计算共同前缀字符串的长度
        let min_length: usize = min(start.len(), limit.len());

        let mut diff_index: usize = 0;
        let mut start_char_vec: Vec<u8>  = start.as_bytes().to_vec();
        let limit_char_vec: &Vec<u8> = &limit.to_vec();
        // or use
        // let start_char_vec: Vec<char> = start.chars().collect::<Vec<_>>();
        // let limit_char_vec: Vec<char> = limit.to_vec().iter().map(|b| *b as char).collect::<Vec<_>>();

        while diff_index < min_length &&
            start_char_vec[diff_index] == limit_char_vec[diff_index]
        {
            // Increment counter
            diff_index += 1;
        }

        // 如果一个字符串是另个一字符串的前缀，无需做截短操作，否则进入 else。
        if diff_index >= min_length {
            // 说明 start是limit的前缀，或者反之，此时不作修改，直接返回
        } else{
            // 尝试执行字符start[diff_index]++， 设置start长度为diff_index+1，并返回
            // ++条件：字符 < oxff 并且字符+1 < limit上该index的字符
            let diff_byte = start_char_vec[diff_index];
            // let diff_char = diff_byte as char;

            if diff_byte < 0xff &&
                // 且 start 中的差异字符的next 小于 limit中的diff_index的字符，
                // 则将 start 差异字符位置+1的元素变更为 差异字符的next
                (diff_byte + 1) < limit_char_vec[diff_index] {
                start_char_vec[diff_index] = diff_byte + 1;
            }
        }

        let shortest_separator: &[u8] = &start_char_vec[0..diff_index+1];

        let shortest_separator_val:  String= Slice::from_buf(shortest_separator).into();
        shortest_separator_val
    }

    fn find_short_successor(&self, key: &String) -> String {
        String::from("a")
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
