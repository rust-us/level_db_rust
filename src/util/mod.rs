use std::result;

pub use arena::Arena;

use crate::util::comparator::{BytewiseComparatorImpl};
use crate::util::status::Status;

/// 常量定义
pub mod r#const;

pub mod slice;
mod slice_test;
pub mod cache;
pub mod coding;
pub mod arena;
mod arena_test;

pub mod status;
mod status_test;
pub mod comparator;
mod comparator_test;
pub mod crc;
mod crc_test;
// pub mod bloom_filter;
// mod bloom_filter_test;
pub mod filter_policy;
mod filter_policy_test;
pub mod filter_policy_bloom;
mod filter_policy_bloom_test;
pub mod filter_policy_internal;
mod filter_policy_internal_test;

pub mod histogram;
mod histogram_test;
pub mod hash;
mod hash_test;
pub mod mutex_lock;
mod mutex_lock_test;
pub mod random;
mod random_test;
pub mod options;
pub mod debug;
pub mod linked_list;
mod linked_list_test;
pub mod unsafe_slice;
pub mod env;
mod env_test;
pub mod mem_debug;
// pub mod error;
// mod error_test;
// pub mod error_code;

/// 定义别名
pub type Result<T> = result::Result<T, Status>;
