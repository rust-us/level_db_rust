use std::iter::Map;
use crate::db::db_format::InternalKey;
use crate::db::file_meta_data::FileMetaData;
use crate::util::slice::Slice;

pub struct VersionEdit {
    comparator_: Slice,
    log_number_: u64,
    prev_log_number_: u64,
    next_file_number_: u64,
    last_sequence_: u64,
    has_comparator_: bool,
    has_log_number_: bool,
    has_prev_log_number_: bool,
    has_next_file_number_: bool,
    has_last_sequence_: bool,

    compact_pointers_: Vec<Pair<u32, InternalKey>>,
    deleted_files_: Vec<Pair<u32, u64>>,
    new_files_: Vec<Pair<u32, FileMetaData>>,
}

pub struct Pair<LEFT, RIGHT> {
    left: LEFT,
    right: RIGHT
}

impl <LEFT, RIGHT>  Pair<LEFT, RIGHT> {
    fn make_pair(l: LEFT, r: RIGHT) -> Self{
        Self{
            left: l,
            right: r,
        }
    }
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
        // todo
        // self.comparator_.clear();
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
        self.comparator_ = name;
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
        self.compact_pointers_.push(Pair::make_pair(level, key))
    }
}