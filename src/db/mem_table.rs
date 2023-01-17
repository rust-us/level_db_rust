use std::rc::Rc;
use crate::traits::comparator_trait::Comparator;
use crate::traits::DataIterator;
use crate::util::comparator::InternalKeyComparator;
use crate::util::slice::Slice;

use crate::util::Result;

pub enum ValueType {
    Insert,
    Deletion,
}

/// 内存表
pub struct MemTable<Cmp: Comparator> {
    cmp: Rc<Cmp>,
}

/// 临时, 查找键
struct LookupKey {}

impl <Cmp: Comparator> MemTable<Cmp> {

    /// 创建内存表
    ///
    /// # Arguments
    ///
    /// * `_comparator`: 比较器
    ///
    /// returns: MemTable
    ///
    /// # Examples
    ///
    /// ```
    /// let mt = MemTable::create(cmp);
    /// ```
    pub fn create(cmp: Rc<Cmp>) -> Self {
        Self {
            cmp,
        }
    }

    /// 返回该表使用的内存近似值
    pub fn approximate_memory_usage(&self) -> usize {
        todo!()
    }

    /// 创建内存表迭代器
    ///
    /// # Arguments
    ///
    /// returns: MemTable
    ///
    /// # Examples
    ///
    /// ```
    /// let mem = MemTable::create(comp);
    /// let it = mem::new_new_iterator()?;
    /// ```
    pub fn new_iterator(&self) -> Result<Box<dyn DataIterator>> {
        todo!()
    }

    /// 像内存表中写入或删除一个元素
    pub fn add(&mut self, _seq_no: usize, _v_type: ValueType, _key: &Slice, _value: Slice) -> Result<()> {
        todo!()
    }

    /// 通过 key 查找结果
    pub fn get(&self, _key: &LookupKey) -> Result<Option<Slice>> {
        todo!()
    }

}

mod test {
    #[test]
    fn test() {

    }
}