use std::{ptr, thread, usize};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};

use custom_proc_macro::{arr, non_null_new_uncheck};

use crate::util::hash::Hash;
use crate::util::Result;
use crate::util::slice::Slice;

/// handle类型定义
type HandleRef<T> = NonNull<LRUHandle<T>>;

// 缓存的对象, 以Handle为单位进行数据传递和共享, 其中的value是只读的, 带有读写锁
#[derive(Debug)]
pub struct LRUHandle<T> {
    // 缓存的键, 当hash出现冲突时判断key是否相等
    key: Slice,
    // 缓存的数据, 只读
    value: Arc<T>,
    // key的hash值, 用于在HandleTable中寻址
    hash: u32,
    // 是否在缓存中
    in_cache: bool,
    // key的长度
    key_length: usize,
    // value的长度或者数据量的大小, 用于统计当前缓存了多少数据量
    charge: usize,
    // 上一节点(lruCache中的双向链表的上一节点)
    prev: Option<HandleRef<T>>,
    // 下一节点(lruCache中的双向链表的下一节点)
    next: Option<HandleRef<T>>,
    // 上一节点(handleTable中的双向链表的上一节点)
    prev_hash: Option<HandleRef<T>>,
    // 下一节点(handleTable中的双向链表的下一节点)
    next_hash: Option<HandleRef<T>>,
}

impl<T> LRUHandle<T> {
    /// 从栈上分配内存
    fn new(key: Slice,
           value: T,
           hash: u32,
           charge: usize,
    ) -> Self {
        let key_length = key.size();
        Self {
            key,
            value: Arc::new(value),
            hash,
            in_cache: true,
            key_length,
            charge,
            prev: None,
            next: None,
            prev_hash: None,
            next_hash: None,
        }
    }
    /// 从堆上分配内存
    ///
    /// # Arguments
    ///
    /// * `key`: 键
    /// * `value`: 值
    /// * `hash`: 键的hash
    /// * `charge`: 值的长度或者数据大小
    ///
    /// returns: HandleRef<T>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn new_on_heap(key: Slice, value: T, hash: u32, charge: usize) -> HandleRef<T> {
        let key_length = key.size();
        // 在堆上分配 LRUHandle<T> 使用的内存
        let data = Box::new(Self {
            key,
            value: Arc::new(value),
            hash,
            in_cache: true,
            key_length,
            charge,
            prev: None,
            next: None,
            prev_hash: None,
            next_hash: None,
        });
        // 不检查是否为空指针 异常情况可能会导致程序崩溃
        // 转为裸指针后这块内存不会被自动回收
        non_null_new_uncheck!(Box::into_raw(data))
    }
    /// 返回handle的键
    pub fn key(&self) -> &Slice {
        &self.key
    }
    /// 返回handle的值
    pub fn value(&self) -> Arc<T> {
        self.value.clone()
    }
}

impl<T> Deref for LRUHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // 解引用为value
        &self.value
    }
}

/// hash表
/// 当写入达到阈值后会进行扩容, 可以传入default_length避免扩容
struct HandleTable<T> {
    // hash表中已写入的数据量
    elements: usize,
    // hash表默认大小, prune时会恢复到这个长度
    default_length: usize,
    // hash表的大小
    length: usize,
    // hash表的table, 堆上分配数组
    list: Vec<Option<HandleRef<T>>>,
    // shard号, 用于debug
    _shard: usize,
    // 标识LRUHandle属于HandleTable, 编译器会检查LRUHandle的生命周期小于HandleTable的生命周期
    _marker: PhantomData<*mut LRUHandle<T>>,
}

/// 格式化长度, 返回2的次幂
fn format_length(length: usize) -> usize {
    // 最小长度是DEFAULT_HASH_TABLE_LENGTH
    if length <= DEFAULT_HASH_TABLE_LENGTH {
        return DEFAULT_HASH_TABLE_LENGTH;
    }
    let mut shift = 0;
    while length > 1 << shift {
        shift += 1;
        if 1_usize.checked_shl(shift).is_none() {
            // 如果发生了溢出, 返回不溢出的最大值
            return 1 << (shift - 1);
        }
    }
    1 << shift
}

impl<T> HandleTable<T> {
    fn new(shard: usize) -> Self {
        Self::new_with_length(shard, DEFAULT_HASH_TABLE_LENGTH)
    }

