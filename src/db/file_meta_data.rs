use crate::db::db_format::InternalKey;

/// @see version_edit FileMetaData
pub struct FileMetaData {
    refs: u32,
    // Seeks allowed until compaction
    allowed_seeks: u32,
    number: u64,
    // File size in bytes
    file_size: u64,
    // Smallest internal key served by table
    smallest: InternalKey,
    // Largest internal key served by table
    largest: InternalKey
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