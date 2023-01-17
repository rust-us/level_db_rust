use crate::util::slice::Slice;

use crate::util::Result;

pub struct BlockBuilder {}

impl BlockBuilder {
    /// 添加数据到block
    ///
    /// # Arguments
    ///
    /// * `key`: 键
    /// * `value`: 值
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn add(&self, _key: &Slice, _value: &Slice) {
        todo!()
    }
    /// 重置builder
    ///
    /// # Examples
    ///
    /// ```
    /// block_builder.reset();
    /// ```
    pub fn reset(&self) {
        todo!()
    }
    /// 构造block
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// let block = block_builder.finish();
    /// ```
    pub fn finish(&self) -> Result<Slice> {
        todo!()
    }
    /// 判断builder是否为空
    ///
    /// # Examples
    ///
    /// ```
    /// let is_empty = block_builder.empty();
    /// ```
    pub fn empty(&self) -> bool {
        todo!()
    }
    /// 估算当前的block大小
    ///
    /// # Examples
    ///
    /// ```
    /// let estimate_size = block_builder.current_size_estimate();
    /// ```
    pub fn current_size_estimate(&self) -> usize {
        todo!()
    }
}