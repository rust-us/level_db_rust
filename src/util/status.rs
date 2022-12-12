/// 使用 Status 这个类来得到你的函数的返回的状态
///
use std::borrow::{Borrow, Cow};
use std::fmt::Debug;
use std::ptr;
use std::ptr::{NonNull, null};
use crate::util::slice::Slice;
use crate::util::status::LevelError::{kCorruption, kInvalidArgument, kIOError, kNotFound, kNotSupported, kOk};

/// Status 的状态
pub enum LevelError {
    kOk,
    kNotFound(Option<Slice>),
    kCorruption(Option<Slice>),
    kNotSupported(Option<Slice>),
    kInvalidArgument(Option<Slice>),
    kIOError(Option<Slice>),

}

impl Default for LevelError {
    fn default() -> Self {
        kOk
    }
}

impl LevelError {
    pub fn get_code(&self) -> u32 {
        match self {
            kOk => {0},
            kNotFound(_) => {1},
            kCorruption(_) => {2},
            kNotSupported(_) => {3},
            kInvalidArgument(_) => {4},
            kIOError(_) => {5},
        }
    }

    /// 得到 error 中的 slice 信息
    pub fn into_msg(self) -> Option<Slice> {
        match self {
            kOk => {None},
            kNotFound(slice) => {
                slice
            },
            kCorruption(slice) => {
                // println!("The slice to be {:?}", slice);
                slice
            },
            kNotSupported(slice) => {
                // println!("The slice to be {:?}", slice);
                slice
            },
            kInvalidArgument(slice) => {
                // println!("The slice to be {:?}", slice);
                slice
            },
            kIOError(slice) => {
                // println!("The slice to be {:?}", slice);
                slice
            },
        }
    }

    /// 判断 状态是否为默认值
    fn is_default(&self) -> bool {
        self.ok()
    }

    /// Returns true iff the status indicates success.
    pub fn ok(&self) -> bool {
        match self {
            kOk => true,
            _ => false
        }
    }

    /// Returns true iff the status indicates a NotFound error.
    pub fn is_not_found(&self) -> bool {
        match self {
            kNotFound(_) => true,
            _ => false
        }
    }

    /// Returns true iff the status indicates a Corruption error.
    pub fn is_corruption(&self) -> bool {
        match self {
            kCorruption(_) => true,
            _ => false
        }
    }

    /// Returns true iff the status indicates an IOError.
    pub fn is_io_error(&self) -> bool {
        match self {
            kIOError(_) => true,
            _ => false
        }
    }

    /// Returns true iff the status indicates a NotSupportedError.
    pub fn is_not_supported_error(&self) -> bool {
        match self {
            kNotSupported(_) => true,
            _ => false
        }
    }

    /// Returns true iff the status indicates an InvalidArgument.
    pub fn is_invalid_argument(&self) -> bool {
        match self {
            kInvalidArgument(_) => true,
            _ => false
        }
    }

    /// Return a string representation of this status suitable for printing.
    /// Returns the string "OK" for success.
    pub fn to_string(self) -> String {
        if self.is_default() {
            return String::from("OK")
        }

        let _tmp:Vec<char> = Vec::with_capacity(30);
        let mut _type: &str = "";

        match self {
            kOk => {
                _type = "OK";
            },
            kNotFound(_)  => {
                _type = "NotFound: ";
            },
            kCorruption(_)  => {
                _type = "Corruption: ";
            },
            kNotSupported(_)  => {
                _type = "Not implemented: ";
            },
            kInvalidArgument(_)  => {
                _type = "Invalid argument: ";
            },
            kIOError(_)  => {
                _type = "IO error: ";
            }
        }

        // todo

        String::from(_type)
    }
}

// 这一组函数用来组合指定的状态信息
impl<'a> LevelError {
    /// 返回 ok 的 Status
    pub fn OK() -> LevelError {
        kOk
    }

    /// 返回 not_found 的 Status
    pub fn not_found(msg: Slice, msg2: Slice) -> LevelError {
        kNotFound(Some(msg))
    }

    /// 返回 Corruption 的 Status
    pub fn corruption(msg: Slice, msg2: Slice) -> LevelError {
        kCorruption(Some(msg))
    }

    /// 返回 NotSupported 的 Status
    pub fn not_supportedfound(msg: Slice, msg2: Slice) -> LevelError {
        LevelError::kNotSupported(Some(msg))
    }

    /// 返回 InvalidArgument 的 Status
    pub fn invalid_argument(msg: Slice, msg2: Slice) -> LevelError {
        LevelError::kInvalidArgument(Some(msg))
    }

    /// 返回 IOError 的 Status
    pub fn io_error(msg: Slice, msg2: Slice) -> LevelError {
        LevelError::kIOError(Some(msg))
    }
}