    fn new_with_length(shard: usize, default_length: usize) -> Self <> {
        // 格式化用户输出的长度为2的次幂
        let length = format_length(default_length);
        Self {
            elements: 0,
            default_length: length,
            length,
            list: vec![None; length],
            _shard: shard,
            _marker: PhantomData::default(),
        }
    }

    /// 从hash表中查询数据
    pub fn look_up(&self, key: &Slice, hash: u32) -> Result<Option<HandleRef<T>>> {
        let index = self.find_index(hash);
        // 获取hash槽位上的数据, 存在则遍历链表
        let mut head = self.list[index];
        while let Some(handle) = head {
            let handle_ref = unsafe { handle.as_ref() };
            if &handle_ref.key == key {
                return Ok(Some(handle));
            }
            head = handle_ref.next_hash;
        }
        Ok(None)
    }

    /// 向hash表中插入数据
    pub fn insert(&mut self, mut handle: HandleRef<T>) -> Result<()> {
        let handle_mut = unsafe { handle.as_mut() };
        let index = self.find_index(handle_mut.hash);
        // 获取hash槽位上的头节点
        match self.list[index] {
            Some(mut head) => {
                let head_mut = unsafe { head.as_mut() };
                // 头插法插入数据
                self.list[index] = Some(handle);
                handle_mut.next_hash = Some(head);
                head_mut.prev_hash = Some(handle);
            }
            None => {
                self.list[index] = Some(handle);
            }
        }
        self.elements += 1;
        self.should_resize()?;
        Ok(())
    }

    /// 从hash表中删除数据, 并回收内存
    pub fn remove(&mut self, key: &Slice, hash: u32) -> Result<Option<HandleRef<T>>> {
        let index = self.find_index(hash);
        let mut head = self.list[index];
        // 获取hash槽位上的数据, 遍历到key相等时删除handle
        while let Some(mut handle) = head {
            let handle_mut = unsafe { handle.as_mut() };
            // key相等进行删除, 这里只断开链表的连接, 内存在lru链表上回收
            if &handle_mut.key == key {
                if handle_mut.prev_hash.is_none() && handle_mut.next_hash.is_none() {
                    // 只有一个节点, 直接置空
                    self.list[index] = None;
                } else if handle_mut.prev_hash.is_none() {
                    // 是头节点, 将头节点移交至下一节点
                    self.list[index] = handle_mut.next_hash;
                    // 下一节点的prev_hash要置空
                    handle_mut.prev_hash = None;
                } else {
                    // 是其余中间节点或尾节点, 删除当前节点并将下一节点移交给上一节点
                    let prev_hash_ptr = unsafe { handle_mut.prev_hash.unwrap().as_mut() };
                    prev_hash_ptr.next_hash = handle_mut.next_hash;
                    // 下一结点不为空时, 将当前节点的prev移交给下一节点的prev
                    if let Some(mut next_hash) = handle_mut.next_hash {
                        let next_hash_ptr = unsafe { next_hash.as_mut() };
                        next_hash_ptr.prev_hash = handle_mut.prev_hash;
                    }
                }
                // 回收内存
                Self::drop_handle(handle.as_ptr());
                self.elements -= 1;
                return Ok(Some(handle));
            }
            head = handle_mut.next_hash;
        }
        Ok(None)
    }


    /// 清空hash表 并回收内存
    pub fn prune(&mut self) {
        for handle in self.list.iter().filter(|v| v.is_some()) {
            // 回收内存
            Self::drop_handle(handle.unwrap().as_ptr());
        }
        // 清空list恢复内存
        self.list.clear();
        self.elements = 0;
        // 恢复到初始的默认容量
        self.list.resize(self.default_length, None);
        self.length = self.default_length;
    }

    /// 获取hash表的长度
    #[inline]
    #[allow(dead_code)]
    pub fn length(&self) -> usize {
        self.length
    }

    /// 是否需要扩容
    /// 需要扩容时调用扩容方法
    #[inline]
    fn should_resize(&mut self) -> Result<()> {
        // 负载因子需要平衡寻址速度与内存占用, 如果扩容后将溢出, 则不扩容
        if (self.elements as f32 > self.list.len() as f32 * LOAD_FACTOR) && self.list.len().checked_shl(1).is_some() {
            self.resize()?
        }
        Ok(())
    }

