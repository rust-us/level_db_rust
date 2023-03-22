use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;
use crate::db::db_format::{Config, InternalKeyComparator, LookupKey};
use crate::db::file_meta_data::FileMetaData;
use crate::db::table_cache::TableCache;
use crate::db::version_edit::VersionEdit;
use crate::traits::comparator_trait::Comparator;
use crate::util::options::{Env, Options, ReadOptions};
use crate::util::slice::Slice;
use crate::util::Result;

// .h  line 58 - 162
pub struct Version {
    // todo  链表实现，前驱后继
    // VersionSet to which this Version belongs
    vset_: Rc<VersionSet>,
    // Next version in linked list
    next_: Rc<Version>,
    // Previous version in linked list
    prev_: Rc<Version>,

    // Number of live refs to this version
    refs_: i32,

    // List of files per level, 内部vec 初始化长度 config::kNumLevels
    files_: Vec<Vec<FileMetaData>>,

    // Next file to compact based on seek stats.
    file_to_compact_: FileMetaData,
    file_to_compact_level_: i32,

    // Level that should be compacted next and its compaction score.
    // Score < 1 means compaction is not strictly needed.  These fields are initialized by Finalize().
    compaction_score_: f64,
    compaction_level_: i32
}

// .h  line 164 - 320
pub struct VersionSet {
    env_: Env,
    dbname_: Slice,
    options_: Options,
    table_cache_: TableCache,
    icmp_: InternalKeyComparator,
    next_file_number_: u64,
    manifest_file_number_: u64,
    last_sequence_: u64,
    log_number_: u64,
    // 0 or backing store for memtable being compacted
    prev_log_number_: u64,

    // Opened lazily
    // todo WritableFile
    // descriptor_file_: tokio.File,
    // todo log::Writer
    // descriptor_log_:  log::Writer,

    // Head of circular doubly-linked list of versions.
    dummy_versions_: Version,
    // dummy_versions_.prev_
    current_: Version,

    // Per-level key at which the next compaction at that level should start.
    // Either an empty string, or a valid InternalKey.
    compact_pointer_: [String; Config::K_NUM_LEVELS]
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
    seek_file: Rc<FileMetaData>,
    seek_file_level: i32
}

// ,cc line 163
struct LevelFileNumIterator {
    icmp_: InternalKeyComparator,
    flist_: Vec<FileMetaData>,
    index_: u32,

    // // Backing store for value().  Holds the file number and size.
    // mutable char value_buf_[16];
    value_buf_: [u8]
}

// line 604
pub struct Builder {
    vset_: Rc<VersionSet>,

    // VersionSet* ;
    // Version* base_;
    // LevelState levels_[config::kNumLevels];

    // a : BTreeSet<FileMetaData>,
}

struct BySmallestKey {
    internal_comparator: InternalKeyComparator
}

struct LevelState {
    deleted_files: Vec<u64>,

    // replace std::set<FileMetaData*, BySmallestKey> FileSet -> added_files
    added_files: BTreeSet<FileMetaData>
}

impl Version {
    fn new(vset: Rc<VersionSet>) -> Self {
        todo!()
    }

    fn set_next(&mut self, data: &Rc<Version>) -> bool {
        todo!()
    }

    fn set_prev(&mut self, data: &Rc<Version>) -> bool {
        todo!()
    }

    fn get_refs(&self) -> i32 {
        self.refs_
    }

    /// todo 等待 Iterator 接口
    ///
    /// 通过self.versionSet中的TableCache.NewIterator, 将 self 对象数据追加到 iters 中
    ///
    /// Append to *iters a sequence of iterators that will yield the contents of this Version when merged together.
    ///
    /// REQUIRES: This version has been saved (see VersionSet::SaveTo)
    ///
    /// # Arguments
    ///
    /// * `options`:
    /// * `iters`:
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn add_iterators(&self, options: ReadOptions, /* mut iters: Vec<Iterator> */) {
        todo!()
    }

