use std::ptr::addr_of_mut;
use crate::util::slice::Slice;
use crate::util::status::Status;

pub struct Iterator {

}

trait IteratorTrait {
    fn valid(&self) -> bool;
    fn seek(&mut self, target: i32);
    fn seek_to_first(&mut self);
    fn seek_to_last(&mut self);
    fn next(&mut self);
    fn prev(&mut self);
    fn key(&self) -> Slice;
    fn value(&self) -> Slice;
    fn status(&self) -> Status;
}