use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::ops::{Deref, Shr};
use std::rc::Rc;
use crate::util::hash::ToHash;
use crate::util::linked_list::LinkedList;
use crate::util::slice::Slice;

use crate::util::Result;

#[derive(Clone, Debug, PartialEq)]
pub struct LRUHandle<T: Clone> {
    key: Slice,
    value: T,
    hash: u32,
    in_cache: bool,
    key_length: usize,
    charge: usize,
    refs: u32,
    prev: Option<Rc<RefCell<LRUHandle<T>>>>,
    next: Option<Rc<RefCell<LRUHandle<T>>>>,
    next_hash: Option<Rc<RefCell<LRUHandle<T>>>>,
}

impl<T: Clone> LRUHandle<T> {
    fn new(key: Slice,
           value: T,
           hash: u32,
           charge: usize,
           prev: Option<Rc<RefCell<LRUHandle<T>>>>,
           next: Option<Rc<RefCell<LRUHandle<T>>>>,
           next_hash: Option<Rc<RefCell<LRUHandle<T>>>>) -> Self {
        let key_length = key.size();
        Self {
            key,
            value,
            hash,
            in_cache: false,
            key_length,
            charge,
            refs: 1,
            prev,
            next,
            next_hash,
        }
    }
    pub fn key(&self) -> &Slice {
        &self.key
    }
    pub fn value(&self) -> &T {
        &self.value
    }
}

#[derive(Clone)]
pub struct HandleTable<T: Clone> {
    length: usize,
    list: [Option<LRUHandle<T>>; 16],
}

impl<T: Clone> Default for HandleTable<T> {
    fn default() -> Self {
        HandleTable {
            length: 16,
            list: [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
        }
    }
}

impl<T: Clone> HandleTable<T> {
    pub fn look_up(&self, key: &Slice, hash: u32) -> Result<Option<LRUHandle<T>>> {
        match &self.list[hash as usize & self.length.wrapping_sub(1)] {
            Some(v) => {
                Ok(Some(v.clone()))
            }
            _ => {
                return Ok(None);
            }
        }
    }

    pub fn insert(&mut self, handle: LRUHandle<T>) {
        let index = handle.hash as usize & self.length.wrapping_sub(1);
        self.list[index] = Some(handle);
    }

    pub fn remove(&mut self, _key: &Slice, _hash: u32) {
        let index = _hash as usize & self.length.wrapping_sub(1);
        self.list[index] = None;
    }

    pub fn length(&self) -> usize {
        self.length
    }

    /// 扩容
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn resize(&mut self) {
        todo!()
    }
}

pub struct LRUCache<T: Clone> {
    capacity: usize,
    usage: usize,
    in_use: Option<LRUHandle<T>>,
    table: HandleTable<T>,
}

impl<T: Clone> LRUCache<T> {
    pub fn new(capacity: usize, usage: usize, in_use: Option<LRUHandle<T>>, table: HandleTable<T>) -> Self {
        Self { capacity, usage, in_use, table }
    }

    // pub fn set_capacity(&mut self, capacity: usize) {
    //     self.capacity = capacity;
    // }

    pub fn insert<F>(&mut self, key: Slice, hash: u32, value: T, charge: usize, deleter: F)
        where F: FnOnce(Slice, T) {
        let e = LRUHandle::new(key,
                               value,
                               hash,
                               charge,
                               None,
                               None,
                               None,
        );
        self.table.insert(e);
        self.usage += 1;
    }

    pub fn look_up(&self, key: &Slice, hash: u32) -> Result<Option<LRUHandle<T>>> {
        self.table.look_up(key, hash)
    }

    pub fn release(&mut self, _handle: &LRUHandle<T>) {
        todo!()
    }

    pub fn erase(&mut self, _key: &Slice, _hash: u32) -> Result<()> {
        self.table.remove(_key, _hash);
        Ok(())
    }
    pub fn prune(&mut self) {
        todo!()
    }
    pub fn total_charge(&self) -> usize {
        todo!()
    }

