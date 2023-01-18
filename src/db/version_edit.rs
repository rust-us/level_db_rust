use std::iter::Map;
use crate::db::db_format::InternalKey;
use crate::db::file_meta_data::FileMetaData;
use crate::util::slice::Slice;
use crate::util::Result;

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
    deleted_files_: Vec<(u32, u64)>,
    new_files_: Vec<(u32, FileMetaData)>,
}

enum Tag {
    // kComparator           = 1,
    // kLogNumber            = 2,
    // kNextFileNumber       = 3,
    // kLastSequence         = 4,
    // kCompactPointer       = 5,
    // kDeletedFile          = 6,
    // kNewFile              = 7,
    // // 8 was used for large value refs
    // kPrevLogNumber        = 9

    kComparator,
    kLogNumber,
    kNextFileNumber,
    kLastSequence,
    kCompactPointer,
    kDeletedFile,
    kNewFile,
    // 8 was used for large value refs
    kPrevLogNumber
}

impl VersionEdit {
    /// 清空
    fn clear(&mut self) {
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

    fn set_comparator_name(&mut self, name: Slice){
        self.has_comparator_ = true;
        self.comparator_ = name.into();
    }

    fn set_prev_log_number(&mut self, num: u64){
        self.has_prev_log_number_ = true;
        self.prev_log_number_ = num;
    }

    fn set_next_file(&mut self, num: u64){
        self.has_next_file_number_ = true;
        self.next_file_number_ = num;
    }

    fn set_last_sequence(&mut self, seq: u64){
        self.has_last_sequence_ = true;
        self.last_sequence_ = seq;
    }

    fn set_compact_pointer(&mut self, level: u32, key: InternalKey) {
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
    fn add_file(&mut self, level: u32, file: u64, file_size: u64, smallest: InternalKey, largest: InternalKey) {
        let file_meta_data = FileMetaData::create_number_file_size_internal_key(file, file_size, smallest, largest);

        self.new_files_.push((level, file_meta_data));
    }

    fn delete_file(&mut self, level: u32, file: u64) {
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
    fn encode_to(&self, target: Vec<u8>) {
        todo!()
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
    fn decode_from(&mut self, source: Slice) {
        self.clear();

        todo!()
    }

    /// VersionEdit 输出调试信息
    fn debug_string(&self) -> Slice {
        todo!()
    }
}

impl<'a> VersionEdit {
    pub fn get_internal_key(inout: Slice) -> Result<InternalKey> {
        todo!()
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