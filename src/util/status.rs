use std::fmt::{Display, Formatter};
use std::io;
use crate::util::r#const::COLON_WHITE_SPACE;
use crate::util::slice::Slice;
use crate::util::status::LevelError::{KCorruption, KIOError, KInvalidArgument, KNotSupported, KNotFound, KOk, KBadRecord};

/// db 中的返回状态，将错误号和错误信息封装成Status类，统一进行处理。
/// 在 leveldb的实现里， 为了节省空间Status将返回码(code), 错误信息message及长度打包存储于一个字符串数组中， 来存储错误信息。
/// 在该项目中， 使用LevelError 和 Slice 存储错误信息
#[derive(Debug)]
pub struct Status {
    err: LevelError,
    msg: Slice
}

impl Default for Status {
    #[inline]
    fn default() -> Self {
        LevelError::ok()
    }
}

impl Status {
    ///
    ///
    /// # Arguments
    ///
    /// * `err`:
    /// * `slice`:
    ///
    /// returns: Status
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn wrapper(err: LevelError, mut slice: Slice) -> Status {
        if err.is_ok() {
            slice = Slice::default();
        }

        Self {
            err,
            msg: slice
        }
    }

    pub fn wrappers(err: LevelError, mut slice1: Slice, slice2: Slice) -> Status {
        slice1.merge(slice2, Some(String::from(COLON_WHITE_SPACE)));

        Self {
            err,
            msg: slice1
        }
    }

    pub fn is_ok(&self) -> bool {
        self.err.is_ok()
    }

    pub fn is_not_found(&self) -> bool {
        self.err.is_not_found()
    }

    pub fn is_corruption(&self) -> bool {
        self.err.is_corruption()
    }

    pub fn is_io_error(&self) -> bool {
        self.err.is_io_error()
    }

    pub fn is_not_supported_error(&self) -> bool {
        self.err.is_not_supported_error()
    }

    pub fn is_invalid_argument(&self) -> bool {
        self.err.is_invalid_argument()
    }

    pub fn get_error_string(&self) -> String {
        self.err.to_string()
    }

    /// 请注意， err 的所有权会发生转移！！！
    pub fn get_error(self) -> LevelError {
        self.err
    }

    /// 得到 LevelError 中的错误信息： Slice
    ///
    /// # Arguments
    ///
    /// returns: Option<Slice>
    ///
    /// # Examples
    ///
    /// ```
    /// let msg1 = "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc";
    /// let msg2 = "456456456456456456456456456456456456456456456456";
    ///
    /// let status: Status = LevelError::io_error(String::from(msg1).into(), String::from(msg2).into());
    /// assert!(&status.get_error().is_io_error());
    ///
    /// let slice: Option<Slice> = status.into_msg();
    /// assert_eq!("abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc: 456456456456456456456456456456456456456456456456",
    ///                                      String::from(slice.unwrap()));
    /// ```
    pub fn into_msg(self) -> Slice {
        self.msg
    }


    /// LevelError 转 String
    ///
    /// # Arguments
    ///
    /// returns: LevelError
    ///
    /// # Examples
    ///
    /// ```
    /// let err: Status = LevelError::invalid_argument(String::from("aa"), String::from("bb"));
    /// let data = err.to_string();
    /// assert_eq!("Invalid argument: aa: bb",  data);
    /// ```
    #[inline]
    pub fn to_string(self) -> String {
        let msg_type = match self.err {
            KOk => "OK",
            KNotFound  => "NotFound: ",
            KCorruption  => "Corruption: ",
            KNotSupported  => "Not implemented: ",
            KInvalidArgument  => "Invalid argument: ",
            KIOError  => "IO error: ",
            KBadRecord=> "wal bad record",
        };

        if self.err.is_ok() {
            return String::from(msg_type);
        }

        let msg = self.msg;
        let error_msg = String::from(msg);

        format!("{}{}", msg_type, error_msg)
    }
}

// impl Display for Status {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let mut print = String::new();
//
//         if self.is_ok() {
//             print.push_str("OK");
//             write!(f, "{}", print);
//
//             return Ok(());
//         }
//
//         print.push_str(&self.get_error_string());
//
//         let slice: &Slice = &self.msg;
//         let errMsg = String::from(slice);
//         print.push_str(errMsg.as_str());
//
//         write!(f, "{}", print);
//
//         Ok(())
//     }
// }

/// Status 的状态
#[derive(Debug)]
pub enum LevelError {
    KOk,
    KNotFound,
    KCorruption,
    KNotSupported,
    KInvalidArgument,
    KIOError,
    KBadRecord,
}

impl LevelError {
    pub fn is_ok(&self) -> bool {
        matches!(*self, KOk)
    }

