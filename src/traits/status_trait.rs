use crate::util::slice::Slice;
use crate::util::status::LevelError;

/// db 中的返回状态，将错误号和错误信息封装成Status类，统一进行处理。
/// 在 leveldb的实现里， 为了节省空间Status将返回码(code), 错误信息message及长度打包存储于一个字符串数组中， 来存储错误信息。
/// 在该项目中， 使用LevelError 和 Slice 存储错误信息
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