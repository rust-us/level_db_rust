use crate::util::Slice;

pub trait Iterator {

    fn valid(&self) -> bool;

    fn seek_to_first(&mut self);

    fn seek_to_last(&mut self);

    fn seek(&mut self, target: &Slice);

    fn next(&mut self);

    fn pre(&mut self);

    fn key(&self) -> &Slice;

    fn value(&self) -> &Slice;
}

trait AAA {

}
