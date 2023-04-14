use std::fs::read;
use std::iter::Map;
use crate::db::db_format::InternalKey;
use crate::db::file_meta_data::FileMetaData;
use crate::db::version_edit;
use crate::util::coding::Encoder;
use crate::util::slice::Slice;
use crate::util::Result;
use crate::util::status::{LevelError, Status};

pub struct VersionEdit {
    comparator_: String,
    log_number_: u64,
    prev_log_number_: u64,
    next_file_number_: u64,
    last_sequence_: u64,
    has_comparator_: bool,
    has_log_number_: bool,
    has_prev_log_number_: bool,
    has_next_file_number_: bool,
    has_last_sequence_: bool,

    compact_pointers_: Vec<(u32, InternalKey)>,
    // left: level;  right: file number
    deleted_files_: Vec<(u32, u64)>,
    // left: level;  right: FileMetaData
    new_files_: Vec<(u32, FileMetaData)>,
}

pub enum Tag {
    k_comparator = 1,
    kLogNumber = 2,
    kNextFileNumber = 3,
    kLastSequence = 4,
    kCompactPointer = 5,
    kDeletedFile = 6,
    kNewFile = 7,
    // 8 was used for large value refs
    kPrevLogNumber = 9,
}

impl Tag {
    /// 得到枚举 Tag 的固定值
    /// Tag numbers for serialized VersionEdit.  These numbers are written to disk and should not be changed.
    pub fn get_value(&self) -> i32 {
        let val = match self {
            Tag::k_comparator => 1,
            Tag::kLogNumber => 2,
            Tag::kNextFileNumber => 3,
            Tag::kLastSequence => 4,
            Tag::kCompactPointer => 5,
            Tag::kDeletedFile => 6,
            Tag::kNewFile => 7,
            Tag::kPrevLogNumber => 9,
            _ => 0
        };

        val
    }

    /// 根据值计算枚举 Tag
    pub fn from_value(val: u32) -> Option<Tag> {
        let val = match val {
            1 => Some(Tag::k_comparator),
            2 => Some(Tag::kLogNumber),
            3 => Some(Tag::kNextFileNumber),
            4 => Some(Tag::kLastSequence),
            5 => Some(Tag::kCompactPointer),
            6 => Some(Tag::kDeletedFile),
            7 => Some(Tag::kNewFile),
            9 => Some(Tag::kPrevLogNumber),
            _ => None
        };

        val
    }
}

impl VersionEdit {
    #[inline]
    pub fn new() -> Self {
        Self {
            comparator_: String::new(),
            log_number_: 0,
            prev_log_number_: 0,
            next_file_number_: 0,
            last_sequence_: 0,
            has_comparator_: false,
            has_log_number_: false,
            has_prev_log_number_: false,
            has_next_file_number_: false,
            has_last_sequence_: false,
            compact_pointers_: vec![],
            deleted_files_: vec![],
            new_files_: vec![],
        }
    }

    #[inline]
    pub fn new_with_log_number(log_number: u64) -> Self {
        let mut version_edit = VersionEdit::new();
        version_edit.set_log_number(log_number);

        version_edit
    }

    #[inline]
    pub fn new_with_prev_log_number(prev_log_number: u64) -> Self {
        let mut version_edit = VersionEdit::new();
        version_edit.set_prev_log_number(prev_log_number);

        version_edit
    }

    /// 清空
    pub fn clear(&mut self) {
        self.comparator_.clear();
        self.log_number_ = 0;
        self.prev_log_number_ = 0;
        self.last_sequence_ = 0;
        self.next_file_number_ = 0;
        self.has_comparator_ = false;
        self.has_log_number_ = false;
        self.has_prev_log_number_ = false;
        self.has_next_file_number_ = false;
        self.has_last_sequence_ = false;
        self.deleted_files_.clear();
        self.new_files_.clear();

        // compact_pointers_ don't clear
    }

    pub fn set_comparator_name(&mut self, name: Slice) {
        self.has_comparator_ = true;
        self.comparator_ = name.into();
    }

    pub fn set_log_number(&mut self, num: u64) {
        self.has_log_number_ = true;
        self.log_number_ = num;
    }

    pub fn set_prev_log_number(&mut self, num: u64) {
        self.has_prev_log_number_ = true;
        self.prev_log_number_ = num;
    }

    pub fn set_next_file(&mut self, num: u64) {
        self.has_next_file_number_ = true;
        self.next_file_number_ = num;
    }

    pub fn set_last_sequence(&mut self, seq: u64) {
        self.has_last_sequence_ = true;
        self.last_sequence_ = seq;
    }

    pub fn set_compact_pointer(&mut self, level: u32, key: InternalKey) {
        self.compact_pointers_.push((level, key))
    }

    /// Add the specified file at the specified number.
    /// REQUIRES: This version has not been saved (see VersionSet::SaveTo)
    /// REQUIRES: "smallest" and "largest" are smallest and largest keys in file
    ///
    /// # Arguments
    ///
    /// * `level`:
    /// * `file`:
    /// * `file_size`:
    /// * `smallest`: 移交所有权
    /// * `largest`: 移交所有权
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn add_file(&mut self, level: u32, file: u64, file_size: u64, smallest: InternalKey, largest: InternalKey) {
        let file_meta_data = FileMetaData::new_with_number_file_size_internal_key(file, file_size, smallest, largest);

        self.new_files_.push((level, file_meta_data));
    }

