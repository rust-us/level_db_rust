use std::io::Write;
use std::sync::{Arc, Mutex};
use crate::db::db_format::{LookupKey, ValueType};
use crate::db::skip_list::SkipList;
use crate::traits::comparator_trait::Comparator;
use crate::traits::DataIterator;
use crate::util::arena::ArenaRef;
use crate::util::slice::Slice;
use crate::util::{Arena, Result};
use crate::util::coding::{Encoder, varint_length};
use crate::util::unsafe_slice::UnsafeSlice;

/// 内存表
pub struct MemTable<Cmp: Comparator> {
    cmp: Arc<Cmp>,
    list: SkipList<Cmp>,
    arena: ArenaRef,
}

impl<Cmp: Comparator> MemTable<Cmp> {
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
            arena,
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
    pub fn add<R: AsRef<[u8]>>(&mut self, seq_no: usize, v_type: ValueType, key: &R, value: &R) -> Result<()> {
        let key_buf = key.as_ref();
        let value_buf = value.as_ref();
        let key_size = key_buf.len();
        let value_size = value_buf.len();
        let internal_key_size = key_size + 8;
        let encoded_len = varint_length(key_size as u64)
            + internal_key_size
            + varint_length(value_size as u64)
            + value_size;
        let mut lock = self.arena.lock()?;
        let buf = lock.allocate(encoded_len);
        let mut encoder = Encoder::with_buf(buf);
        // 需要保证写入的数据不会超过buf.len(), 否则溢出
        unsafe {
            // write key size
            encoder.uncheck_put_varint32(internal_key_size as u32);
            // write key slice
            encoder.uncheck_put_buf(key_buf);
            // write seq_no and type
            encoder.uncheck_put_fixed64((seq_no << 8 | v_type.get_value()) as u64);
            // write value slice
            encoder.uncheck_put_buf(value_buf);
        }
        // let slice = Slice::from_buf(buf);
        self.list.insert(UnsafeSlice::new_with_arena(buf, self.arena.clone())?)
    }

    /// 通过 key 查找结果
    pub fn get(&self, _key: &LookupKey) -> Result<Option<Slice>> {
        todo!()
    }
}

mod test {
    #[test]
    fn test() {}
}