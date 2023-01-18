use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;
use crate::db::db_format::{InternalKey, InternalKeyComparator};

/// @see version_edit FileMetaData
#[derive(Debug)]
pub struct FileMetaData {
    refs: i32,
    // Seeks allowed until compaction
    allowed_seeks: i32,
    number: u64,
    // File size in bytes
    file_size: u64,
    // Smallest internal key served by table
    smallest: InternalKey,
    // Largest internal key served by table
    largest: InternalKey
}

#[allow(improper_ctypes)]
extern {
    fn memcmp(s1: *const i32, s2: *const i32) -> i32;
}

impl Default for FileMetaData {
    #[inline]
    fn default() -> Self {
        Self {
            refs: 0,
            allowed_seeks: 1 << 30,
            number: 0,
            file_size: 0,
            smallest: InternalKey::default(),
            largest: InternalKey::default()
        }
    }
}

impl FileMetaData {
    pub fn create_refs(refs: i32) -> Self {
        FileMetaData::create_refs_allowed_seeks(refs, 1 << 30)
    }

    pub fn create_refs_allowed_seeks(refs: i32, allowed_seeks: i32) -> Self {
        FileMetaData::create_refs_allowed_seeks_file_size(refs, allowed_seeks, 0)
    }

    pub fn create_refs_allowed_seeks_file_size(refs: i32, allowed_seeks: i32, file_size: u64) -> Self {
        FileMetaData::create_refs_allowed_seeks_file_size_internal_key(refs, allowed_seeks, file_size, InternalKey::default(), InternalKey::default())
    }

    pub fn create_refs_allowed_seeks_file_size_internal_key(refs: i32, allowed_seeks: i32, file_size: u64,
                                                            smallest: InternalKey, largest: InternalKey) -> Self {
        FileMetaData::create(refs, allowed_seeks, 0, file_size, smallest, largest)
    }

    pub fn create_number_file_size_internal_key(number: u64, file_size: u64, smallest: InternalKey, largest: InternalKey) -> Self {
        FileMetaData::create(0, 1 << 30, number, file_size, smallest, largest)
    }

    pub fn create(refs: i32, allowed_seeks: i32, number: u64, file_size: u64, smallest: InternalKey, largest: InternalKey) -> Self {
        Self {
            refs,
            allowed_seeks,
            number,
            file_size,
            smallest,
            largest
        }
    }

    pub fn get_smallest(&self) -> &InternalKey {
        &self.smallest
    }

    pub fn get_number(&self) -> u64 {
        self.number
    }

    pub fn get_refs(&self) -> i32 {
        self.refs
    }

    pub fn add_refs(&mut self, num: i32) {
        self.refs += num;
    }
}

impl PartialEq for FileMetaData {
    /// 判断两个 FileMetaData 是否相同
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.refs == other.refs
        && self.allowed_seeks == other.allowed_seeks
        && self.number == other.number
        && self.file_size == other.file_size
        && self.smallest.eq(&other.smallest)
        && self.largest.eq(&other.largest)
    }
}

impl PartialOrd for FileMetaData {
    /// 判断两个 FileMetaData 的大小关系
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // todo
        // InternalKeyComparator::compare()
        Option::Some(Ordering::Equal)
    }
}