    pub fn delete_file(&mut self, level: u32, file: u64) {
        self.deleted_files_.push((level, file));
    }

    /// 将 VersionEdit 对象编码至 target 中
    ///
    /// # Arguments
    ///
    /// * `target`:
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn encode_to(&self, target: &mut Vec<u8>) -> Result<()> {
        let mut encoder = Encoder::with_vec(target);
        if self.has_comparator_ {
            encoder.put_varint32(Tag::k_comparator.get_value() as u32)?;
            encoder.put_varint32(self.comparator_.len() as u32)?;
            // fixme 需要正确使用 put_length_prefixed_slice  将slice长度及slice内容编码到target
            // encoder.put_length_prefixed_slice(self.comparator_.len())?;
        }
        if self.has_log_number_ {
            encoder.put_varint32(Tag::kLogNumber.get_value() as u32)?;
            encoder.put_varint64(self.log_number_)?;
        }

        if self.has_prev_log_number_ {
            encoder.put_varint32(Tag::kPrevLogNumber.get_value() as u32)?;
            encoder.put_varint64(self.prev_log_number_)?;
        }

        if self.has_next_file_number_ {
            encoder.put_varint32(Tag::kNextFileNumber.get_value() as u32)?;
            encoder.put_varint64(self.next_file_number_)?;
        }

        if self.has_last_sequence_ {
            encoder.put_varint32(Tag::kLastSequence.get_value() as u32)?;
            encoder.put_varint64(self.last_sequence_)?;
        }

        for i in 0..self.compact_pointers_.len() {
            encoder.put_varint32(Tag::kCompactPointer.get_value() as u32)?;
            encoder.put_varint32(self.compact_pointers_[i].0)?;
            // fixme 需要正确使用put_length_prefixed_slice
            // encoder.put_length_prefixed_slice( self.compact_pointers_[i].1.encode_len())?;
        }

        for i in 0..self.deleted_files_.len() {
            encoder.put_varint32(Tag::kDeletedFile.get_value() as u32)?;
            encoder.put_varint32(self.deleted_files_[i].0)?;
            encoder.put_varint64(self.deleted_files_[i].1)?;
        }

        for i in 0..self.new_files_.len() {
            let f: &FileMetaData = &self.new_files_[i].1;
            encoder.put_varint32(Tag::kNewFile.get_value() as u32)?;
            // level
            encoder.put_varint32(self.new_files_[i].0)?;
            encoder.put_varint64(f.get_number())?;
            encoder.put_varint64(f.get_file_size())?;
            // fixme 需要正确使用put_length_prefixed_slice
            // encoder.put_length_prefixed_slice( f.get_smallest().encode_len())?;
            // encoder.put_length_prefixed_slice( f.get_largest().encode_len())?;
        }
        Ok(())
    }

    /// 将 source 中的数据解码至 self VersionEdit 中
    ///
    /// # Arguments
    ///
    /// * `source`:
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn decode_from(&mut self, source: &Slice) -> Status {
        self.clear();

        let version_edit = VersionEdit::new();

        let msg: Option<Tag> = Option::None;

        // todo Coding::get_varint32 存在问题。开发暂停
        // while msg.is_none() && Coding::get_varint32(source) != 0_u32 {
        //     let tag_value = Coding::get_varint32(source);
        //     let tag = Tag::from_value(tag_value);
        //
        //     if tag.is_none() {
        //         return LevelError::corruption_string("VersionEdit", "unknown tag");
        //     }
        //
        // }
        todo!()
    }

    /// VersionEdit 输出调试信息
    pub fn debug_string(&self) -> Slice {
        let debug_str = String::from("VersionEdit {");

        let mut has_comparator_str = String::default();
        if (self.has_comparator_) {
            has_comparator_str.push_str(format!("\n Comparator: {}", self.comparator_.as_str()).as_str());
        }

        let mut has_log_number__str = String::default();
        // if(self.has_log_number_){
        // todo
        //     // let append_log_number = logging.AppendNumberTo(&r, self.log_number_);
        //     let append_log_number = self.log_number_ + "".as_ref();
        //     has_log_number__str.push_str(format!("\n LogNumber: {}", append_log_number).as_str());
        // }

        let rs = format!("{}{}{}", debug_str, has_log_number__str, "\n}\n");

        Slice::from(rs)
    }
}

/// 静态方法
impl<'a> VersionEdit {
    pub fn get_internal_key(input: Slice) -> Result<InternalKey> {
        let key = InternalKey::default();

        todo!()

        // Slice str;
        // if (GetLengthPrefixedSlice(input, &str)) {
        //     dst->DecodeFrom(str);
        //     return true;
        // } else {
        //     return false;
        // }
    }

    /// 从 Slice 中解出 level 值
    ///
    /// # Arguments
    ///
    /// * `input`:
    ///
    /// returns: bool
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn get_level(input: Slice) -> Result<u32> {
        todo!()
    }
}