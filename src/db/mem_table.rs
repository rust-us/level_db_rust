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
use crate::util::unsafe_slice::UnsafeSlice;

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
    pub fn add<R: AsRef<[u8]>>(&mut self, seq_no: usize, v_type: ValueType, key: &R, value: &R) -> Result<()> {
        let key_buf = key.as_ref();
        let value_buf = value.as_ref();
        let key_size = key_buf.len();
        let value_size = value_buf.len();
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
        offset += (&mut buf[offset..]).write(key_buf)?;
        // write seq_no and type
        offset = Coding::encode_fixed64((seq_no << 8 | v_type.get_value()) as u64, buf, offset);
        // write value slice
        (&mut buf[offset..]).write(value_buf)?;
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
                let klength = entry.sub_slice(0, 5).to_slice();
                let key_length = Coding::get_varint32(&klength) as usize;
                let var_klength = Coding::varint_length(key_length);
                let user_key = entry.sub_slice(var_klength, key_length - 8);
                if self.cmp.compare(user_key.as_ref(), _key.user_key().as_ref()).unwrap().is_eq() {
                    let tag = Coding::decode_fixed64(entry.sub_slice(var_klength + key_length - 8, 8).as_ref());
                    let value_type = ValueType::try_from((tag & 0xff) as i32);
                    match value_type {
                        Ok(ValueType::KTypeValue) => return Ok(Some(self.get_length_prefixed_slice(entry.sub_slice(var_klength + key_length, entry.len() - var_klength + key_length).as_ref()))),
                        Ok(ValueType::KTypeDeletion) => return Ok(None),
                        _ => return Ok(None)
                    }
                }
                return Ok(None)
            }
        }
        return Ok(None)
    }

    fn get_length_prefixed_slice(&self, data: &[u8]) -> Slice {
        let vlength = Slice::from_buf(&data[..5]);
        let key_length = Coding::get_varint32(&vlength) as usize;
        Slice::from_buf(&data[key_length..])
    }

}

mod test {
    #[test]
    fn test() {

    }
}