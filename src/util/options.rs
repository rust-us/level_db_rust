use crate::db::db::Snapshot;
use crate::traits::comparator_trait::Comparator;
use crate::util::comparator::BytewiseComparatorImpl;

pub enum CompressionType {
    NoCompression,
    SnappyCompression
}

/// TODO temp
pub struct  Env {}

pub struct Cache {}

pub struct FilterPolicy {}

pub struct Options {

    /// Comparator used to define the order of keys in the table.
    /// Default: a comparator that uses lexicographic byte-wise ordering
    ///
    /// REQUIRES: The client must ensure that the comparator supplied
    /// here has the same name and orders keys *exactly* the same as the
    /// comparator provided to previous open calls on the same DB.
    pub cmp: Box<dyn Comparator>,
    /// If true, the database will be created if it is missing.
    pub create_if_missing: bool,
    /// If true, an error is raised if the database already exists.
    pub error_if_exists: bool,
    /// If true, the implementation will do aggressive checking of the
    /// data it is processing and will stop early if it detects any
    /// errors.  This may have unforeseen ramifications: for example, a
    /// corruption of one DB entry may cause a large number of entries to
    /// become unreadable or for the entire DB to become unopenable.
    pub paranoid_checks: bool,
    /// Use the specified object to interact with the environment,
    /// e.g. to read/write files, schedule background work, etc.
    /// Default: Env::Default()
    pub env: Env,

    /// Amount of data to build up in memory (backed by an unsorted log
    /// on disk) before converting to a sorted on-disk file.
    ///     /// Larger values increase performance, especially during bulk loads.
    /// Up to two write buffers may be held in memory at the same time,
    /// so you may wish to adjust this parameter to control memory usage.
    /// Also, a larger write buffer will result in a longer recovery time
    /// the next time the database is opened.
    pub write_buffer_size: usize,
    /// Number of open files that can be used by the DB.  You may need to
    /// increase this if your database has a large working set (budget
    /// one open file per 2MB of working set).
    pub max_open_files: u32,
    /// Control over blocks (user data is stored in a set of blocks, and
    /// a block is the unit of reading from disk).

    /// If non-null, use the specified cache for blocks.
    /// If null, leveldb will automatically create and use an 8MB internal cache.
    pub block_cache: Option<Cache>,

    /// Approximate size of user data packed per block.  Note that the
    /// block size specified here corresponds to uncompressed data.  The
    /// actual size of the unit read from disk may be smaller if
    /// compression is enabled.  This parameter can be changed dynamically.
    pub block_size: usize,
    /// Number of keys between restart points for delta encoding of keys.
    /// This parameter can be changed dynamically.  Most clients should
    /// leave this parameter alone.
    pub block_restart_interval: u32,
    /// Leveldb will write up to this amount of bytes to a file before
    /// switching to a new one.
    /// Most clients should leave this parameter alone.  However if your
    /// filesystem is more efficient with larger files, you could
    /// consider increasing the value.  The downside will be longer
    /// compactions and hence longer latency/performance hiccups.
    /// Another reason to increase this parameter might be when you are
    /// initially populating a large database.
    pub max_file_size: usize,
    /// Compress blocks using the specified compression algorithm.  This
    /// parameter can be changed dynamically.
    ///     /// Default: kSnappyCompression, which gives lightweight but fast
    /// compression.
    ///     /// Typical speeds of kSnappyCompression on an Intel(R) Core(TM)2 2.4GHz:
    ///    ~200-500MB/s compression
    ///    ~400-800MB/s decompression
    /// Note that these speeds are significantly faster than most
    /// persistent storage speeds, and therefore it is typically never
    /// worth switching to kNoCompression.  Even if the input data is
    /// incompressible, the kSnappyCompression implementation will
    /// efficiently detect that and will switch to uncompressed mode.
    pub compression: CompressionType,
    /// EXPERIMENTAL: If true, append to existing MANIFEST and log files
    /// when a database is opened.  This can significantly speed up open.
    ///     /// Default: currently false, but may become true later.
    pub reuse_logs: bool,
    /// If non-null, use the specified filter policy to reduce disk reads.
    /// Many applications will benefit from passing the result of
    /// NewBloomFilterPolicy() here.
    pub filter_policy: Option<FilterPolicy>,
}
/// Options that control read operations
pub struct ReadOptions {
    /// If true, all data read from underlying storage will be
    /// verified against corresponding checksums.
    pub verify_checksums: bool,
    /// Should the data read for this iteration be cached in memory?
    /// Callers may wish to set this field to false for bulk scans.
    pub fill_cache: bool,
    /// If "snapshot" is non-null, read as of the supplied snapshot
    /// (which must belong to the DB that is being read and which must
    /// not have been released).  If "snapshot" is null, use an implicit
    /// snapshot of the state at the beginning of this read operation.
    pub snapshot: Option<Box<dyn Snapshot>>,
}

/// Options that control write operations
pub struct WriteOptions {
    /// If true, the write will be flushed from the operating system
    /// buffer cache (by calling WritableFile::Sync()) before the write
    /// is considered complete.  If this flag is true, writes will be
    /// slower.
    ///     /// If this flag is false, and the machine crashes, some recent
    /// writes may be lost.  Note that if it is just the process that
    /// crashes (i.e., the machine does not reboot), no writes will be
    /// lost even if sync==false.
    ///     /// In other words, a DB write with sync==false has similar
    /// crash semantics as the "write()" system call.  A DB write
    /// with sync==true has similar crash semantics to a "write()"
    /// system call followed by "fsync()".
    pub sync: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            cmp: Box::new(BytewiseComparatorImpl::default()),
            create_if_missing: false,
            error_if_exists: false,
            paranoid_checks: false,
            env: Env {},
            write_buffer_size: 4 * 1024 * 1024,
            max_open_files: 1000,
            block_cache: None,
            block_size: 4 * 1024,
            block_restart_interval: 16,
            max_file_size: 2 * 1024 * 1024,
            compression: CompressionType::NoCompression,
            reuse_logs: false,
            filter_policy: None
        }
    }
}

impl Default for ReadOptions {
    fn default() -> Self {
        Self {
            verify_checksums: false,
            fill_cache: true,
            snapshot: None
        }
    }
}

impl Default for WriteOptions {
    fn default() -> Self {
        Self {
            sync: false
        }
    }
}