    /// 数据搜索
    ///
    /// 一级一级地搜索，因为条目不会跨越级别。如果在较小的级别上发现数据，则后面的级别是不相关的。
    ///
    /// # Arguments
    ///
    /// * `options`:
    /// * `key`:
    /// * `value`:
    ///
    /// returns: Result<GetStats, Status>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn get(&self, options: ReadOptions, key: LookupKey, value: Slice) -> Result<GetStats> {
        todo!()
    }

    /// Adds "stats" into the current state.
    /// Returns true if a new compaction may need to be triggered, false otherwise.
    ///
    /// REQUIRES: lock is held
    ///
    /// # Arguments
    ///
    /// * `stats`:
    ///
    /// returns: bool
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn update_stats(&mut self, stats: GetStats) -> bool{
        todo!()
    }

    /// 记录在指定内部键处读取的字节样本。
    /// 大约每 config:：K_READ_BYTES_PERIOD 字节采样一次。如果可能需要触发新的压缩，则返回true
    ///
    /// Record a sample of bytes read at the specified internal key.
    /// Samples are taken approximately once every config::kReadBytesPeriod
    /// bytes.  Returns true if a new compaction may need to be triggered.
    ///
    /// REQUIRES: lock is held  要求：锁定
    ///
    /// # Arguments
    ///
    /// * `key`:
    ///
    /// returns: bool
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn record_read_sample(&self, key: Slice) -> bool {
        todo!()
    }

    // //
    // // Reference count management (so Versions do not disappear out from
    // // under live iterators)
    // void Ref();
    // void Unref();
    //
    // void GetOverlappingInputs(
    // int level,
    // const InternalKey* begin,         // nullptr means before all keys
    // const InternalKey* end,           // nullptr means after all keys
    // std::vector<FileMetaData*>* inputs);
    //
    // // Returns true iff some file in the specified level overlaps
    // // some part of [*smallest_user_key,*largest_user_key].
    // // smallest_user_key==nullptr represents a key smaller than all the DB's keys.
    // // largest_user_key==nullptr represents a key largest than all the DB's keys.
    // bool OverlapInLevel(int level,
    // const Slice* smallest_user_key,
    // const Slice* largest_user_key);
    //
    // // Return the level at which we should place a new memtable compaction
    // // result that covers the range [smallest_user_key,largest_user_key].
    // int PickLevelForMemTableOutput(const Slice& smallest_user_key,
    // const Slice& largest_user_key);
    //
    // int NumFiles(int level) const { return files_[level].size(); }
    //
    // // Return a human readable string that describes this version's contents.
    // std::string DebugString() const;

    // Iterator* NewConcatenatingIterator(const ReadOptions&, int level) const;
    //
    // // Call func(arg, level, f) for every file that overlaps user_key in
    // // order from newest to oldest.  If an invocation of func returns
    // // false, makes no more calls.
    // //
    // // REQUIRES: user portion of internal_key == user_key.
    // void ForEachOverlapping(Slice user_key, Slice internal_key,
    // void* arg,
    // bool (*func)(void*, int, FileMetaData*));
}

impl Drop for Version {
    /// version_set.cc 中析构方法 Version::~Version() 的对应实现
    fn drop(&mut self) {
        assert_eq!(self.get_refs(), 0);

        // // Remove from linked list
        // self.prev_.set_next(&self.next_);
        // self.next_.set_prev(&self.prev_);

        // // Drop references to files
        // for level in 0..Config::K_NUM_LEVELS {
        //     for i in 0..self.files_[level].len() {
        //         let mut meta: FileMetaData = files_[level][i];
        //         assert!(meta.get_refs() > 0);
        //
        //         meta.add_refs(-1);
        //         if meta.get_refs() <= 0 {
        //             // todo  C 语义  delete f
        //             // delete f;
        //         }
        //     }
        // }
    }
}

// impl VersionSet {
//     //.
// }

impl VersionSet {
    /// 返回文件源数据中最小的索引。 如果文件不存在，则返回文件数量
    ///
    /// Return the smallest index i such that files[i]->largest >= key.
    /// Return files.size() if there is no such file.
    ///
    /// REQUIRES: "files" contains a sorted list of non-overlapping files.
    /// # Arguments
    ///
    /// * `icmp`:
    /// * `files`:
    /// * `key`:
    ///
    /// returns: u32
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn find_file(icmp: InternalKeyComparator, files:&Vec<FileMetaData>, key:&Slice) -> u32 {
        todo!()
    }

    /// 如果 user key 范围[smallest_user_key, largest_user_key] 与 “files”中的 [smallest.user_key(), largest.user_key()] 重叠，则返回true
    /// minimal==nullptr表示比DB中的所有键都小的键。
    /// maximum==nullptr表示比DB中的所有键都大的键。
    ///
    /// Returns true iff some file in "files" overlaps the user key range [smallest_user_key, largest_user_key].
    /// smallest==nullptr represents a key smaller than all keys in the DB.
    /// largest==nullptr represents a key largest than all keys in the DB.
    ///
    /// REQUIRES: If disjoint_sorted_files, files[] contains disjoint ranges
    ///           in sorted order.
    ///
    /// # Arguments
    ///
    /// * `icmp`:
    /// * `disjoint_sorted_files`:
    /// * `files`:
    /// * `smallest_user_key`:
    /// * `largest_user_key`:
    ///
    /// returns: bool
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn some_file_overlaps_range(icmp: InternalKeyComparator, disjoint_sorted_files:bool,
                                files:&Vec<FileMetaData>, smallest_user_key:&Slice,largest_user_key:&Slice) -> bool {
        todo!()
    }
}

impl Compaction {
    // fn create(options: Options, level: u32) -> Self {
    //     Self {
    //
    //     }
    // }
}

// todo 等到 Iterator 接口
// impl Iterator for LevelFileNumIterator {
//
// }

impl Builder {
    
}

impl BySmallestKey {

    /// FileMetaData 比较
    ///
    /// # Arguments
    ///
    /// * `f1`:
    /// * `f2`:
    ///
    /// returns: bool
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn operator(&self, f1: &FileMetaData, f2: &FileMetaData) -> bool {
        // line 607
        let r: u32 = self.internal_comparator.compare_internal_key(
            f1.get_smallest(),
            f2.get_smallest()
        );

        if r != 0 {
            return r < 0;
        }

        // Break ties by file number
        f1.get_number() < f2.get_number()
    }
}