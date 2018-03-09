extern crate dummy_xml;

use std::fs::File;
use std::io::prelude::*;

use dummy_xml::parser;

#[test]
fn parse_test() {
    let mut f = File::open("./xml/large.xml").expect("file not found");
    let mut contents = String::new();
    let result = f.read_to_string(&mut contents);
    assert_eq!(result.is_ok(), true);

    let _ = parser::parse(contents.as_bytes());
}

#[test]
fn sample() {
    let result =
        dummy_xml::parser::parse_str("<parent><child1 name='go'/><child2 name='rust'/></parent>");
    match result {
        Ok(document) => {
            let root = document.root();
            println!("root is {}", root.name());
            println!(
                "child2's name is {:?}",
                root.last_child().and_then(|node| node.attribute("name"))
            );
        }
        Err(error) => panic!("{:?}", error),
    }
}

#[test]
fn parse_blank() {
    let result = parser::parse_str("");
    assert!(result.is_err());
}

#[test]
fn parse_negative_1() {
    let result = parser::parse_str("<parent><child1 ");
    assert!(result.is_err());
}