    pub fn is_not_found(&self) -> bool {
        matches!(*self, KNotFound)
    }

    pub fn is_corruption(&self) -> bool {
        matches!(*self, KCorruption)
    }

    pub fn is_io_error(&self) -> bool {
        matches!(*self, KIOError)
    }

    pub fn is_not_supported_error(&self) -> bool {
        matches!(*self, KNotSupported)
    }

    pub fn is_invalid_argument(&self) -> bool {
        matches!(*self, KInvalidArgument)
    }

    pub fn ok() -> Status {
        Status{
            err: Default::default(),
            msg: Default::default()
        }
    }

    pub fn not_found(mut msg: Slice, msg2: Slice) -> Status {
        let _ = &msg.merge(msg2, Some(String::from(COLON_WHITE_SPACE)));

        Status{
            err: KNotFound,
            msg
        }
    }

    /// 生成 LevelError.KCorruption
    ///
    /// # Arguments
    ///
    /// * `msg`: Slice
    /// * `msg2`: Slice
    ///
    /// returns: LevelError
    ///
    /// # Examples
    ///
    /// ```
    ///  LevelError::corruption(String::from(msg1).into(), String::from(msg2).into())
    /// ```
    pub fn corruption(mut msg: Slice, msg2: Slice) -> Status {
        let _ = &msg.merge(msg2, Some(String::from(COLON_WHITE_SPACE)));

        Status{
            err: KCorruption,
            msg
        }
    }

    pub fn not_supported(mut msg: Slice, msg2: Slice) -> Status {
        let _ = &msg.merge(msg2, Some(String::from(COLON_WHITE_SPACE)));

        Status{
            err: KNotSupported,
            msg
        }
    }

    pub fn invalid_argument(mut msg: Slice, msg2: Slice) -> Status {
        let _ = &msg.merge(msg2, Some(String::from(COLON_WHITE_SPACE)));

        Status{
            err: KInvalidArgument,
            msg
        }
    }

    /// 生成 LevelError.KIOError
    ///
    /// # Arguments
    ///
    /// * `msg`: Slice
    /// * `msg2`: Slice
    ///
    /// returns: LevelError
    ///
    /// # Examples
    ///
    /// ```
    ///  let err: LevelError = LevelError::io_error(String::from("aa"), String::from("bb"));
    ///  assert!(&err.is_io_error());
    /// ```
    pub fn io_error(mut msg: Slice, msg2: Slice) -> Status {
        let _ = &msg.merge(msg2, Some(String::from(COLON_WHITE_SPACE)));

        Status{
            err: KIOError,
            msg
        }
    }
}

impl Default for LevelError {
    #[inline]
    fn default() -> LevelError {
        KOk
    }
}

impl TryFrom<i32> for LevelError {
    type Error = String;

    /// i32 错误码转 LevelError
    ///
    /// # Arguments
    ///
    /// * `value`:  错误码的值
    ///
    /// returns: Result<LevelError, <LevelError as TryFrom<i32>>::Error>
    ///
    /// # Examples
    ///
    /// ```
    ///        let rs: LevelError = LevelError::try_from(3)?;
    ///         assert!(&rs.is_not_supported_error());
    /// ```
    #[inline]
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(KOk),
            1 => Ok(KNotFound),
            2 => Ok(KCorruption),
            3 => Ok(KNotSupported),
            4 => Ok(KInvalidArgument),
            5 => Ok(KIOError),
            6 => Ok(KBadRecord),
            // all other numbers
            _ => Err(String::from(format!("Unknown code: {}", value)))
        }
    }
}

impl From<io::Error> for Status {
    fn from(e: io::Error) -> Self {
        LevelError::io_error(e.to_string().into(), "".into())
    }
}

impl Display for LevelError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut print = String::new();

        let msg_type = match &self {
            KOk => "OK",
            KNotFound  => "NotFound: ",
            KCorruption  => "Corruption: ",
            KNotSupported  => "Not implemented: ",
            KInvalidArgument  => "Invalid argument: ",
            KIOError  => "IO error: ",
            KBadRecord => "wal bad record: ",
        };
        print.push_str(msg_type);

        write!(f, "{}", print)
    }
}

// impl Deref for LevelError {
//     type Target = i32;
//
//     /// StatusTrait 解引用到 i32
//     fn deref(&self) -> &Self::Target {
//         let le = match self {
//             KOk => 0,
//             KNotFound(_) => 1,
//             KCorruption(_) => 2,
//             KNotSupported(_) => 3,
//             KInvalidArgument(_) => 4,
//             KIOError(_) => 5,
//         };
//
//         &*le
//     }
// }