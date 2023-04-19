use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut, Shr};
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard};
use std::{io, result, thread};
use std::any::Any;
use std::str::FromStr;
use std::sync::atomic::AtomicUsize;
use custom_proc_macro::arr;
use crate::util::hash::{Hash, ToHash};
use crate::util::linked_list::LinkedList;
use crate::util::slice::Slice;

use crate::util::Result;

// 缓存的对象, 以Handle为单位进行数据传递和共享, 其中的value是只读的, 带有读写锁
#[derive(Debug)]
pub struct LRUHandle<T> {
    // 缓存的键, 当hash出现冲突时判断key是否相等
    key: Slice,
    // 缓存的数据, 只读
    value: T,
    // key的hash值, 用于在HandleTable中寻址
    hash: u32,
    // 是否在缓存中
    in_cache: bool,
    // key的长度
    key_length: usize,
    // value的长度或者大小
    charge: usize,
    // 上一节点
    prev: Option<Arc<RwLock<LRUHandle<T>>>>,
    // 下一节点
    next: Option<Arc<RwLock<LRUHandle<T>>>>,
    // 下一lru节点
    next_lru: Option<Arc<RwLock<LRUHandle<T>>>>,
}

impl<T> LRUHandle<T> {
    fn new(key: Slice,
           value: T,
           hash: u32,
           charge: usize,
    ) -> Self {
        let key_length = key.size();
        Self {
            key,
            value,
            hash,
            in_cache: false,
            key_length,
            charge,
            prev: None,
            next: None,
            next_lru: None,
        }
    }
    pub fn key(&self) -> &Slice {
        &self.key
    }
    pub fn value(&self) -> &T {
        &*self
    }
}

impl<T> Deref for LRUHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for LRUHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[derive(Debug)]
struct HandleTable<T> {
    length: usize,
    list: Vec<Option<Arc<RwLock<LRUHandle<T>>>>>,
}

impl<T> Default for HandleTable<T> {
    fn default() -> Self {
        HandleTable {
            length: 16,
            list: vec![None; 16],
        }
    }
}

impl<T> HandleTable<T> {
    pub fn look_up(&self, key: &Slice, hash: u32) -> Result<Option<Arc<RwLock<LRUHandle<T>>>>> {
        // 获取hash槽位上的数据, 存在则遍历链表
        let index = hash as usize & self.length.wrapping_sub(1);
        let mut head = self.list[index].clone();
        while let Some(handle) = head {
            let read = handle.read()?;
            if &read.key == key {
                return Ok(Some(handle.clone()));
            }
            head = read.next.clone();
        }
        Ok(None)
    }

    pub fn insert(&mut self, handle: LRUHandle<T>) -> Result<()> {
        let index = handle.hash as usize & (self.length - 1);
        // 获取hash槽位上的数据, 不存在直接插入, 存在插入尾部
        match self.list[index].clone() {
            Some(mut head) => {
                while let Some(value) = head.clone().write()?.next.clone() {
                    head = value;
                }
                head.clone().write()?.next = Some(Arc::new(RwLock::new(handle)));
            }
            None => {
                self.list[index] = Some(Arc::new(RwLock::new(handle)));
            }
        }
        Ok(())
    }

    pub fn remove(&mut self, key: &Slice, hash: u32) -> Result<()> {
        let index = hash as usize & self.length.wrapping_sub(1);
        let mut head = self.list[index].clone();
        // 获取hash槽位上的数据, 遍历到key相等时删除handle
        while let Some(handle) = head {
            let write = handle.write()?;
            if &write.key == key {
                if write.prev.is_none() && write.next.is_none() {
                    // 只有一个节点直接置空
                    self.list[index] = None;
                } else if write.prev.is_none() {
                    // 头节点移交至下一节点
                    self.list[index] = write.next.clone();
                } else {
                    // 其余中间节点或尾节点, 删除当前节点并将下一节点移交给上一节点
                    write.prev.clone().unwrap().write()?.next = write.next.clone()
                }
            }
            head = write.next.clone();
        }
        Ok(())
    }

    pub fn length(&self) -> usize {
        self.length
    }

    fn resize(&mut self) {
        todo!()
    }
}

#[derive(Debug)]
struct LRUCache<T> {
    capacity: usize,
    usage: usize,
    in_use: Option<LRUHandle<T>>,
    table: HandleTable<T>,
}

impl<T> Default for LRUCache<T> {
    fn default() -> Self {
        Self {
            capacity: 0,
            usage: 0,
            in_use: None,
            table: HandleTable::default(),
        }
    }
}

impl<T> LRUCache<T> {
    pub fn new(capacity: usize, usage: usize, in_use: Option<LRUHandle<T>>, table: HandleTable<T>) -> Self {
        Self { capacity, usage, in_use, table }
    }

    pub fn insert(&mut self, key: Slice, hash: u32, value: T, charge: usize) -> Result<()> {
        let e = LRUHandle::new(key,
                               value,
                               hash,
                               charge,
        );
        self.table.insert(e)?;
        self.usage += 1;
        self.capacity += 1;
        Ok(())
    }

