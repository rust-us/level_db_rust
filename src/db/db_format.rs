use std::cmp::Ordering;
use std::ops::Deref;
use crate::db::db_format::ValueType::{K_TYPE_DELETION, K_TYPE_VALUE};
use crate::traits::comparator_trait::Comparator;
use crate::util::slice::Slice;

pub enum ValueType {
    /// 0x0
    K_TYPE_DELETION,

    /// 0x1
    K_TYPE_VALUE,
}

pub struct ParsedInternalKey {
    user_key: Slice,
    sequence: u64,
    value_type: ValueType
}

#[derive(Debug)]
pub struct InternalKey {
    rep_: Slice
}

/// InternalKeyComparator
pub struct InternalKeyComparator {
    user_comparator_: dyn Comparator
}

/// 查找键
// todo   add clone trait
pub struct LookupKey {
    // We construct a char array of the form:
    //    klength  varint32               <-- start_
    //    userkey  char[klength]          <-- kstart_
    //    tag      uint64
    //                                    <-- end_
    // The array is a suitable MemTable key.
    // The suffix starting with "userkey" can be used as an InternalKey.

    start_: Slice,
    kstart_: Slice,
    end_: Slice,

    // Avoid allocation for short keys
    space_: [u8; 200],
}

impl ValueType {
    pub fn get_value(&self) -> i32 {
        let le = match self {
            K_TYPE_DELETION => 0,
            K_TYPE_VALUE => 1
        };

        le
    }
}

impl TryFrom<i32> for ValueType {
    type Error = String;

    /// i32 转 ValueType
    ///
    /// # Arguments
    ///
    /// * `value`:  值
    ///
    /// returns: Result<ValueType, <ValueType as TryFrom<i8>>::Error>
    ///
    /// # Examples
    ///
    /// ```
    ///        let rs: ValueType = ValueType::try_from(1)?;
    ///         assert!(&rs == K_TYPE_DELETION);
    /// ```
    #[inline]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(K_TYPE_DELETION),
            1 => Ok(K_TYPE_VALUE),
            // all other numbers
            _ => Err(String::from(format!("Unknown code: {}", value)))
        }
    }
}

impl Default for ParsedInternalKey {
    #[inline]
    fn default() -> Self {
        ParsedInternalKey {
            user_key: Default::default(),
            sequence: 0,
            value_type: K_TYPE_DELETION,
        }
    }
}

impl ParsedInternalKey {

    fn debug_string(&self) -> Slice {
        Slice::default()
    }

    /// Return the length of the encoding of "key".
    fn internal_key_encoding_length(&self, key: ParsedInternalKey) -> usize {
        key.user_key.size() + 8
    }

    // 将 self 的数据追加到 result 中
    fn append_internal_key(&self, result: Slice) {
        todo!()
    }

    fn new(user_key: Slice, sequence: u64, value_type: ValueType) -> Self {
        Self {
            user_key,
            sequence,
            value_type,
        }
    }

    /// Attempt to parse an internal key from "internal_key".  On success,
    /// stores the parsed data in "*result", and returns true.
    /// On error, returns false, leaves "*result" in an undefined state.
    fn parse_internal_key(internal_key : Slice, target: ParsedInternalKey) -> bool {
        // line 173
        todo!()
    }

    /// Returns the user key portion of an internal key.
    fn extract_user_key(internal_key : Slice) -> Slice {
        todo!()
    }
}

impl Default for InternalKey {
    #[inline]
    fn default() -> Self {
        Self {
            rep_: Slice::default()
        }
    }
}

impl PartialEq for InternalKey {
    /// 判断两个 InternalKey 是否相同
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.rep_.eq(&other.rep_)
    }
}

impl InternalKey {
    fn new(user_key: Slice, sequence: u64, value_type: ValueType) -> Self {
        // line 145
        let result: Slice = Slice::default();
        ParsedInternalKey::new(user_key, sequence, value_type)
            .append_internal_key(result);

        Self{
            // rep_: result
            // todo result值如何赋值
            rep_: Slice::default()
        }
    }

    ///
    /// xxx
    ///
    /// # Arguments
    /// * `input`:
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn decode_from(&self, input: Slice) {
        todo!()

        // wangbo
        // self.rep_.assign(input.borrow_data().bytes());
    }

