extern crate dummy_xml;

use dummy_xml::writer;
use dummy_xml::node::Node;

#[test]
fn sample() {
    let mut root = Node::new("parent");
    root.append_child("child1")
        .append_attribute("gender", "male");
    root.append_child("child2")
        .append_attribute("gender", "female");
    root.append_child("child3")
        .append_attribute("gender", "non-gender");
    let mut result = String::new();
    writer::write(&root, &mut result);
    println!("{}", result);
}