    /// 获取hash槽位
    #[inline]
    fn find_index(&self, hash: u32) -> usize {
        hash as usize & self.length.wrapping_sub(1)
    }

    /// hash表扩容
    /// 扩容操作较少使用, 标记为cold
    #[cold]
    fn resize(&mut self) -> Result<()> {
        let old_len = self.list.len();
        let new_len = self.list.len() << 1;
        self.list.resize(new_len, None);
        self.length = new_len;
        let list = &mut self.list;
        let list_ptr = list.as_mut_ptr();
        // 遍历原hash表
        for (index, handle_option) in list[0..old_len].iter_mut().enumerate() {
            if handle_option.is_none() {
                // 为空的直接跳过
                continue;
            }
            let mut current_option = *handle_option;
            let (mut low_head, mut low_tail) = (None, None);
            let (mut high_head, mut high_tail) = (None, None);
            while let Some(mut current) = current_option {
                let current_mut = unsafe { current.as_mut() };
                let next = current_mut.next_hash;
                // 与原来的容量进行与运算, 可能落在原位置 或者 原位置 + old_len
                if current_mut.hash as usize & old_len == 0 {
                    // 低位
                    if low_head.is_none() {
                        low_head = current_option;
                        low_tail = current_option;
                    } else {
                        // 头插法
                        current_mut.next_hash = low_head;
                        unsafe { low_head.unwrap().as_mut().prev_hash = current_option };
                        low_head = current_option;
                    }
                } else {
                    // 高位
                    if high_head.is_none() {
                        high_head = current_option;
                        high_tail = current_option;
                    } else {
                        // 头插法
                        current_mut.next_hash = high_head;
                        unsafe { high_head.unwrap().as_mut().prev_hash = current_option };
                        high_head = current_option;
                    }
                }
                current_option = next;
            }
            if low_head.is_some() {
                unsafe {
                    // 头节点的prev_hash需要置空
                    low_head.unwrap().as_mut().prev_hash = None;
                    // 尾节点的next_hash需要置空
                    low_tail.unwrap().as_mut().next_hash = None;
                }
            }
            unsafe { ptr::write(list_ptr.add(index), low_head); }
            if high_head.is_some() {
                unsafe {
                    // 头节点的prev_hash需要置空
                    high_head.unwrap().as_mut().prev_hash = None;
                    // 尾节点的next_hash需要置空
                    high_tail.unwrap().as_mut().next_hash = None;
                }
            }
            unsafe { ptr::write(list_ptr.add(old_len + index), high_head); }
        }
        Ok(())
    }

    /// 将裸指针包装回Box并回收
    /// 只能在hash表删除后回收内存, 在其他位置回收内存可能会double free, 或其他未定义行为
    #[inline]
    fn drop_handle(handle_ptr: *mut LRUHandle<T>) {
        // 将指针包装回box, box会在作用域结束之后自动drop掉
        unsafe { Box::from_raw(handle_ptr) };
    }
}

struct LRUCache<T> {
    // hash表, 用于存放缓存数据
    table: HandleTable<T>,
    // cache的容量
    capacity: usize,
    // cache的当前使用量, 使用量超过容量会进行扩容
    usage: usize,
    // lru链表的头指针, 最近使用的
    head_of_lru: Option<HandleRef<T>>,
    // lru链表的尾指针, 最先被删除
    tail_of_lru: Option<HandleRef<T>>,
    // shard号, 用于debug
    _shard: usize,
}

/// 默认容量 值的总长度或者是数据总大小
const DEFAULT_CACHE_PRE_SHARD_CAPACITY: usize = (DEFAULT_CACHE_CAPACITY + (K_NUM_SHARDS - 1)) / K_NUM_SHARDS;

impl<T> LRUCache<T> {
    fn new(shard: usize) -> Self {
        Self::new_with_capacity(shard, DEFAULT_CACHE_PRE_SHARD_CAPACITY, DEFAULT_SHARD_LENGTH)
    }
    /// 创建LruCache, 使用默认table, 指定容量
    fn new_with_capacity(shard: usize, capacity: usize, default_length: usize) -> Self {
        Self::new_with_table_capacity(shard, capacity, default_length)
    }

