# dummy_xml
A fast DOM XML parser

## Example
How to parse:
```rust
extern crate dummy_xml;

...

use dummy_xml::parser;

...

let result = parser::parse_str("<parent><child1 name='go'/><child2 name='rust'/></parent>");
match result {
    Ok(document) => {
        let root = document.root();
        println!("root is {:?}", root);
    }
    Err(error) => panic!("{:?}", error),
}
```
