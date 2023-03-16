#![feature(box_syntax)]
#![feature(let_else)]
#![feature(generic_associated_types)]

extern crate core;

mod db;
mod table;
mod util;
mod traits;

mod test {

    #[test]
    pub fn test() {
        println!("hello world");
    }

}