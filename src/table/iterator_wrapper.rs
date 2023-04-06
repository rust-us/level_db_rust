use crate::traits::DataIterator;
use crate::util::slice::Slice;

pub struct IteratorWrapper {
    iterator: DataIterator,
    valid: bool,
    key: &Slice
}

impl IteratorWrapper {
    fn iter(&self) -> impl DataIterator {
        return &self.iterator;
    }

    fn set(&mut self, iter: impl DataIterator) {
        if (self.iterator == null) {

        }
    }

    fn valid(&self) -> bool {
        return self.valid;
    }

    fn key(&self) -> &Slice {
        return self.key;
    }

    fn value(&self) -> &Slice {
        return self.iterator.value();
    }

    fn status(&self) {
        return self.iterator.stauts();
    }

    fn next(&mut self) {
        self.iterator.next();
        self.update();
    }

    fn prev(&mut self) {
        self.iterator.pre();
        self.update();
    }

    fn seek(&mut self, target: &Slice) {
        self.iterator.seek(target);
        self.update();
    }

    fn seek_to_first(&mut self) {
        self.iterator.seek_to_first();
        self.update()
    }

    fn seek_to_last(&mut self) {
        self.iterator.seek_to_last();
        self.update();
    }

    fn update(&mut self) {
        self.valid = self.iterator.valid();
        if self.valid == true {
            self.key = self.iterator.key();
        }
    }
}