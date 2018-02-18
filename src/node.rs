use attribute::Attribute;

pub struct Node {
    name: String,
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>,
    first_attr: Option<Attribute>,
    last_attr: Option<Attribute>,
}

impl Node {
    pub fn new(name: String) -> Node {
        Node{name: name, next: None, prev: None, first_attr: None, last_attr: None}
    }
}