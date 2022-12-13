use crate::util::slice::Slice;
use crate::util::status::LevelError;

pub trait StatusTrait {

    /// 判断 状态是否为默认值
    ///
    /// # Arguments
    ///
    /// returns: bool, true 为默认值， false 为不是默认值
    ///
    /// # Examples
    ///
    /// ```
    ///  let err: LevelError = LevelError::ok();
    ///  assert!(err.is_default());
    /// ```
    fn is_default(&self) -> bool;

    fn into_code(&self) -> u32;

    fn into_msg(self) -> Option<Slice>;

    fn is_ok(&self) -> bool;

    fn is_not_found(&self) -> bool;

    fn is_corruption(&self) -> bool;

    fn is_io_error(&self) -> bool;

    fn is_not_supported_error(&self) -> bool;

    fn is_invalid_argument(&self) -> bool;

    /// LevelError 转 String
    ///
    /// # Arguments
    ///
    /// returns: LevelError
    ///
    /// # Examples
    ///
    /// ```
    /// let error: LevelError = LevelError::invalid_argument(String::from("aa"), String::from("bb"));
    /// let data = error.to_string();
    /// assert_eq!("Invalid argument: aa: bb",  data);
    /// ```
    fn to_string(self) -> String;

    fn ok() -> LevelError;

    fn not_found(msg: Slice, msg2: Slice) -> LevelError;

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
    fn corruption(msg: Slice, msg2: Slice) -> LevelError;

    fn not_supported(msg: Slice, msg2: Slice) -> LevelError;
    fn invalid_argument(msg: Slice, msg2: Slice) -> LevelError;

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
    fn io_error(msg: Slice, msg2: Slice) -> LevelError;

}