use attribute::Attribute;

#[derive(Debug)]
pub struct Node {
    name: String,
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>,
    first_attr: Option<Attribute>,
    last_attr: Option<Attribute>,
}

impl Node {
    pub fn new(name: String) -> Node {
        Node {
            name: name,
            next: None,
            prev: None,
            first_attr: None,
            last_attr: None,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn next(&self) -> &Option<Box<Node>> {
        &self.next
    }

    pub fn set_next(&mut self, value: Option<Box<Node>>) {
        self.next = value;
    }

    pub fn prev(&self) -> &Option<Box<Node>> {
        &self.prev
    }

    pub fn set_prev(&mut self, value: Option<Box<Node>>) {
        self.prev = value;
    }
}
