use std::rc::Rc;
use crate::traits::comparator_trait::ComparatorTrait;
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
    ///
    ///
    /// # Arguments
    ///
    /// * `_comparator`:
    ///
    /// returns: Result<Box<dyn DataIterator, Global>, Status>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn new_iterator(&self, _comparator: Rc<impl ComparatorTrait>) -> Result<Box<dyn DataIterator>> {
        todo!()
    }
}