    pub fn look_up(&self, key: &Slice, hash: u32) -> Result<Option<Arc<RwLock<LRUHandle<T>>>>> {
        self.table.look_up(key, hash)
    }

    pub fn erase(&mut self, key: &Slice, hash: u32) -> Result<()> {
        self.table.remove(key, hash)?;
        self.capacity += 1;
        Ok(())
    }
    pub fn prune(&mut self) -> Result<()> {
        Ok(())
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
}

const K_NUM_SHARD_BITS: usize = 4;
const K_NUM_SHARDS: usize = 1 << K_NUM_SHARD_BITS;

#[derive(Debug)]
pub struct ShardLRUCache<T> {
    shard: [LRUCache<T>; 16],
    // 封闭构造器, 请使用ShardLRUCache::new()进行构造, 请勿自行构造结构体
    __private: (),
}

#[inline]
fn hash_slice(slice: &Slice) -> u32 {
    Hash::hash_code(slice, 0)
}

#[inline]
fn shard(hash: u32) -> usize {
    (hash >> (32 - K_NUM_SHARD_BITS)) as usize
}

#[inline]
fn pre_shard(capacity: usize) -> usize {
    (capacity + (K_NUM_SHARDS - 1)) / K_NUM_SHARDS
}

unsafe impl<T> Send for ShardLRUCache<T> {}

unsafe impl<T> Sync for ShardLRUCache<T> {}

/// shard的实现可以降低锁粒度, 提高并发度
impl<T> ShardLRUCache<T> {
    pub fn new() -> ShardLRUCache<T> {
        Self {
            shard: arr!([LRUCache::default(); 16]),
            __private: (),
        }
    }

    pub fn new_with_arc() -> Arc<RwLock<ShardLRUCache<T>>> {
        Arc::new(RwLock::new(ShardLRUCache {
            shard: arr!([LRUCache::default(); 16]),
            __private: (),
        }))
    }

    pub fn insert(&mut self, key: &Slice, value: T, charge: usize) -> Result<()> {
        let hash = hash_slice(key);
        self.shard[shard(hash)].insert(key.clone(), hash, value, charge)
    }
    pub fn lookup(&self, key: &Slice) -> Result<Option<Arc<RwLock<LRUHandle<T>>>>> {
        let hash = hash_slice(key);
        self.shard[shard(hash)].look_up(key, hash)
    }
    pub fn erase(&mut self, key: &Slice) -> Result<()> {
        // 删除缓存
        let hash = hash_slice(key);
        self.shard[shard(hash)].erase(key, hash)
    }
    pub fn prune(&mut self) -> Result<()> {
        // 清空全部shard的缓存
        for mut shard in &mut self.shard {
            shard.prune()?
        }
        Ok(())
    }
}

#[test]
fn test_insert_cache() -> Result<()> {
    let mut cache = ShardLRUCache::new();
    let key = Slice::from("test_key");
    cache.insert(&key, 10, 4)?;
    println!("{:?}", cache);
    let handle = cache.lookup(&key)?;
    println!("{:?}", handle);
    assert_eq!(true, handle.is_some());
    assert_eq!(&10, handle.unwrap().read()?.value());

    Ok(())
}

#[test]
fn test_insert_cache_multi_thread() -> Result<()> {
    let mut cache = ShardLRUCache::new_with_arc();

    let mut thread_vec = vec![];
    let thread_count = 128;
    // 创建5线程写入缓存
    for i in 0..thread_count {
        let share_cache = cache.clone();
        let thread = thread::spawn(move || -> Result<()>{
            let key = Slice::from("test_key".to_string() + &i.to_string());
            share_cache.write()?.insert(&key, i, 4)?;

            println!("write thread {}, write value: {}", i, i);
            Ok(())
        });
        thread_vec.push(thread);
    }

    for thread in thread_vec {
        thread.join().unwrap()?;
    }

    let mut thread_vec = vec![];

    // 创建5线程读取缓存
    for i in 0..thread_count {
        let share_cache = cache.clone();
        let thread = thread::spawn(move || -> Result<()>{
            let key = Slice::from("test_key".to_string() + &i.to_string());
            let read = share_cache.read()?.lookup(&key)?;
            println!("read thread {}, read value: {}", i, read.clone().unwrap().read()?.value);
            assert_eq!(true, read.is_some());
            assert_eq!(i, read.clone().unwrap().read()?.value);
            Ok(())
        });
        thread_vec.push(thread);
    }

    for thread in thread_vec {
        thread.join().unwrap()?;
    }

    // 线程全部执行完打印缓存信息
    println!("{:?}", cache);

    Ok(())
}

#[test]
fn test_erase_cache() -> Result<()> {
    let mut cache = ShardLRUCache::new();
    let key = Slice::from("test_key");
    cache.insert(&key, 10, 4)?;
    println!("{:?}", cache);
    cache.erase(&key)?;
    println!("{:?}", cache);
    let handle = cache.lookup(&key)?;
    println!("{:?}", handle);
    assert_eq!(true, handle.is_none());

    Ok(())
}

#[test]
fn test_clear_cache() -> Result<()> {
    todo!()
}