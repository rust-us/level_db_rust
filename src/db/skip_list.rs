use std::cmp::Ordering;
use std::mem;
use std::mem::size_of;
use std::ptr::null_mut;
use std::sync::{Arc, RwLock};

use rand::prelude::*;
use crate::debug;
use crate::traits::comparator_trait::Comparator;

use crate::util::arena::ArenaRef;
use crate::util::{Arena, Result};
use crate::util::slice::Slice;
use crate::util::status::{LevelError, Status};

type RawNode = *mut Node;

const MAX_LEVEL: usize = 8;

// todo
struct Node {
    /// 存储的值, 如果为空，则是头指针或者尾指针
    key: Option<Slice>,
    /// 数组元素首地址，代表一个数组，指向每层的下一个节点。
    next_elems: *mut RawNode,
    /// 当前节点高度
    level: usize,
}

pub struct SkipList<Cmp: Comparator> {
    /// 最高层数
    height: usize,
    /// 存储数据数量
    num: usize,
    /// 头部指针
    head: RawNode,
    /// 尾指针
    tail: RawNode,
    /// 比较器
    cmp: Arc<Cmp>,
    /// 内存分配器
    arena: ArenaRef,
}

pub struct Iter<'a, Cmp: Comparator> {
    list: &'a SkipList<Cmp>,
    node: RawNode,
}

impl<Cmp: Comparator> SkipList<Cmp> {
    pub fn create(comparator: Arc<Cmp>, arena: ArenaRef) -> Self {
        Self {
            height: 0,
            num: 0,
            head: Node::create_head(arena.clone()),
            tail: Node::create_tail(),
            cmp: comparator,
            arena,
        }
    }

    pub fn insert(&mut self, key: Slice) -> Result<()> {
        // TODO 这里是否可以优化
        if self.contains(&key) {
            return Ok(());
        }
        if self.num == 0 {
            self.insert_ele0(key)
        } else {
            unsafe {
                self.insert_elen(key)
            }
        }
    }

    #[inline]
    fn insert_ele0(&mut self, key: Slice) -> Result<()> {
        let level = rand_level();
        debug!("insert {}, level: {}", &key, level);
        let node = Node::create(key, level, self.arena.clone());
        // head bind node
        // TODO, use macro to expand for-loop
        unsafe {
            (&mut *node).level = level;
            (&mut *self.head).level = level;
            for l in 0..level {
                (&mut *self.head).set_node(l, node);
                (&mut *node).set_node(l, self.tail);
            }
        }
        self.height = level;
        self.num = 1;
        return Ok(());
    }

    unsafe fn insert_elen(&mut self, key: Slice) -> Result<()> {
        let mut current = self.head;
        let node_height = rand_level();
        let node_top_level = node_height - 1;
        debug!("insert {}, level: {}", &key, node_height);
        let node_ptr = Node::create(key, node_height, self.arena.clone());
        let node = unsafe { &mut *node_ptr };
        // loop from highest level to 0
        for l in (0..self.height).rev() {
            'inner_loop: loop {
                let ele_ptr = unsafe { (&*current).get_node(l) };
                let ele = unsafe { &mut *ele_ptr };
                if ele.is_tail() {
                    if l <= node_top_level {
                        // ele is tail node, add node to last
                        (&mut *current).set_node(l, node_ptr);
                        node.set_node(l, self.tail);
                        debug!("bind: {} before: {}, after: <tail>, at level: {}",
                                        node.key.as_ref().unwrap(),
                                        (&*current).key.as_ref().unwrap(),
                                        l);
                    }
                    break 'inner_loop;
                } else {
                    match self.cmp.compare(node.key.as_ref().unwrap(), ele.key.as_ref().unwrap()) {
                        Some(Ordering::Less) => {
                            // node higher than current level at ele
                            if node_top_level >= l {
                                (&mut *current).set_node(l, node_ptr);
                                node.set_node(l, ele_ptr);
                                if (&*current).is_head() {
                                    debug!("bind: {} before: <head>, after: {}, at level: {}",
                                        node.key.as_ref().unwrap(),
                                        ele.key.as_ref().unwrap(),
                                        l);
                                } else {
                                    debug!("bind: {} before: {}, after: {}, at level: {}",
                                        node.key.as_ref().unwrap(),
                                        (&*current).key.as_ref().unwrap(),
                                        ele.key.as_ref().unwrap(),
                                        l);
                                }
                            }
                            break 'inner_loop;
                        }
                        Some(Ordering::Greater) => {
                            current = ele;
                        }
                        Some(Ordering::Equal) => {
                            // ignore equals
                            return Ok(());
                        }
                        None => {
                            return Err(Status::wrapper(LevelError::KInvalidArgument, "key not comparable".into()));
                        }
                    }
                }
            }
        }
        // if head level is less than new node, then fix head node height
        if self.height < node_height {
            for l in (self.height()..node_height).rev() {
                (&mut *self.head).set_node(l, node_ptr);
                node.set_node(l, self.tail);
            }
            self.height = node_height;
        }
        self.num += 1;
        Ok(())
    }

    pub fn contains(&self, key: &Slice) -> bool {
        debug!("================== begin contains, key: {} ==================", key);
        if self.num == 0 {
            return false;
        }
        unsafe {
            let mut current = unsafe { &*self.head };
            for level in (0..self.height).rev() {
                'a_loop: loop {
                    let ele_ptr = current.get_node(level);
                    let ele = &*ele_ptr;
                    if ele.is_tail() {
                        // tail node
                        if level == 0 {
                            debug!("next is tail, return false");
                            return false;
                        } else {
                            debug!("next is tail, continue");
                            break 'a_loop;
                        }
                    }
                    {
                        debug!("node: {} at level: {}", ele.key.as_ref().unwrap(), level)
                    }
                    match self.cmp.compare(key, ele.key.as_ref().unwrap()) {
                        None => return false,
                        Some(Ordering::Equal) => return true,
                        Some(Ordering::Less) => {
                            // break loop, decrease the level
                            break;
                        }
                        Some(Ordering::Greater) => {
                            if current.level() == 0 {
                                return false;
                            }
                            current = ele;
                        }
                    };
                }
            }
        }
        // can not found in all level
        false
    }

    #[inline]
    pub fn max_height(&self) -> usize {
        MAX_LEVEL
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.num
    }

    #[inline]
    pub fn iter(&self) -> Iter<Cmp> {
        Iter::create(&self)
    }

    #[inline]
    pub fn memory_usage(&self) -> usize {
        let a = Arc::new(RwLock::new(Arena::default()));
        a.read().unwrap().memory_usage();
        self.arena.lock().unwrap().memory_usage()
    }

    fn rnd_level(&self) -> usize {
        let mut level = 1;
        for _ in 1..MAX_LEVEL {
            if random() {
                level += 1;
            }
        }
        level
    }
}

