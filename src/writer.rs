use node::{Attribute, Node};
use std::fmt::Write;

pub fn write(src: &Node, des: &mut String) {
    write!(des, "<{}", src.name()).expect("Error occurred while trying to write in String");
    src.first_attribute().map(|attr| write_attribute(attr, des));
    write!(des, ">").expect("Error occurred while trying to write in String");
    src.first_child().map(|node| write(node, des));
    write!(des, "</{}>", src.name()).expect("Error occurred while trying to write in String");
    src.next_sibling().map(|node| write(node, des));
}

fn write_attribute(src: &Attribute, des: &mut String) {
    if src.value().contains("'") {
        write!(des, " {}=\"{}\"", src.name(), src.value())
            .expect("Error occurred while trying to write in String");
    } else {
        write!(des, " {}='{}'", src.name(), src.value())
            .expect("Error occurred while trying to write in String");
    }
    src.next_attribute().map(|attr| write_attribute(attr, des));
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser;

    #[test]
    fn write_test() {
        let result = parser::parse_str("<parent><child1 name='go'></child1></parent>");
        let mut txt = String::new();
        write(result.unwrap().root(), &mut txt);
        assert_eq!(txt, "<parent><child1 name='go'></child1></parent>");
    }
}
