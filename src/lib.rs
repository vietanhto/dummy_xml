#![feature(test, allocator_api)]
extern crate test;
extern crate time;

pub mod node;
pub mod parser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
