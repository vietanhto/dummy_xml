use node::Node;

pub struct Document {
    first: Option<Node>,
}

impl Document {
    pub fn new() -> Document {
        Document { first: None }
    }
}