#![feature(test, allocator_api)]
extern crate test;
extern crate time;

pub mod attribute;
pub mod node;
pub mod parser;
pub mod raw_node;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
