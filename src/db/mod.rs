use crate::db::mem_table::MemTable;
use crate::db::skip_list::SkipList;
use crate::util::comparator::{BytewiseComparatorImpl, InternalKeyComparator};
use crate::util::slice::Slice;

pub mod log_writer;
pub mod log_reader;
mod log_wr_test;
pub mod skip_list;
pub mod mem_table;
pub mod db;
mod skip_list_test;

/// 默认调表
pub type DefaultSkipList = SkipList<BytewiseComparatorImpl>;
/// 默认内存表
pub type DefaultMemTable = MemTable<InternalKeyComparator>;