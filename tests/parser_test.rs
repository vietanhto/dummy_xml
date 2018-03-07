#![feature(test)]
extern crate dummy_xml;
extern crate test;

use std::fs::File;
use std::io::prelude::*;
use test::Bencher;

use dummy_xml::parser::Parser;

#[bench]
fn bench_parse(b: &mut Bencher) {
    let mut f = File::open("./xml/large.xml").expect("file not found");
    let mut contents = String::new();
    let result = f.read_to_string(&mut contents);
    assert_eq!(result.is_ok(), true);

    let parser = Parser::new();
    b.iter(|| {
        let _ = parser.parse(contents.as_bytes());
    });
}
