use std::ops::Deref;
use crate::db::db_format::ValueType::{K_TYPE_DELETION, K_TYPE_VALUE};
use crate::util::slice::Slice;

/// Maximum encoding length of a BlockHandle
pub const k_num_levels: usize = 7;

// Level-0 compaction is started when we hit this many files.
pub const kl0_compaction_trigger: usize = 4;

// Soft limit on number of level-0 files.  We slow down writes at this point.
pub const kl0_slowdown_writes_trigger: usize = 8;

// Maximum number of level-0 files.  We stop writes at this point.
pub const kL0_stop_writes_trigger: usize = 12;

// Maximum level to which a new compacted memtable is pushed if it
// does not create overlap.  We try to push to level 2 to avoid the
// relatively expensive level 0=>1 compactions and to avoid some
// expensive manifest file operations.  We do not push all the way to
// the largest level since that can generate a lot of wasted disk
// space if the same key space is being repeatedly overwritten.
pub const k_max_mem_compact_level: usize = 2;

// Approximate gap in bytes between samples of data read during iteration.
pub const k_read_bytes_period: usize = 1048576;

// kValueTypeForSeek defines the ValueType that should be passed when
// constructing a ParsedInternalKey object for seeking to a particular
// sequence number (since we sort sequence numbers in decreasing order
// and the value type is embedded as the low 8 bits in the sequence
// number in internal keys, we need to use the highest-numbered
// ValueType, not the lowest).
pub const k_value_type_for_seek: ValueType = ValueType::K_TYPE_VALUE;

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

pub struct InternalKey {
    rep_: Slice
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
    fn new(u: Slice, seq: u64, t: ValueType) -> Self {
        Self {
            user_key: u,
            sequence: seq,
            value_type: t,
        }
    }

    fn debug_string(&self) -> Slice {
        Slice::default()
    }

    /// Return the length of the encoding of "key".
    fn internal_key_encoding_length(&self, key: ParsedInternalKey) -> usize {
        key.user_key.size() + 8
    }

    fn append_internal_key(&self, key : ParsedInternalKey) -> Slice {
        todo!()
    }

    /// Attempt to parse an internal key from "internal_key".  On success,
    /// stores the parsed data in "*result", and returns true.
    /// On error, returns false, leaves "*result" in an undefined state.
    fn parse_internal_key(&self, internal_key : Slice, target: ParsedInternalKey) -> bool {
        todo!()
    }

    /// Returns the user key portion of an internal key.
    fn extract_user_key(&self, internal_key : Slice) -> Slice {
        todo!()
    }
}

impl InternalKey {
    // line139
}