impl<Cmp: Comparator> ToString for SkipList<Cmp> {
    fn to_string(&self) -> String {
        let mut tree = String::with_capacity(1024);
        // calculate each item width
        let mut widths = Vec::with_capacity(tree.len());
        self.iter().for_each(|s| {
            widths.push(s.size());
        });
        // print value list
        if self.num > 0 {
            unsafe {
                let mut node = &*((&*self.head).get_node(0));
                tree.push_str("[head]");
                while !node.is_head_or_tail() {
                    tree.push_str(" -> ");
                    tree.push_str(node.key.as_ref().unwrap().as_str());
                    let level_str = format!("({})", node.level);
                    tree.push_str(level_str.as_str());
                    node = &*node.get_node(0);
                }
            }
        }
        tree.push_str("-> [tail]");
        format!("height: {}, num: {}\n {}", self.height, self.num, tree)
    }
}


impl Node {
    #[inline]
    fn create(src: Slice, level: usize, arena: ArenaRef) -> RawNode {
        let node = box Self {
            key: Some(src),
            next_elems: allocate_next_elems(arena),
            level,
        };
        Box::into_raw(node)
    }

    #[inline]
    fn create_head(arena: ArenaRef) -> RawNode {
        let node = box Self {
            key: None,
            next_elems: allocate_next_elems(arena),
            level: MAX_LEVEL,
        };
        Box::into_raw(node)
    }

    #[inline]
    fn create_tail() -> RawNode {
        let node = box Self {
            key: None,
            next_elems: null_mut(),
            level: 0,
        };
        Box::into_raw(node)
    }

    #[inline]
    #[must_use]
    fn is_head_or_tail(&self) -> bool {
        self.key.is_none()
    }

    #[inline]
    #[must_use]
    fn is_tail(&self) -> bool {
        self.key.is_none() && self.level == 0
    }

    #[inline]
    #[must_use]
    fn is_head(&self) -> bool {
        self.key.is_none() && self.level > 0
    }


    #[inline]
    fn level(&self) -> usize {
        self.level
    }

    #[inline]
    #[must_use]
    unsafe fn get_node(&self, level: usize) -> RawNode {
        assert!(level < MAX_LEVEL);
        self.next_elems.offset(level as isize).read()
    }

    #[inline]
    unsafe fn set_node(&mut self, level: usize, node: RawNode) {
        assert!(level < MAX_LEVEL);
        self.next_elems.offset(level as isize).write(node);
    }
}

fn rand_level() -> usize {
    let mut level = 1_usize;
    while random::<bool>() {
        level += 1;
        if level >= MAX_LEVEL {
            break;
        }
    }
    level
}

fn allocate_next_elems(arena: ArenaRef) -> *mut RawNode {
    // RawNode is a raw ptr
    assert_eq!(size_of::<RawNode>(), size_of::<usize>());
    // allocate next_elems to 8 capacity array
    let elems_size = size_of::<RawNode>() * MAX_LEVEL;
    let mut lock = arena.lock().expect("lock arena");
    let elems_ptr = lock.allocate(elems_size);
    // transmute raw ptr to RawNode ptr
    unsafe {
        mem::transmute(elems_ptr.as_ptr())
    }
}

#[inline]
fn min_max(a: usize, b: usize) -> (usize, usize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

// 'b lifetime is bigger than 'a
impl<'a, Cmp: Comparator> Iter<'a, Cmp> {
    fn create(list: &'a SkipList<Cmp>) -> Self {
        Self {
            list,
            node: list.head,
        }
    }
}

impl<'a, Cmp: Comparator> Iterator for Iter<'a, Cmp> {
    type Item = &'a Slice;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if (&*self.node).is_tail() {
                return None;
            } else {
                self.node = (&*self.node).get_node(0);
            }
            (&*self.node).key.as_ref()
        }
    }
}