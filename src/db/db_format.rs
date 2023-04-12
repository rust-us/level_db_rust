use std::cmp::Ordering;
use std::io::Write;
use std::sync::Arc;
use crate::db::db_format::ValueType::{KTypeDeletion, KTypeValue};
use crate::db::file_meta_data::FileMetaData;
use crate::traits::coding_trait::CodingTrait;
use crate::traits::comparator_trait::Comparator;
use crate::util::coding::Coding;
use crate::util::slice::Slice;
use crate::util::unsafe_slice::UnsafeSlice;

pub enum ValueType {
    /// 0x0
    KTypeDeletion,

    /// 0x1
    KTypeValue,
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
    user_comparator_: Arc<dyn Comparator>
}

/// 查找键
// todo   add clone trait
pub struct LookupKey {
    /// |klength(varint32)|user key(string)|sequence number(7 bytes)|value type(1 byte)|
    data: Slice,
    /// start index at user key
    user_key_start: usize,
}

impl ValueType {
    pub fn get_value(&self) -> usize {
        let le = match self {
            KTypeDeletion => 0,
            KTypeValue => 1
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
            0 => Ok(KTypeDeletion),
            1 => Ok(KTypeValue),
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
            value_type: KTypeDeletion,
        }
    }
}

impl ParsedInternalKey {

    pub fn debug_string(&self) -> Slice {
        Slice::default()
    }

    /// Return the length of the encoding of "key".
    pub fn internal_key_encoding_length(&self, key: ParsedInternalKey) -> usize {
        key.user_key.size() + 8
    }

    // 将 self 的数据追加到 result 中
    pub fn append_internal_key(&self, result: Slice) {
        self.user_key.merge(result, None)
    }

    pub fn new(user_key: Slice, sequence: u64, value_type: ValueType) -> Self {
        Self {
            user_key,
            sequence,
            value_type,
        }
    }

    /// Attempt to parse an internal key from "internal_key".  On success,
    /// stores the parsed data in "*result", and returns true.
    /// On error, returns false, leaves "*result" in an undefined state.
    pub fn parse_internal_key(internal_key : Slice, target: ParsedInternalKey) -> bool {
        // line 173
        todo!()
    }

    /// Returns the user key portion of an internal key.
    pub fn extract_user_key(internal_key : Slice) -> Slice {
        let len : usize = internal_key.size();
        if len>= 8 {
            return &internal_key[len-8..len];
        }
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
    pub fn new(user_key: Slice, sequence: u64, value_type: ValueType) -> Self {
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

    pub fn decode_from(&self, input: &UnsafeSlice) {
        // xiaohui
        self.rep_.merge(input, None);
        return !self.rep_.empty()
    }

    /// 输出 InternalKey 调试信息
    pub fn debug_string(&self) -> Slice {
        // line 164
        return self::Default();
    }

    pub fn encode(self) -> Slice {
        self.rep_
    }

    /// 取得  Slice的长度
    pub fn encode_len(&self) -> usize {
        self.rep_.size()
    }

    pub fn user_key(self) -> Slice {
        ParsedInternalKey::extract_user_key(self.rep_)
    }

    pub fn set_from(self, p: ParsedInternalKey) {
        // self.rep_.clear();
        p.append_internal_key(self.rep_);
    }

    pub fn clear(&self) {
        self.rep_.remove_prefix(self.rep_.size());
    }
}

impl Default for InternalKeyComparator {
    fn default() -> Self {
        todo!()
    }
}

impl InternalKeyComparator {
    pub fn create(_cmp: Box<dyn Comparator>) -> Box<Self> {
        todo!()
    }

    pub fn user_comparator(&self) -> Box<dyn Comparator> {
        todo!()
    }

    pub fn compare_internal_key(&self, key1: &InternalKey, key2: &InternalKey) -> u32 {
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

    fn compare(&self, _a: &[u8], _b: &[u8]) -> Option<Ordering> {
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
    pub fn new(user_key: Slice, sequence: usize) -> Self {
        let user_key_size = user_key.size();
        let need = user_key_size + 13; // A conservative estimate
        let mut data = Vec::with_capacity(need);
        let buf = data.as_mut_slice();
        let klength = Coding::varint_length(user_key_size + 8);
        let mut offset = 0;
        // write key size
        offset = Coding::encode_varint32(klength as u32, buf, offset);
        // write key slice
        offset += (&mut buf[offset..]).write(user_key.as_ref()).expect("write user_key");
        // write sequence number and value type
        Coding::encode_fixed64(
            pack_sequence_and_type(sequence, ValueType::KTypeValue),
                buf, offset);

        LookupKey {
            data: Slice::from_vec(data),
            user_key_start: klength
        }
    }

    /// Return a key suitable for lookup in a MemTable.
    pub fn mem_table_key(&self) -> Slice {
        self.data.clone()
    }

    /// Return an internal key (suitable for passing to an internal iterator)
    pub fn internal_key(&self) -> Slice {
        // line 204
        let buf = self.data.as_ref();
        let internal_key_buf = &buf[self.user_key_start..];
        Slice::from_buf(internal_key_buf.clone())
    }

    /// Return the user key
    pub fn user_key(&self) -> Slice {
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

const K_MAX_SEQUENCE_NUMBER: usize = (1 << 56) - 1;

#[inline]
pub fn pack_sequence_and_type(seq_no: usize, v_type: ValueType) -> u64 {
    debug_assert!(seq_no <= K_MAX_SEQUENCE_NUMBER);
    debug_assert!(v_type.get_value() <= 1);
    ((seq_no << 8) | v_type.get_value()) as u64
}

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
    pub const K_VALUE_TYPE_FOR_SEEK: ValueType = ValueType::KTypeValue;
}