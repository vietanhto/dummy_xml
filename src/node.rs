use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::fmt;

use attribute::Attribute;

type Link<T> = Option<Rc<RefCell<T>>>;

pub struct Node {
    name: String,
    me: Link<Node>,
    next: Link<Node>,
    parent: Link<Node>,
    prev: Link<Node>,
    first_child: Link<Node>,
    last_child: Link<Node>,
    first_attr: Link<Attribute>,
    last_attr: Link<Attribute>,
}

pub trait XmlNode {
    fn name(&self) -> &String;
    fn next_sibling(&self) -> Option<Ref<Self>>;
    fn previous_sibling(&self) -> Option<Ref<Self>>;
    fn parent(&self) -> Option<Ref<Self>>;
    fn parent_mut(&mut self) -> Option<RefMut<Self>>;
    fn first_child(&self) -> Option<Ref<Self>>;
    fn last_child(&self) -> Option<Ref<Self>>;
    fn clone_rc(&self) -> Rc<RefCell<Self>>;
    // fn next_sibling(&self) -> Ref<Self>;
    // fn previous_sibling(&self) -> Ref<Self>;
    // xml_attribute xml_node::append_attribute(const char_t* name);
    // xml_attribute xml_node::prepend_attribute(const char_t* name);
    // xml_attribute xml_node::insert_attribute_after(const char_t* name, const xml_attribute& attr);
    // xml_attribute xml_node::insert_attribute_before(const char_t* name, const xml_attribute& attr);
    fn append_child(&mut self, name: String);
    // xml_node xml_node::prepend_child(xml_node_type type = node_element);
    // xml_node xml_node::insert_child_after(xml_node_type type, const xml_node& node);
    // xml_node xml_node::insert_child_before(xml_node_type type, const xml_node& node);

    // xml_node xml_node::append_child(const char_t* name);
    // xml_node xml_node::prepend_child(const char_t* name);
    // xml_node xml_node::insert_child_after(const char_t* name, const xml_node& node);
    // xml_node xml_node::insert_child_before(const char_t* name, const xml_node& node);
}

impl Node {
    pub fn new(name: String) -> Rc<RefCell<Node>> {
        let node = Rc::new(RefCell::new(Node {
            name: name,
            me: None,
            next: None,
            prev: None,
            parent: None,
            first_child: None,
            last_child: None,
            first_attr: None,
            last_attr: None,
        }));
        node.borrow_mut().me = Some(node.clone());
        node
    }

    // pub fn name(&self) -> &String {
    //     &self.name
    // }

    // pub fn next(&self) -> Option<Ref<Node>> {
    //     self.next.as_ref().map(|node| node.borrow())
    // }

    // pub fn set_next(&mut self, value: Link<Node>) {
    //     self.next = value;
    // }

    // pub fn prev(&self) -> Option<Ref<Node>> {
    //     self.prev.as_ref().map(|node| node.borrow())
    // }

    // pub fn set_prev(&mut self, value: Link<Node>) {
    //     self.prev = value;
    // }
}

impl XmlNode for Node {
    fn name(&self) -> &String {
        &self.name
    }

    fn next_sibling(&self) -> Option<Ref<Self>> {
        self.next.as_ref().map(|node| node.borrow())
    }

    fn previous_sibling(&self) -> Option<Ref<Self>> {
        self.prev.as_ref().map(|node| node.borrow())
    }

    fn parent(&self) -> Option<Ref<Self>> {
        self.parent.as_ref().map(|node| node.borrow())
    }

    fn parent_mut(&mut self) -> Option<RefMut<Self>> {
        self.parent.as_mut().map(|node| node.borrow_mut())
    }

    fn clone_rc(&self) -> Rc<RefCell<Self>> {
        return self.me.clone().unwrap();
    }

    fn append_child(&mut self, name: String) {
        let node = Node::new(name);

        match self.last_child.take() {
            Some(last_child) => {
                node.borrow_mut().prev = Some(last_child.clone());
                last_child.borrow_mut().next = Some(node.clone());
                self.last_child = Some(node.clone());
            }
            None => {
                self.last_child = Some(node.clone());
                self.first_child = Some(node.clone())
            }
        }
        node.borrow_mut().parent = self.me.clone();
    }

    fn first_child(&self) -> Option<Ref<Self>> {
        self.first_child.as_ref().map(|node| node.borrow())
    }

    fn last_child(&self) -> Option<Ref<Self>> {
        self.last_child.as_ref().map(|node| node.borrow())
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_node_test() {
        let node = Node::new("mpd".to_string());
        assert_eq!(node.borrow().name(), "mpd");
        node.borrow_mut().append_child("child1".to_string());
        node.borrow_mut().append_child("child2".to_string());

        let node = node.borrow();
        let c1 = node.first_child().unwrap();
        assert_eq!(c1.name(), "child1");

        let c2 = c1.next_sibling().unwrap();
        assert_eq!(c2.name(), "child2");

        let node = c2.parent().unwrap();
        assert_eq!(node.name(), "mpd");
    }
}
