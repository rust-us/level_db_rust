use std::cmp::Ordering;
use std::ptr::NonNull;
use std::rc::Rc;

use crate::traits::comparator_trait::ComparatorTrait;
use crate::util::Arena;
use crate::util::comparator::BytewiseComparatorImpl;
use crate::util::Result;
use crate::util::slice::Slice;

use rand::prelude::*;
use crate::util::status::{LevelError, Status};

// todo
struct Node<T> {
    /// 存储的值
    key: T,
    /// 最大深度
    max_level: u8,
    /// 柔性数组, 该节点层下存储的指向后方的节点
    next: Vec<NonNull<Node<T>>>,
}

pub struct SkipList<T, It: ComparatorTrait> {
    /// 最高层数
    level: u8,
    /// 最高层数
    max_level: u8,
    /// 存储数据数量
    num: usize,
    /// 头部指针
    head: Option<Node<T>>,
    /// 比较器
    comp: Rc<It>,
    /// 内存分配器
    arena: Rc<Arena>,
}

impl<T, It: ComparatorTrait> SkipList<T, It> {

    pub fn create(comparator: Rc<It>, arena: Rc<Arena>) -> Self {
        Self {
            level: 0,
            max_level: 8,
            num: 0,
            head: None,
            comp: comparator,
            arena,
        }
    }

    pub fn insert(&mut self, key: &Slice) -> Result<()> {
        if self.head.is_none() {
            self.head = Some(Node::create(key));
            self.level = 1;
            self.num = 1;
            self.max_level = 1;
            return Ok(())
        }
        let pre = self.head.unwrap();
        let mut next = true;
        while next {
            match self.comp.compare(&pre.key, key) {
                None => Err(Status::wrapper(LevelError::KInvalidArgument, "key not comparable".into()));
                Some(Ordering::Equal) => {

                },
                Some(Ordering::Less) => {

                },
                Some(Ordering::Greater) => {

                }
            }
        }
        todo!()
    }

    pub fn contains(&self, _key: &Slice) -> bool {
        todo!()
    }

    pub fn get_max_height(&self) -> usize {
        todo!()
    }

    fn rnd_level(&self) -> u8 {
        let mut level = 1;
        for _ in 1..self.max_level {
            if random() {
                level += 1;
            }
        }
        level
    }

}

impl <T> Node<T> {
    fn create(key: T) -> Self {
        Self {
            key,
            max_level: 1,
            next: Vec::with_capacity(4),
        }
    }
}