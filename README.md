# dummy_xml
A fast DOM XML parser.
This is a bad incomplete clone of [pugixml](https://pugixml.org/)

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

//expect to see: 
//<parent>
//    <child1 gender='male'></child1>
//    <child2 gender='female'></child2>
//    <child3 gender='non-gender'></child3>
//</parent>
```

## Performance
```rust
//dummy_xml
test dummy_xml_large     ... bench:   2,085,654 ns/iter (+/- 320,643)
test dummy_xml_medium    ... bench:     613,433 ns/iter (+/- 99,178)
test dummy_xml_small     ... bench:       4,531 ns/iter (+/- 867)

//quick-xml
test quick_xml_large     ... bench:   2,043,964 ns/iter (+/- 237,291)
test quick_xml_medium    ... bench:     489,891 ns/iter (+/- 76,405)
test quick_xml_small     ... bench:       6,982 ns/iter (+/- 962)

//sxd_document
test sxd_document_medium ... bench:   3,351,998 ns/iter (+/- 479,587)
test sxd_document_small  ... bench:      52,789 ns/iter (+/- 8,984)

//xml5ever
test xml5ever_large      ... bench:   8,014,460 ns/iter (+/- 937,028)
test xml5ever_medium     ... bench:   7,051,655 ns/iter (+/- 779,365)
test xml5ever_small      ... bench:      45,966 ns/iter (+/- 35,997)

//xmlrs
test xmlrs_large         ... bench:  28,106,508 ns/iter (+/- 2,995,895)
test xmlrs_medium        ... bench:  13,045,977 ns/iter (+/- 1,617,525)
test xmlrs_small         ... bench:      94,883 ns/iter (+/- 17,121)
```
Benchmark using [choose-your-xml-rs](https://github.com/RazrFalcon/choose-your-xml-rs) on Mar 8, 2018