    fn lru_remove(&mut self, _handle: &LRUHandle<T>) {
        todo!()
    }
    fn lru_append(&mut self, _head_of_list: &LRUHandle<T>, _e: LRUHandle<T>) {
        todo!()
    }
    fn refer(&self, _e: &LRUHandle<T>) {
        todo!()
    }
    fn unref(&self, _e: &LRUHandle<T>) {
        todo!()
    }
}

const K_NUM_SHARD_BITS: usize = 4;
const K_NUM_SHARDS: usize = 1 << K_NUM_SHARD_BITS;

pub struct ShardLRUCache<T: Clone + 'static> {
    shard: Vec<LRUCache<T>>,
}

impl<T: Clone> ShardLRUCache<T> {
    /// 构造一个指定容量的ShardLRUCache
    ///
    /// # Arguments
    ///
    /// * `capacity`: 容量
    ///
    /// returns: ShardLRUCache<T>
    ///
    /// # Examples
    ///
    /// ```
    /// ShardLRUCache::new_with_capacity(32);
    /// ```
    pub fn new_with_capacity(capacity: usize) -> Self {
        let per_shard: usize = (capacity + (K_NUM_SHARDS - 1)) / K_NUM_SHARD_BITS;

        let mut shard_vec: Vec<LRUCache<T>> = Vec::with_capacity(K_NUM_SHARDS);
        for _ in 1..K_NUM_SHARDS {
            let table = HandleTable::default();
            let cache: LRUCache<T> = LRUCache::new(per_shard, 0, None, table);
            shard_vec.push(cache);
        }
        Self {
            shard: shard_vec
        }
    }

    fn hash_slice(s: &Slice) -> u32 {
        s.to_hash_with_seed(0)
    }

    fn shard(hash: u32) -> u32 {
        hash.shr(32 - K_NUM_SHARD_BITS)
    }

    /// 从缓存中获取数据
    ///
    /// # Arguments
    ///
    /// * `key`: 键
    ///
    /// returns: Result<LRUHandle<T>, Status>
    ///
    /// # Examples
    ///
    /// ```
    /// let value= cache.lookup(Slice::from("123"));
    /// ```
    pub fn lookup(&self, key: &Slice) -> Result<Option<LRUHandle<T>>> {
        let hash = Self::hash_slice(&key);
        let i = Self::shard(hash);
        self.shard[i as usize].look_up(key, hash)
    }

    /// 插入数据到缓存
    ///
    /// # Arguments
    ///
    /// * `key`: 键
    /// * `value`: 值
    /// * `charge`: 空间占用量
    /// * `deleter`: 删除的回调函数
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// cache.insert(Slice::from("123", 123,1,move || {}))
    /// ```
    pub fn insert<F>(&mut self, key: Slice, value: T, charge: usize, deleter: F) -> Result<()>
        where F: FnOnce(Slice, T) {
        let hash = Self::hash_slice(&key);
        let i = Self::shard(hash);
        let mut shard = &mut self.shard[i as usize];
        shard.insert(key, hash, value, charge, deleter);
        Ok(())
    }

    /// 释放引用
    /// 当数据不再需要使用时, 使用方必须释放引用
    ///
    /// # Arguments
    ///
    /// * `handle`: 需要释放的值
    ///
    /// returns: Result<(), Status>
    ///
    /// # Examples
    ///
    /// ```
    /// cache.release(handle);
    /// ```
    pub fn release(&mut self, handle: LRUHandle<T>) -> Result<()> {
        todo!()
    }

    /// 从缓存中删除值
    ///
    /// # Arguments
    ///
    /// * `key`: 值
    ///
    /// returns: Result<(), Status>
    ///
    /// # Examples
    ///
    /// ```
    /// cache.erase(Slice::from("123"));
    /// ```
    pub fn erase(&mut self, key: &Slice) -> Result<()> {
        let hash = Self::hash_slice(&key);
        let i = Self::shard(hash);
        let mut shard = &mut self.shard[i as usize];
        shard.erase(key, hash)
    }
}