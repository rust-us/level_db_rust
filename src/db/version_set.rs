use std::collections::{BTreeMap, BTreeSet};
use crate::db::db_format::InternalKeyComparator;
use crate::db::file_meta_data::FileMetaData;

// line 164
pub struct VersionSet {
}

// line 323
pub struct Compaction {
}

// line 604
pub struct Builder {
    a : BTreeSet<FileMetaData>,
}

struct BySmallestKey {
    internal_comparator: InternalKeyComparator
}

struct LevelState {
    deleted_files: Vec<u64>,
    added_files: BTreeSet<FileMetaData>
}

impl BySmallestKey {
    fn operator(f1: FileMetaData, f2: FileMetaData) -> bool {
        // line 607
        true
    }
}