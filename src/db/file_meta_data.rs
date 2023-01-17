
pub struct FileMetaData {
    refs: u32,
    // Seeks allowed until compaction
    allowed_seeks: u32,
    number: u64,
    // File size in bytes
    file_size: u64,
    // smallest: InternalKey,

}