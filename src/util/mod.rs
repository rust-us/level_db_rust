use crate::util::status::LevelError;
use std::result;

pub mod slice;
mod slice_test;
pub mod coding;
mod coding_test;
pub mod arena;
mod arena_test;

pub use arena::Arena;

pub mod status;
mod status_test;

/// 定义别名
pub type Result<T> = result::Result<T, LevelError>;
