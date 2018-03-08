# dummy_xml
A fast DOM XML parser. 

## Example
How to parse:
```rust
extern crate dummy_xml;

use dummy_xml::parser;

let result = parser::parse_str("<parent><child1 name='go'/><child2 name='rust'/></parent>");
match result {
    Ok(document) => {
        let root = document.root();
        println!("root is {:?}", root);
    }
    Err(error) => panic!("{:?}", error),
}
```

How to write:
```rust
use dummy_xml::writer;
use dummy_xml::node::Node;

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

//expect to see: 
//<parent>
//    child1 gender='male'></child1>
//    <child2 gender='female'></child2>
//    <child3 gender='non-gender'></child3>
//</parent>
```