    /// 创建LruCache, 指定table, 指定容量
    fn new_with_table_capacity(shard: usize, capacity: usize, default_length: usize) -> Self {
        Self {
            table: HandleTable::new_with_length(shard, default_length),
            capacity,
            usage: 0,
            head_of_lru: None,
            tail_of_lru: None,
            _shard: shard,
        }
    }

    /// 向lru缓存中插入数据
    /// # Arguments
    /// * `key`: 键
    /// * `hash`: 键的hash
    /// * `value`: 值
    /// * `charge`: 值的长度或数据大小
    /// returns: Result<(), Status>
    /// # Examples
    /// ```
    ///
    /// ```
    pub fn insert(&mut self, key: Slice, hash: u32, value: T, charge: usize) -> Result<()> {
        let handle = LRUHandle::new_on_heap(
            key.clone(),
            value,
            hash,
            charge);
        // hash表中插入数据
        self.table.insert(handle)?;
        // 插入lru
        self.lru_append(handle)?;
        // 使用量加上写入的value的长度或者数据大小
        self.usage += charge;

        // 使用量已经达到容量, 那么删除最少使用的
        if self.usage >= self.capacity {
            if let Some(tail) = self.tail_of_lru {
                let tail_ref = unsafe { tail.as_ref() };
                // 先删除lru链表尾
                self.lru_remove(tail)?;
                // 于从hash表中删除链表尾, 同时回收内存
                self.table.remove(&tail_ref.key, tail_ref.hash)?;
            }
        }

        Ok(())
    }