    /// 输出 InternalKey 调试信息
    fn debug_string(&self) -> Slice {
        // line 164
        todo!()
    }

    fn encode(self) -> Slice {
        self.rep_
    }

    fn user_key(self) -> Slice {
        ParsedInternalKey::extract_user_key(self.rep_)
    }

    fn set_from(self, p: ParsedInternalKey) {
        // self.rep_.clear();
        p.append_internal_key(self.rep_);
    }

    fn clear(self) {
        // self.rep_.clear();
    }
}

impl InternalKeyComparator {
    fn new(c: Box<dyn Comparator>) -> Box<Self> {
        todo!()
    }

    fn user_comparator(&self) -> Box<dyn Comparator> {
        todo!()
    }

    fn compare(&self, a: InternalKey, b: InternalKey) -> u32 {
        // line 122, 167
        todo!()
    }
}

/// InternalKeyComparator 比较器: 用来比较内部键（Internal Key）。
/// 内部键值是为了方便处理，将原普通键、序列号和值类型组成的新键。
impl Comparator for InternalKeyComparator {
    // fn new(c: Box<ComparatorTrait>) -> InternalKeyComparator {
    //     todo!()
    // }

    fn compare(&self, _a: &Slice, _b: &Slice) -> Option<Ordering> {
        todo!()
    }

    fn get_name(&self) -> String {
        String::from("leveldb.InternalKeyComparator")
    }

    fn find_shortest_separator(&self, _start: &String, _limit: &Slice) -> String {
        todo!()
    }

    fn find_short_successor(&self, _key: &String) -> String {
        todo!()
    }
}

impl LookupKey {
    /// Initialize *this for looking up user_key at a snapshot with
    /// the specified sequence number.
    fn new(user_key: Slice, sequence: u64) -> Self {
        // todo
        todo!()
    }

    /// Return a key suitable for lookup in a MemTable.
    fn mem_table_key(&self) -> Slice {
        todo!()
    }

    /// Return an internal key (suitable for passing to an internal iterator)
    fn internal_key(&self) -> Slice {
        // line 204
        todo!()
    }

    /// Return the user key
    fn user_key(&self) -> Slice {
        // line 207
        todo!()
    }
}

// like  ~LookupKey()
// impl Drop for LookupKey {
//     fn drop(&mut self) {
//         todo!()
//     }
// }
//
// impl Default for LookupKey {
//     #[inline]
//     fn default() -> Self {
//         Self {
//             start_: Default::default(),
//             kstart_: Default::default(),
//             end_: Default::default(),
//             space_: [u8; 200],
//         }
//     }
// }


pub struct Config {}
impl Config {
    /// Maximum encoding length of a BlockHandle
    pub const K_NUM_LEVELS: usize = 7;

    // Level-0 compaction is started when we hit this many files.
    pub const KL0_COMPACTION_TRIGGER: usize = 4;

    // Soft limit on number of level-0 files.  We slow down writes at this point.
    pub const KL0_SLOWDOWN_WRITES_TRIGGER: usize = 8;

    // Maximum number of level-0 files.  We stop writes at this point.
    pub const K_L0_STOP_WRITES_TRIGGER: usize = 12;

    // Maximum level to which a new compacted memtable is pushed if it
// does not create overlap.  We try to push to level 2 to avoid the
// relatively expensive level 0=>1 compactions and to avoid some
// expensive manifest file operations.  We do not push all the way to
// the largest level since that can generate a lot of wasted disk
// space if the same key space is being repeatedly overwritten.
    pub const K_MAX_MEM_COMPACT_LEVEL: usize = 2;

    // Approximate gap in bytes between samples of data read during iteration.
    pub const K_READ_BYTES_PERIOD: usize = 1048576;

    // kValueTypeForSeek defines the ValueType that should be passed when
// constructing a ParsedInternalKey object for seeking to a particular
// sequence number (since we sort sequence numbers in decreasing order
// and the value type is embedded as the low 8 bits in the sequence
// number in internal keys, we need to use the highest-numbered
// ValueType, not the lowest).
    pub const K_VALUE_TYPE_FOR_SEEK: ValueType = ValueType::K_TYPE_VALUE;
}