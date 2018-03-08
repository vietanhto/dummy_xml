extern crate dummy_xml;

use dummy_xml::writer;
use dummy_xml::node::Node;

#[test]
fn sample() {
    let mut root = Node::new("parent".to_string());
    root.append_child("child1".to_string())
        .append_attribute("gender".to_string(), "male".to_string());
    root.append_child("child2".to_string())
        .append_attribute("gender".to_string(), "female".to_string());
    root.append_child("child3".to_string())
        .append_attribute("gender".to_string(), "non-gender".to_string());
    let mut result = String::new();
    writer::write(&root, &mut result);
    println!("{}", result);
}
