use std::io::Write;
use std::sync::{Arc, Mutex};
use crate::db::db_format::{LookupKey, ValueType};
use crate::db::skip_list::SkipList;
use crate::traits::coding_trait::CodingTrait;
use crate::traits::comparator_trait::Comparator;
use crate::traits::DataIterator;
use crate::util::arena::ArenaRef;
use crate::util::slice::Slice;
use crate::util::{Arena, Result};
use crate::util::coding::Coding;

/// 内存表
pub struct MemTable<Cmp: Comparator> {
    cmp: Arc<Cmp>,
    list: SkipList<Cmp>,
    arena: ArenaRef,
}

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
    pub fn create(cmp: Arc<Cmp>) -> Self {
        let arena = Arc::new(Mutex::new(Arena::default()));
        let list = SkipList::create(cmp.clone(), arena.clone());
        Self {
            cmp,
            list,
            arena
        }
    }

    /// 返回该表使用的内存近似值
    #[inline]
    pub fn approximate_memory_usage(&self) -> usize {
        self.arena.lock().unwrap().memory_usage()
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
    /// let it = mem.new_new_iterator()?;
    /// ```
    pub fn new_iterator(&self) -> Result<Box<dyn DataIterator>> {
        todo!()
    }

    /// 像内存表中写入或删除一个元素
    pub fn add(&mut self, seq_no: usize, v_type: ValueType, key: &Slice, value: Slice) -> Result<()> {
        let key_size = key.size();
        let value_size = value.size();
        let internal_key_size = key_size + 8;
        let encoded_len = Coding::varint_length(key_size)
            + internal_key_size
            + Coding::varint_length(value_size)
            + value_size;
        let mut lock = self.arena.lock()?;
        let buf = lock.allocate(encoded_len);
        let mut offset = 0;
        // write key size
        offset = Coding::encode_varint32(internal_key_size as u32, buf, offset);
        // write key slice
        offset += (&mut buf[offset..]).write(key.as_ref())?;
        // write seq_no and type
        offset = Coding::encode_fixed64((seq_no << 8 | v_type.get_value()) as u64, buf, offset);
        // write value slice
        (&mut buf[offset..]).write(value.as_ref())?;
        let slice = Slice::from_buf(buf);
        self.list.insert(slice)
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