    /// 从lru缓存查询数据
    pub fn look_up(&self, key: &Slice, hash: u32) -> Result<Option<Arc<T>>> {
        match self.table.look_up(key, hash) {
            Ok(handle) => {
                match handle {
                    Some(handle) => {
                        // 返回为Arc<T>, 这样用户才可以和缓存在多个线程中共享数据
                        Ok(Some(unsafe { handle.as_ref() }.value.clone()))
                    }
                    None => { Ok(None) }
                }
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    /// 从lru缓存中删除数据, 同时回收内存
    pub fn erase(&mut self, key: &Slice, hash: u32) -> Result<usize> {
        let mut charge = 0;
        // 先从hash表中删除, 同时回收内存
        let removed_handle = self.table.remove(key, hash)?;
        if let Some(removed) = removed_handle {
            // 再删除lru链表中的数据
            self.lru_remove(removed)?;
            charge = unsafe { removed.as_ref().charge };
        }

        // 返回删除了多少数据量
        Ok(charge)
    }

    /// 清空lru缓存, 同时回收内存
    pub fn prune(&mut self) -> Result<()> {
        // hash表清空, 回收内存
        self.table.prune();
        // lru头尾指针置空
        self.head_of_lru = None;
        self.tail_of_lru = None;
        // 使用量归零
        self.usage = 0;
        Ok(())
    }

    /// 获取当前缓存的数据量
    #[inline]
    pub fn total_charge(&self) -> usize {
        self.usage
    }

    /// 获取当前hash表的槽位数
    pub fn slots(&self) -> usize {
        self.table.length
    }

    /// 向lru链表中插入新缓存, 头插法
    ///
    /// # Arguments
    ///
    /// * `head_of_list`:
    /// * `handle`:
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn lru_append(&mut self, mut handle: HandleRef<T>) -> Result<()> {
        if let None = self.head_of_lru {
            // 头节点为空时, 尾节点也为空
            self.head_of_lru = Some(handle);
            self.tail_of_lru = Some(handle);
            return Ok(());
        }
        // 头插法, 插入lru链表头
        let handle_mut = unsafe { handle.as_mut() };
        let mut head = self.head_of_lru.unwrap();
        let head_mut = unsafe { head.as_mut() };
        head_mut.prev = Some(handle);
        handle_mut.next = Some(head);

        // 更新头指针
        self.head_of_lru = Some(handle);

        Ok(())
    }

    /// 删除lru链表中的数据, 同时回收内存
    fn lru_remove(&mut self, mut handle: HandleRef<T>) -> Result<()> {
        let handle_mut = unsafe { handle.as_mut() };

        // 有上一节点, 上一节点直接连接到下一节点
        if let Some(mut prev) = handle_mut.prev {
            unsafe { prev.as_mut() }.next = handle_mut.next;
        } else {
            // 没有上一节点代表是链表头, 需要更新头指针
            self.head_of_lru = handle_mut.next;
        }

        // 有下一节点, 下一节点直接连接到上一节点
        if let Some(mut next) = handle_mut.next {
            unsafe { next.as_mut() }.prev = handle_mut.prev;
        } else {
            // 没有下一节点代表是链表尾, 需要更新尾指针
            self.tail_of_lru = handle_mut.prev;
        }

        // 使用量
        self.usage -= handle_mut.charge;

        // 删除后, 标记数据已经不在缓存中
        handle_mut.in_cache = false;

        Ok(())
    }
}

macro_rules! cache_element {
    ($shard:expr, $capacity:expr, $default_length:expr) => (RwLock::new(LRUCache::new_with_capacity($shard, pre_shard($capacity), $default_length)));
}

macro_rules! cache_element_default {
    ($shard:expr, $capacity:expr, $default_length:expr) => (RwLock::new(LRUCache::new($shard)));
}

const K_NUM_SHARD_BITS: usize = 5;
/// 默认shard数 32
const K_NUM_SHARDS: usize = 1 << K_NUM_SHARD_BITS;
/// 默认1000万条或者10M数据
const DEFAULT_CACHE_CAPACITY: usize = 10_000_000;
/// 负载因子不要太小, 否则会浪费内存
const LOAD_FACTOR: f32 = 0.75;
const DEFAULT_HASH_TABLE_LENGTH: usize = 16;
// 默认hash表长度为默认shard数*默认的hash表长度
const DEFAULT_SHARD_LENGTH: usize = K_NUM_SHARDS * DEFAULT_HASH_TABLE_LENGTH;

/// 具有多个shard的lru缓存
/// shard的实现可以降低锁粒度, 提高并发度
/// shard之间的lru容量是相等的, 会进行独立的lru淘汰, hash表扩容等操作
/// 每个shard拥有独立的读写锁, 一个shard的读写操作不会影响另一个shard的读写
/// 插入和删除数据时会更新容量, 当容量达到上限时会进行扩容操作
/// 目前没有实现自动的缩容操作, 可以调用total_charge判断当前容量并进行手动清空
///
/// ### Note
/// 1.当使用RC构成双向链表时, 请不要尝试打印cache, 否则会无限递归
/// ShardLRUCache, LRUCache, HandleTable 不实现Debug
/// 2. 加读锁后请勿再加次读锁, 否则可能死锁
/// |      线程1      |      线程2      |
/// |     ------     |     -------    |
/// |      read      |                |
/// |                |  write(block)  |
/// |   read(dead)   |                |
///
pub struct ShardLRUCache<T> {
    // shard用于降低锁粒度
    shard: [RwLock<LRUCache<T>>; 32],
    // 默认的初始化hash表长度, 用于初始化hash表
    // 使用较大的值可以避免扩容, 但是不要使用过大的值避免浪费空间
    default_length: usize,
    // 当前所有shard中lru cache的最大容量, 超过这个容量将会淘汰数据
    capacity: usize,
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

/// 所有权可以多线程传递
unsafe impl<T> Send for ShardLRUCache<T> {}

/// 不可变借用可以多线程共享, 内部shard具有可变性并且加锁, 可以安全的在多线程环境下使用
unsafe impl<T> Sync for ShardLRUCache<T> {}

impl<T> ShardLRUCache<T> {
    /// 私有化构造器
    /// 请使用ShardLRUCache::new()进行构造, 请勿尝试自行构造结构体
    fn default() -> Self {
        Self {
            shard: arr!([cache_element_default; 32]),
            default_length: DEFAULT_SHARD_LENGTH,
            capacity: DEFAULT_CACHE_CAPACITY,
        }
    }

    /// 创建ShardLruCache单线程使用
    /// 单线程使用时内部的读写锁会被编译器消除
    ///
    /// # Arguments
    ///
    /// * `capacity`: 最大容量, 超出这个容量时, 将会开始淘汰数据
    /// * `default_length`: 默认的hash表容量, 使用较大的值可以避免扩容, 但不要使用太大的值, 避免空间浪费
    ///
    /// returns: ShardLRUCache<T>
    ///
    /// # Examples
    ///
    /// ```
    /// use level_db_rust::util::cache::ShardLRUCache;
    /// let charge = 4;
    /// let total_length = 10000;
    /// ShardLRUCache::new_with_capacity(charge * total_length, 1000);
    /// ```
    pub fn new_with_capacity(capacity: usize, default_length: usize) -> ShardLRUCache<T> {
        let mut default_length = if default_length <= DEFAULT_SHARD_LENGTH {
            DEFAULT_SHARD_LENGTH
        } else {
            default_length
        };
        default_length = default_length / K_NUM_SHARDS;
        Self {
            shard: arr!([cache_element; 32]),
            default_length,
            capacity,
        }
    }


    /// 创建ShardLruCache多线程使用
    /// lookUp会加读锁, insert/erase/prune等写操作会加写锁
    /// 持有写锁的线程panic后, 会导致锁中毒, 数据无法访问, 持有读锁线程panic不会中毒
    ///
    /// # Arguments
    ///
    /// * `capacity`: 最大容量, 超出这个容量时, 将会开始淘汰数据
    /// * `default_length`: 默认的hash表容量, 使用较大的值可以避免扩容, 但不要使用太大的值, 避免空间浪费
    ///
    /// returns: Arc<ShardLRUCache<T>>
    ///
    /// # Examples
    ///
    /// ```
    /// use std::thread;
    /// use level_db_rust::util::cache::ShardLRUCache;
    /// let charge = 4;
    /// let total_length = 10000;
    /// let cache = ShardLRUCache::new_arc_with_capacity(charge * total_length, 1000);
    /// thread::spawn(move || -> Result<(),E>{
    ///     cache_clone.insert("key".into(), 1, charge)?;
    ///     Ok(())
    /// });
    /// ```
    pub fn new_arc_with_capacity(capacity: usize, default_length: usize) -> Arc<ShardLRUCache<T>> {
        let default_length = if default_length <= DEFAULT_SHARD_LENGTH {
            DEFAULT_SHARD_LENGTH
        } else {
            default_length
        };
        let default_length_per_shard = default_length / K_NUM_SHARDS;
        Arc::new(Self {
            shard: arr!([cache_element; 32]),
            default_length: default_length_per_shard,
            capacity,
        })
    }


    /// 向shard中插入数据
    /// 插入时会将值写入指定的shard, 每个
    /// # Arguments
    /// * `key`: 键
    /// * `value`: 值
    /// * `charge`: 值长度或者数据大小
    /// returns: Result<(), Status>
    /// # Examples
    /// ```
    /// use level_db_rust::util::cache::ShardLRUCache;
    /// let cache = ShardLRUCache::new_with_capacity(40_0000, 1000);
    /// let value = 1;
    /// cache.insert("key".into(), value, charge)?;
    /// ```
    pub fn insert(&self, key: &Slice, value: T, charge: usize) -> Result<()> {
        let hash = hash_slice(key);
        let shard = shard(hash);
        let result = self.shard[shard].write()?.insert(key.clone(), hash, value, charge);
        result
    }

    /// 从shard中查询缓存数据
    /// 返回Arc包装的数据, 便于多线程共享value的引用, 请不要在cache外回收value的内存
    ///
    /// # Arguments
    /// * `key`: 键
    /// returns: Result<Option<Arc<RwLock<LRUHandle<T>>>>, Status>
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use level_db_rust::util::cache::ShardLRUCache;
    /// use level_db_rust::util::slice::Slice;
    ///
    /// let cache = ShardLRUCache::new_with_capacity(40_0000, 1000);
    /// let key: Slice = "key".into();
    /// let value: Option<Arc<T>> = cache.lookup(&key)?;
    /// ```
    pub fn lookup(&self, key: &Slice) -> Result<Option<Arc<T>>> {
        let hash = hash_slice(key);
        let shard = shard(hash);
        self.shard[shard].read()?.look_up(key, hash)
    }

    /// 从shard中删除缓存数据
    ///
    /// # Arguments
    /// * `key`: 键
    /// returns: Result<(), Status>
    /// # Examples
    ///
    /// ```
    /// use level_db_rust::util::cache::ShardLRUCache;
    /// use level_db_rust::util::slice::Slice;
    ///
    /// let mut cache = ShardLRUCache::new_with_capacity(40_0000, 1000);
    /// let key: Slice = "key".into();
    /// cache.erase(&key)?;
    /// ```
    pub fn erase(&mut self, key: &Slice) -> Result<()> {
        let hash = hash_slice(key);
        // 删除缓存
        self.shard[shard(hash)].write()?.erase(key, hash)?;
        Ok(())
    }

    /// 清空全部shard的缓存
    ///
    /// returns: Result<(), Status>
    /// # Examples
    /// ```
    /// use level_db_rust::util::cache::ShardLRUCache;
    ///
    /// let mut cache = ShardLRUCache::new_with_capacity(40_0000, 1000);
    /// cache.prune()?;
    /// ```
    pub fn prune(&mut self) -> Result<()> {
        // 清空全部shard的缓存
        for shard in &mut self.shard {
            shard.write()?.prune()?
        }
        Ok(())
    }

    /// 获取当前缓存的总数据量
    pub fn total_charge(&self) -> Result<usize> {
        let mut total_charge = 0;
        for shard in &self.shard {
            total_charge += shard.read()?.total_charge();
        }
        Ok(total_charge)
    }

    /// 获取当前缓存的最大容量
    #[inline]
    #[allow(dead_code)]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// 获取当前全部shard中的槽位数, 可用于判断内存占用情况及扩容效果
    #[allow(dead_code)]
    pub fn slots(&self) -> Result<usize> {
        let mut slots = 0;
        for shard in &self.shard {
            slots += shard.read()?.slots();
        }
        Ok(slots)
    }
}

#[test]
fn test_insert_lookup_single() -> Result<()> {
    let capacity = 10_0000;
    let cache = ShardLRUCache::new_with_capacity(capacity, 100);
    let key = Slice::from("test_key".to_owned() + &1.to_string());
    cache.insert(&key, 1, 4)?;

    let result = cache.lookup(&key)?;
    assert_eq!(true, result.is_some());
    assert_eq!(1, *result.unwrap());

    Ok(())
}

#[test]
fn test_insert_cache() -> Result<()> {
    let size = 100_0000;
    let capacity = 1_0000_0000;
    let cache = ShardLRUCache::new_with_capacity(capacity, size);

    let slots = cache.slots()?;
    eprintln!("init slots: {}", slots);

    let charge = 4;
    for i in 0..size {
        let key = Slice::from("test_key".to_owned() + &i.to_string());
        // dbg!(key.clone().to_string());
        cache.insert(&key, i, charge)?;
    }

    let total_charge = cache.total_charge()?;
    dbg!(total_charge);
    assert_eq!(size * charge, total_charge);

    println!("insert count: {}", size);

    let slots = cache.slots()?;
    println!("slots after insert: {}", slots);

    for i in 0..size {
        let key = Slice::from("test_key".to_owned() + &i.to_string());
        let value = cache.lookup(&key)?;
        // dbg!(value.clone());
        assert_eq!(true, value.is_some(), "i: {}", i);
        assert_eq!(i, *value.unwrap());
    }


    Ok(())
}

#[test]
fn test_insert_lru() -> Result<()> {
    // 测试lru淘汰
    let size = 100_0000;
    let capacity = 4_0000;
    let cache = ShardLRUCache::new_with_capacity(capacity, size);
    let charge = 4;
    for i in 0..size {
        let key = Slice::from("test_key".to_owned() + &i.to_string());
        // dbg!(key.clone().to_string());
        cache.insert(&key, i, charge)?;
    }

    let total_charge = cache.total_charge()?;
    dbg!(total_charge);
    // 由于shard分布可能有倾斜, 写入的容量小于容量限制即可
    assert_eq!(true, total_charge < capacity);

    let mut count = 0;
    for i in 0..size {
        let key = Slice::from("test_key".to_owned() + &i.to_string());
        let value = cache.lookup(&key)?;
        // dbg!(value.clone());
        if let Some(v) = value {
            assert_eq!(i, *v, "i: {}", i);
            count += 1;
        }
    }

    // 由于shard分布可能有倾斜, 可以取出数量小于容量限制即可
    dbg!(count);
    assert_eq!(true, count < capacity / charge);

    // 写入数量应该等于写入容量除以单个数据的大小
    assert_eq!(count, total_charge / charge);

    Ok(())
}

#[test]
fn test_insert_cache_multi_thread() -> Result<()> {
    // todo 多线程写入 数据分组
    let capacity = 4_0000;
    let thread_count: usize = 8;
    let charge = 4;
    let cache = ShardLRUCache::new_arc_with_capacity(capacity, thread_count);

    let mut thread_vec = vec![];
    // 创建多线程写入缓存
    for i in 0..thread_count {
        let share_cache = cache.clone();
        let thread_builder = thread::Builder::new().name("my-thread".to_string().to_owned() + i.to_string().as_str());
        let thread = thread_builder.spawn(move || -> Result<()>{
            let key = Slice::from("test_key".to_string() + &i.to_string());
            share_cache.insert(&key, i, charge)?;

            // println!("write thread {}, write value: {}", i, i);
            Ok(())
        });
        thread_vec.push(thread);
    }

    for thread in thread_vec {
        thread?.join().unwrap()?;
    }

    let mut thread_vec = vec![];

    let in_cache_count = Arc::new(AtomicUsize::new(0));
    let out_cache_count = Arc::new(AtomicUsize::new(0));
    // 创建多线程读取缓存
    for i in 0..thread_count {
        let share_cache = cache.clone();
        let share_in_cache_count = in_cache_count.clone();
        let share_out_cache_count = out_cache_count.clone();
        let thread = thread::spawn(move || -> Result<()>{
            let key = Slice::from("test_key".to_string() + &i.to_string());
            let read = share_cache.lookup(&key)?;
            if read.is_some() {
                assert_eq!(i, *read.clone().unwrap().as_ref());
                share_in_cache_count.fetch_add(1, Ordering::Relaxed);
            } else {
                share_out_cache_count.fetch_add(1, Ordering::Relaxed);
            }
            Ok(())
        });
        thread_vec.push(thread);
    }

    for thread in thread_vec {
        thread.join().unwrap()?;
    }

    println!("in cache count: {}", in_cache_count.load(Ordering::Acquire));
    println!("out cache count: {}", out_cache_count.load(Ordering::Acquire));
    let total_charge = cache.total_charge()?;
    println!("thread_count: {}, charge: {}, capacity: {}, total_charge: {}", thread_count, charge, capacity, total_charge);
    assert_eq!(true, charge * in_cache_count.load(Ordering::Acquire) < capacity);

    Ok(())
}

#[test]
fn test_erase_cache() -> Result<()> {
    let mut cache = ShardLRUCache::new_with_capacity(1000000000, 1024);
    let key = Slice::from("test_key");
    cache.insert(&key, 10, 4)?;
    cache.erase(&key)?;
    cache.insert(&key, 10, 4)?;
    cache.erase(&key)?;
    let handle = cache.lookup(&key)?;
    println!("{:?}", handle);
    assert_eq!(true, handle.is_none());

    Ok(())
}

#[test]
fn test_prune() -> Result<()> {
    let default_length = 1024;
    let mut cache = ShardLRUCache::new_with_capacity(1000000000, default_length);

    let slots = cache.slots()?;
    dbg!(slots);

    let count = 100_0000;

    let charge = 4;
    println!("-------------------- before insert --------------------");
    for i in 0..count {
        let key: Slice = ("key".to_owned() + &i.to_string()).into();
        cache.insert(&key, i, charge)?;
    }
    println!("-------------------- after insert --------------------");


    let total_charge = cache.total_charge()?;
    dbg!(total_charge);
    assert_eq!(charge * count, total_charge);

    for i in 0..count {
        let key: Slice = ("key".to_owned() + &i.to_string()).into();
        let value = cache.lookup(&key)?;
        assert_eq!(true, value.is_some(), "i: {}", i);
        assert_eq!(i, *value.unwrap());
    }

    dbg!(cache.capacity());
    let slots = cache.slots()?;
    dbg!(slots);

    println!("-------------------- before prune --------------------");
    cache.prune()?;
    println!("-------------------- after prune --------------------");

    let slots = cache.slots()?;
    dbg!(slots);
    assert_eq!(default_length, slots);
    dbg!(cache.capacity());

    // 清空后 总存储的数据量为0
    let total_charge = cache.total_charge()?;
    dbg!(total_charge);
    assert_eq!(0, total_charge);

    // 清空后 数据不能再查询出来
    for i in 0..count {
        let key: Slice = ("key".to_owned() + &i.to_string()).into();
        let value = cache.lookup(&key)?;
        assert_eq!(true, value.is_none(), "i: {}", i);
    }

    Ok(())
}