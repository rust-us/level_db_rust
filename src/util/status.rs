/// 使用 Status 这个类来得到你的函数的返回的状态
///
use crate::util::slice::Slice;
use crate::util::status::LevelError::{KCorruption, KIOError, KInvalidArgument, KNotSupported, KNotFound, KOk};

/// Status 的状态
pub enum LevelError {
    KOk,
    KNotFound(Option<Slice>),
    KCorruption(Option<Slice>),
    KNotSupported(Option<Slice>),
    KInvalidArgument(Option<Slice>),
    KIOError(Option<Slice>),

}

impl Default for LevelError {
    fn default() -> Self {
        KOk
    }
}

impl LevelError {
    pub fn get_code(&self) -> u32 {
        match self {
            KOk => {0},
            KNotFound(_) => {1},
            KCorruption(_) => {2},
            KNotSupported(_) => {3},
            KInvalidArgument(_) => {4},
            KIOError(_) => {5},
        }
    }

    /// 得到 error 中的 slice 信息
    pub fn into_msg(self) -> Option<Slice> {
        match self {
            KOk => {None},
            KNotFound(slice) => {
                slice
            },
            KCorruption(slice) => {
                slice
            },
            KNotSupported(slice) => {
                slice
            },
            KInvalidArgument(slice) => {
                slice
            },
            KIOError(slice) => {
                slice
            },
        }
    }

    /// 判断 状态是否为默认值
    fn is_default(&self) -> bool {
        self.is_ok()
    }

    /// Returns true iff the status indicates success.
    pub fn is_ok(&self) -> bool {
        match self {
            KOk => true,
            _ => false
        }
    }

    /// Returns true iff the status indicates a NotFound error.
    pub fn is_not_found(&self) -> bool {
        match self {
            KNotFound(_) => true,
            _ => false
        }
    }

    /// Returns true iff the status indicates a Corruption error.
    pub fn is_corruption(&self) -> bool {
        match self {
            KCorruption(_) => true,
            _ => false
        }
    }

    /// Returns true iff the status indicates an IOError.
    pub fn is_io_error(&self) -> bool {
        match self {
            KIOError(_) => true,
            _ => false
        }
    }

    /// Returns true iff the status indicates a NotSupportedError.
    pub fn is_not_supported_error(&self) -> bool {
        match self {
            KNotSupported(_) => true,
            _ => false
        }
    }

    /// Returns true iff the status indicates an InvalidArgument.
    pub fn is_invalid_argument(&self) -> bool {
        match self {
            KInvalidArgument(_) => true,
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
            KOk => {
                _type = "OK";
            },
            KNotFound(_)  => {
                _type = "NotFound: ";
            },
            KCorruption(_)  => {
                _type = "Corruption: ";
            },
            KNotSupported(_)  => {
                _type = "Not implemented: ";
            },
            KInvalidArgument(_)  => {
                _type = "Invalid argument: ";
            },
            KIOError(_)  => {
                _type = "IO error: ";
            }
        }

        // todo

        String::from(_type)
    }
}

/// 这一组函数用来组合指定的状态信息
impl<'a> LevelError {
    /// 返回 ok 的 Status
    pub fn ok() -> LevelError {
        KOk
    }

    /// 返回 not_found 的 Status
    pub fn not_found(mut msg: Slice, msg2: Slice) -> LevelError {
        msg.merge(msg2, Some(String::from(": ")));
        KNotFound(Some(msg))
    }

    /// 返回 Corruption 的 Status
    pub fn corruption(mut msg: Slice, msg2: Slice) -> LevelError {
        msg.merge(msg2, Some(String::from(": ")));
        KCorruption(Some(msg))
    }

    /// 返回 NotSupported 的 Status
    pub fn not_supportedfound(mut msg: Slice, msg2: Slice) -> LevelError {
        msg.merge(msg2, Some(String::from(": ")));
        KNotSupported(Some(msg))
    }

    /// 返回 InvalidArgument 的 Status
    pub fn invalid_argument(mut msg: Slice, msg2: Slice) -> LevelError {
        msg.merge(msg2, Some(String::from(": ")));
        KInvalidArgument(Some(msg))
    }

    /// 返回 IOError 的 Status
    pub fn io_error(mut msg: Slice, msg2: Slice) -> LevelError {
        msg.merge(msg2, Some(String::from(": ")));
        KIOError(Some(msg))
    }

}
