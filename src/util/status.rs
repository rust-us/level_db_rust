
/// 使用 Status 这个类来得到你的函数的返回的状态
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
    fn default() -> Self {
        KOk
    }
}

impl LevelError {

    /// 判断 状态是否为默认值
    fn is_default(&self) -> bool {
        self.is_ok()
    }

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

        let msg_type = match self {
            KOk => "OK",
            KNotFound(_)  => "NotFound: ",
            KCorruption(_)  => "Corruption: ",
            KNotSupported(_)  => "Not implemented: ",
            KInvalidArgument(_)  => "Invalid argument: ",
            KIOError(_)  => "IO error: "
        };

        let error_msg = String::from(self.into_msg().unwrap());

        format!("{}{}", String::from(msg_type), error_msg)
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
        &msg.merge(msg2, Some(String::from(COLON_WHITE_SPACE)));
        KNotFound(Some(msg))
    }

    /// 返回 Corruption 的 Status
    pub fn corruption(mut msg: Slice, msg2: Slice) -> LevelError {
        &msg.merge(msg2, Some(String::from(COLON_WHITE_SPACE)));
        KCorruption(Some(msg))
    }

    /// 返回 NotSupported 的 Status
    pub fn not_supported(mut msg: Slice, msg2: Slice) -> LevelError {
        &msg.merge(msg2, Some(String::from(COLON_WHITE_SPACE)));
        KNotSupported(Some(msg))
    }

    /// 返回 InvalidArgument 的 Status
    pub fn invalid_argument(mut msg: Slice, msg2: Slice) -> LevelError {
        &msg.merge(msg2, Some(String::from(COLON_WHITE_SPACE)));
        KInvalidArgument(Some(msg))
    }

    /// 返回 IOError 的 Status
    pub fn io_error(mut msg: Slice, msg2: Slice) -> LevelError {
        &msg.merge(msg2, Some(String::from(COLON_WHITE_SPACE)));
        KIOError(Some(msg))
    }

}


/// 非 pub 方法的测试用例
#[test]
fn test() {
    let err: LevelError = LevelError::ok();
    assert!(err.is_default());

    let err: LevelError = LevelError::io_error(String::from("a").into(),
                                               String::from("b").into());
    assert!(!err.is_default());
}