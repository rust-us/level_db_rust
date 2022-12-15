use crate::util::status::LevelError;
use std::result;
pub use arena::Arena;

/// 常量定义
pub mod r#const;

pub mod slice;
mod slice_test;
pub mod coding;
mod coding_test;
pub mod arena;
mod arena_test;

pub mod status;
mod status_test;
pub mod comparator;
mod comparator_test;
mod crc;
mod crc_test;
pub mod bloom_filter;
mod bloom_filter_test;
pub mod filter_policy;

/// 定义别名
pub type ResultT<T> = result::Result<T, LevelError>;

pub mod histogram;
mod histogram_test;
mod hash;
mod hash_test;
