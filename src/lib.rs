#![feature(box_syntax)]
#![feature(let_else)]
#![feature(generic_associated_types)]

extern crate core;

pub mod db;
mod table;
pub mod util;
mod traits;

mod test {

    #[test]
    pub fn test() {
        println!("hello world");
    }

}