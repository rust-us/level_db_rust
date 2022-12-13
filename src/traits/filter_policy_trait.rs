use crate::util::slice::Slice;

pub trait FilterPolicy {

    fn name() -> String;

    fn create_filter(&self, keys: Slice, n: u32) -> String;

}