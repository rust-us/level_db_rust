use std::fmt::Error;
use crate::traits::status_trait::StatusTrait;
/// 使用 Status 这个类来得到函数的返回的状态
///
use crate::util::r#const::COLON_WHITE_SPACE;
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
    fn default() -> LevelError {
        KOk
    }
}

impl StatusTrait for LevelError {

    fn is_ok(&self) -> bool {
        match self {
            KOk => true,
            _ => false
        }
    }

    fn is_not_found(&self) -> bool {
        match self {
            KNotFound(_) => true,
            _ => false
        }
    }

    fn is_corruption(&self) -> bool {
        match self {
            KCorruption(_) => true,
            _ => false
        }
    }

    fn is_io_error(&self) -> bool {
        match self {
            KIOError(_) => true,
            _ => false
        }
    }

    fn is_not_supported_error(&self) -> bool {
        match self {
            KNotSupported(_) => true,
            _ => false
        }
    }

    fn is_invalid_argument(&self) -> bool {
        match self {
            KInvalidArgument(_) => true,
            _ => false
        }
    }

    fn ok() -> LevelError {
        KOk
    }

    fn not_found(mut msg: Slice, msg2: Slice) -> LevelError {
        &msg.merge(msg2, Some(String::from(COLON_WHITE_SPACE)));
        KNotFound(Some(msg))
    }

    fn corruption(mut msg: Slice, msg2: Slice) -> LevelError {
        &msg.merge(msg2, Some(String::from(COLON_WHITE_SPACE)));
        KCorruption(Some(msg))
    }

    fn not_supported(mut msg: Slice, msg2: Slice) -> LevelError {
        &msg.merge(msg2, Some(String::from(COLON_WHITE_SPACE)));
        KNotSupported(Some(msg))
    }

    fn invalid_argument(mut msg: Slice, msg2: Slice) -> LevelError {
        &msg.merge(msg2, Some(String::from(COLON_WHITE_SPACE)));
        KInvalidArgument(Some(msg))
    }

    fn io_error(mut msg: Slice, msg2: Slice) -> LevelError {
        &msg.merge(msg2, Some(String::from(COLON_WHITE_SPACE)));
        KIOError(Some(msg))
    }

    fn is_default(&self) -> bool {
        self.is_ok()
    }

    fn into_code(&self) -> u32 {
        match self {
            KOk => {0},
            KNotFound(_) => {1},
            KCorruption(_) => {2},
            KNotSupported(_) => {3},
            KInvalidArgument(_) => {4},
            KIOError(_) => {5},
        }
    }

    fn into_msg(self) -> Option<Slice> {
        match self {
            KOk => None,
            /// 以后可能会差异化处理，因此不做 _ 的默认输出
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

    #[inline]
    fn to_string(self) -> String {
        if self.is_default() {
            return String::from("OK")
        }

        let msg_type = match self {
            KOk => "OK",
            KNotFound(_)  => "NotFound: ",
            KCorruption(_)  => "Corruption: ",
            KNotSupported(_)  => "Not implemented: ",
            KInvalidArgument(_)  => "Invalid argument: ",
            KIOError(_)  => "IO error: "
        };

        let error_msg = String::from(self.into_msg().unwrap());

        format!("{}{}", msg_type, error_msg)
    }
}


impl TryFrom<u32> for LevelError {
    type Error = String;

    /// 错误码转 LevelError
    ///
    /// # Arguments
    ///
    /// * `value`:  错误码的值
    ///
    /// returns: Result<LevelError, <LevelError as TryFrom<u32>>::Error>
    ///
    /// # Examples
    ///
    /// ```
    ///        let rs: LevelError = LevelError::try_from(3)?;
    ///         assert!(&rs.is_not_supported_error());
    /// ```
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(KOk),
            1 => Ok(KNotFound(None)),
            2 => Ok(KCorruption(None)),
            3 => Ok(KNotSupported(None)),
            4 => Ok(KInvalidArgument(None)),
            5 => Ok(KIOError(None)),
            // all other numbers
            _ => Err(String::from(format!("Unknown code: {}", value)))
        }
    }
}