use std::rc::Rc;
use crate::traits::comparator_trait::ComparatorTrait;
use crate::util::Arena;
use crate::util::slice::Slice;
use crate::util::Result;

pub struct SkipList<T> {
    node: Node<T>
}

struct Node<T> {
    value: T,
}

impl <T> SkipList<T> {

    pub fn create(_comparator: Rc<Box<dyn ComparatorTrait>>, _arena: Rc<Arena>) -> Self {
        todo!()
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