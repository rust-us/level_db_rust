use crate::util::slice::Slice;

use crate::util::Result;

pub struct FilterBlockBuilder {}

impl FilterBlockBuilder {
    /// 设置block的起始位置
    ///
    /// # Arguments
    ///
    /// * `_block_offset`: 偏移量
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// filter_block_builder.start_block(1024_u64);
    /// ```
    pub fn start_block(&mut self, _block_offset: u64) {
        todo!()
    }

    /// 添加key到builder
    ///
    /// # Arguments
    ///
    /// * `_key`: 键
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn add_key(&mut self, _key: &Slice) {
        todo!()
    }
    /// 构造filterBlock
    ///
    /// # Examples
    ///
    /// ```
    /// filter_block_builder.finish();
    /// ```
    pub fn finish(&mut self) -> Result<Slice> {
        todo!()
    }
}

pub struct FilterBlockReader {}

impl FilterBlockReader {
    pub fn key_may_match(&self, _block_offset: u64, _key: &Slice) -> bool {
        todo!()
    }
}