use crate::db::db_format::InternalKeyComparator;
use crate::db::skip_list::SkipList;
<<<<<<< HEAD
use crate::db::mem_table::MemTable;
use crate::util::comparator::{BytewiseComparatorImpl};
use crate::util::slice::Slice;
=======
use crate::util::comparator::{BytewiseComparatorImpl, InternalKeyComparator};

>>>>>>> 7ab46579f8abd8c45c40227dfb601ec7468625eb

pub mod log_writer;
pub mod log_reader;
pub mod table_cache;
mod log_wr_test;
pub mod skip_list;
pub mod mem_table;
pub mod db;
mod skip_list_test;
pub mod db_format;
mod db_format_test;
pub mod file_meta_data;
mod file_meta_data_test;
pub mod version_set;
mod version_set_test;
pub mod version_edit;
mod version_edit_test;

/// 默认调表
pub type DefaultSkipList = SkipList<BytewiseComparatorImpl>;
/// 默认内存表
pub type DefaultMemTable = MemTable<InternalKeyComparator>;