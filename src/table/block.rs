use std::rc::Rc;
use crate::traits::comparator_trait::Comparator;
use crate::traits::DataIterator;

use crate::util::Result;

pub struct Block {}

impl Block {
    /// 获取block的大小
    ///
    /// # Examples
    ///
    /// ```
    /// let block = Block {};
    /// let size = block.size();
    /// ```
    pub fn size(&self) {
        todo!()
    }
    /// 生成迭代器
    ///
    /// # Arguments
    ///
    /// * `_comparator`: 比较器
    ///
    /// returns: Result<Box<dyn DataIterator, Global>, Status>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn new_iterator(&self, _comparator: Rc<impl Comparator>) -> Result<Box<dyn DataIterator>> {
        todo!()
    }
}