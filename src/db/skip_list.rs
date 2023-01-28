use std::rc::Rc;

use crate::traits::comparator_trait::Comparator;
use crate::util::Arena;
use crate::util::comparator::BytewiseComparatorImpl;
use crate::util::Result;
use crate::util::slice::Slice;

// todo
struct Node<T> {
    value: T,
}

pub struct SkipList<T, It: Comparator> {
    node: Option<Node<T>>,
    comp: Rc<It>,
}

impl<T, It: Comparator> SkipList<T, It> {

    pub fn create(comparator: Rc<It>, _arena: Rc<Arena>) -> Self {
        Self {
            node: None,
            comp: comparator,
        }
    }

    pub fn insert(&mut self, _seq_no: usize, _key: &Slice) -> Result<()> {
        todo!()
    }

    pub fn contains(&self, _key: &Slice) -> bool {
        todo!()
    }

    pub fn get_max_height(&self) -> usize {
        todo!()
    }
}