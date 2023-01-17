use std::collections::{BTreeMap, BTreeSet};
use crate::db::db_format::InternalKeyComparator;
use crate::db::file_meta_data::FileMetaData;
use crate::db::version_edit::VersionEdit;
use crate::util::cache::Cache;
use crate::util::options::{Env, Options};
use crate::util::slice::Slice;

// .h  line 58 - 162
pub struct Version {

    // VersionSet* vset_;            // VersionSet to which this Version belongs
    // Version* next_;               // Next version in linked list
    // Version* prev_;               // Previous version in linked list
    // int refs_;                    // Number of live refs to this version
    //
    // // List of files per level
    // std::vector<FileMetaData*> files_[config::kNumLevels];
    //
    // // Next file to compact based on seek stats.
    // FileMetaData* file_to_compact_;
    // int file_to_compact_level_;
    //
    // // Level that should be compacted next and its compaction score.
    // // Score < 1 means compaction is not strictly needed.  These fields
    // // are initialized by Finalize().
    // double compaction_score_;
    // int compaction_level_;
}

// .h  line 164 - 320
pub struct VersionSet {
    // TableCache
}

// .h  line 323 - 393
pub struct Compaction {
    level_: u32,
    max_output_file_size_: u64,
    input_version_: Version,
    edit_: VersionEdit

    // // Each compaction reads inputs from "level_" and "level_+1"
    // std::vector<FileMetaData*> inputs_[2];      // The two sets of inputs
    //
    // // State used to check for number of overlapping grandparent files
    // // (parent == level_ + 1, grandparent == level_ + 2)
    // std::vector<FileMetaData*> grandparents_;
    // size_t grandparent_index_;  // Index in grandparent_starts_
    // bool seen_key_;             // Some output key has been seen
    // int64_t overlapped_bytes_;  // Bytes of overlap between current output
    // // and grandparent files
    //
    // // State for implementing IsBaseLevelForKey
    //
    // // level_ptrs_ holds indices into input_version_->levels_: our state
    // // is that we are positioned at one of the file ranges for each
    // // higher level than the ones involved in this compaction (i.e. for
    // // all L >= level_ + 2).
    // size_t level_ptrs_[config::kNumLevels];
}

// .h   line 68 - 71
struct GetStats {
    seek_file: FileMetaData,
    seek_file_level: u32
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

impl Compaction {
    fn create(options: Options, level: u32) -> Self {
        Self {

        }
    }
}

impl BySmallestKey {
    fn operator(f1: FileMetaData, f2: FileMetaData) -> bool {
        // line 607
        true
    }
}

// todo  临时使用。等待提供方
struct TableCache {
    // todo  Env 临时使用。等待提供方
    env_: Env,
    dbname_: Slice
}