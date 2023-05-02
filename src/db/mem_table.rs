use std::io::Write;
use std::sync::{Arc, Mutex};
use crate::db::db_format::{LookupKey, ValueType};
use crate::db::skip_list::SkipList;
use crate::traits::comparator_trait::Comparator;
use crate::traits::DataIterator;
use crate::util::arena::ArenaRef;
use crate::util::slice::Slice;
use crate::util::{Arena, Result};
use crate::util::coding::{Decoder, Encoder, varint_length};
use crate::util::status::{LevelError, Status};
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
        let memkey = _key.mem_table_key();
        let mut iter = self.list.iter();
        iter.seek(&memkey); // seek需要完成
        if iter.valid() {
            // entry format is:
            //    klength  varint32
            //    userkey  char[klength]
            //    tag      uint64
            //    vlength  varint32
            //    value    char[vlength]
            // Check that it belongs to same user key.  We do not check the
            // sequence number since the Seek() call above should have skipped
            // all entries with overly large sequence numbers.
            let entry = iter.key();
            unsafe {
                let slice = entry.to_slice();
                let mut decoder = Decoder::with_slice(&slice);
                let klength = decoder.uncheck_get_varint32();
                let user_key = decoder.get_buf((klength - 8) as usize);
                let tag = decoder.get_fixed64();
                return self.cmp.compare(user_key.unwrap().as_ref(), _key.user_key().as_ref())
                    .filter(|x| x.is_eq())
                    .map(|_| ValueType::try_from((tag.unwrap() & 0xff) as i32))
                    .map_or(Ok(None), |x| {
                        match x {
                            Ok(v) => Ok(Some(v)),
                            Err(e) => Err(Status::wrapper_str(LevelError::KCorruption, &e))
                        }
                    })
                    .map(|vt| {
                        vt.map(|v| {
                            match v {
                               ValueType::KTypeValue => {
                                   let vlength = decoder.get_varint32();
                                   Some(decoder.get_buf(vlength.unwrap() as usize).unwrap())
                               },
                                ValueType::KTypeDeletion => None
                            }
                        }).unwrap_or(None)
                    }).or(Ok(None));
            }
        }
        return Ok(None)
    }
}

mod test {
    #[test]
    fn test() {}
}