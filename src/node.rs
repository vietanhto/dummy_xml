use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::fmt;

use attribute::Attribute;

type Link<T> = Option<Rc<RefCell<T>>>;

#[derive(Debug)]
pub enum NodeType {
    Element,
    PcData,                //<node> text1 <child/> text2 </node>
    CData,                 //<node> <![CDATA[text1]]> <child/> <![CDATA[text2]]> </node> -> done
    Comment,               //<!-- comment text -->
    ProcessingInstruction, //?name value?>
    Declaration,           //<?xml version="1.0"?>
    Doctype,               //<!DOCTYPE greeting [ <!ELEMENT greeting (#PCDATA)> ]>
}

pub struct Node {
    name: String,
    node_type: NodeType,
    value: String,
    me: Link<Node>,
    next: Link<Node>,
    parent: Link<Node>,
    prev: Link<Node>,
    first_child: Link<Node>,
    last_child: Link<Node>,
    first_attr: Link<Attribute>,
    last_attr: Link<Attribute>,
}

impl Node {
    pub fn new(name: String) -> Rc<RefCell<Node>> {
        let node = Rc::new(RefCell::new(Node {
            name: name,
            node_type: NodeType::Element,
            value: String::from(""),
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

    pub fn new_by_type(node_type: NodeType) -> Rc<RefCell<Node>> {
        let node = Rc::new(RefCell::new(Node {
            name: String::from(""),
            node_type: node_type,
            value: String::from(""),
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

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn set_value(&mut self, value: String) -> &mut Self {
        self.value = value;
        self
    }

    pub fn node_type(&self) -> &NodeType {
        &self.node_type
    }

    pub fn set_node_type(&mut self, node_type: NodeType) -> &mut Self {
        self.node_type = node_type;
        self
    }

    pub fn next_sibling(&self) -> Option<Ref<Self>> {
        self.next.as_ref().map(|node| node.borrow())
    }

    pub fn next_sibling_mut(&mut self) -> Option<RefMut<Self>> {
        self.next.as_mut().map(|node| node.borrow_mut())
    }

    pub fn previous_sibling(&self) -> Option<Ref<Self>> {
        self.prev.as_ref().map(|node| node.borrow())
    }

    pub fn previous_sibling_mut(&mut self) -> Option<RefMut<Self>> {
        self.prev.as_mut().map(|node| node.borrow_mut())
    }

    pub fn parent(&self) -> Option<Ref<Self>> {
        self.parent.as_ref().map(|node| node.borrow())
    }

    pub fn parent_mut(&mut self) -> Option<RefMut<Self>> {
        self.parent.as_mut().map(|node| node.borrow_mut())
    }

    pub fn first_child(&self) -> Option<Ref<Self>> {
        self.first_child.as_ref().map(|node| node.borrow())
    }

    pub fn first_child_mut(&mut self) -> Option<RefMut<Self>> {
        self.first_child.as_mut().map(|node| node.borrow_mut())
    }

    pub fn last_child(&self) -> Option<Ref<Self>> {
        self.last_child.as_ref().map(|node| node.borrow())
    }

    pub fn last_child_mut(&mut self) -> Option<RefMut<Self>> {
        self.last_child.as_mut().map(|node| node.borrow_mut())
    }

    pub fn clone_rc(&self) -> Rc<RefCell<Self>> {
        return self.me.clone().unwrap();
    }
    // xml_attribute xml_node::append_attribute(const char_t* name);
    // xml_attribute xml_node::prepend_attribute(const char_t* name);
    // xml_attribute xml_node::insert_attribute_after(const char_t* name, const xml_attribute& attr);
    // xml_attribute xml_node::insert_attribute_before(const char_t* name, const xml_attribute& attr);
    pub fn append_child(&mut self, name: String) -> Rc<RefCell<Node>> {
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
        node
    }

    pub fn prepend_child(&mut self, name: String) -> Rc<RefCell<Node>> {
        let node = Node::new(name);

        match self.first_child.take() {
            Some(first_child) => {
                node.borrow_mut().next = Some(first_child.clone());
                first_child.borrow_mut().prev = Some(node.clone());
                self.first_child = Some(node.clone());
            }
            None => {
                self.first_child = Some(node.clone());
                self.last_child = Some(node.clone())
            }
        }
        node.borrow_mut().parent = self.me.clone();
        node
    }

    pub fn append_child_by_type(&mut self, node_type: NodeType) -> Rc<RefCell<Node>> {
        let node = Node::new_by_type(node_type);

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
        node
    }

    pub fn prepend_child_by_type(&mut self, node_type: NodeType) -> Rc<RefCell<Node>> {
        let node = Node::new_by_type(node_type);

        match self.first_child.take() {
            Some(first_child) => {
                node.borrow_mut().next = Some(first_child.clone());
                first_child.borrow_mut().prev = Some(node.clone());
                self.first_child = Some(node.clone());
            }
            None => {
                self.first_child = Some(node.clone());
                self.last_child = Some(node.clone())
            }
        }
        node.borrow_mut().parent = self.me.clone();
        node
    }

    // xml_node xml_node::prepend_child(const char_t* name);
    // xml_node xml_node::insert_child_before(xml_node_type type, const xml_node& node);
    // xml_node xml_node::insert_child_after(const char_t* name, const xml_node& node);
    // xml_node xml_node::insert_child_before(const char_t* name, const xml_node& node);
    // typedef xml_node_iterator iterator;
    // iterator begin() const;
    // iterator end() const;
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.name, self.value, self.node_type)
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

        {
            let node = node.borrow();
            let c1 = node.first_child().unwrap();
            assert_eq!(c1.name(), "child1");

            let c2 = c1.next_sibling().unwrap();
            assert_eq!(c2.name(), "child2");

            let node = c2.parent().unwrap();
            assert_eq!(node.name(), "mpd");
        }

        node.borrow_mut().prepend_child("child0".to_string());

        {
            let node = node.borrow();
            let c1 = node.first_child().unwrap();
            assert_eq!(c1.name(), "child0");
        }

        let txt_node = node.borrow_mut().append_child_by_type(NodeType::PcData);
        txt_node.borrow_mut().set_value("text1".to_string());

        {
            let node = node.borrow();
            let txt = node.last_child().unwrap();
            assert_eq!(txt.value(), "text1");
        }
    }
}
