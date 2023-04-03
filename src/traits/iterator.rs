use crate::util::slice::Slice;
use crate::util::status::Status;
use crate::util::unsafe_slice::UnsafeSlice;

///
/// Iterator 迭代器定义
pub trait DataIterator {
    /// 检查当前位置是否有效
    ///
    /// # Arguments
    ///
    ///
    /// returns: bool
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn valid(&self) -> bool;
    /// 将迭代器定位到开始位置
    ///
    /// # Arguments
    ///
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn seek_to_first(&mut self);
    /// 将迭代器定位到末尾位置
    ///
    /// # Arguments
    ///
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn seek_to_last(&mut self);
    /// 将迭代器定位到指定位置
    ///
    /// # Arguments
    ///
    /// * `target`: 迭代器位置
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn seek(&mut self, target: &Slice);
    /// 定位到下一个元素
    ///
    /// # Arguments
    ///
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn next(&mut self);

    /// 定位到上一个元素
    ///
    /// # Arguments
    ///
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn pre(&mut self);

    /// 获取key值
    ///
    /// # Arguments
    ///
    ///
    /// returns: Slice
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    /// todo UnsafeSlice 与 Slice 应该存在一个共同traits或者struct 便于API操作
    fn key(&self) -> UnsafeSlice;

    /// 获取value值
    ///
    /// # Arguments
    ///
    ///
    /// returns: Slice
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn value(&self) -> UnsafeSlice;

    fn status(&self) -> Status;

}
