use std::ptr;
use std::borrow::{Borrow, BorrowMut};

#[derive(Debug)]
pub struct Attribute {
    name: String,
    value: String,
    next: Option<Box<Attribute>>,
    prev: *mut Attribute,
}

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

#[derive(Debug)]
pub struct Node {
    name: String,
    value: String,
    node_type: NodeType,
    next: Option<Box<Node>>,
    parent: *mut Node,
    prev: *mut Node,
    first_child: Option<Box<Node>>,
    last_child: *mut Node,
    first_attr: Option<Box<Attribute>>,
    last_attr: *mut Attribute,
}

impl Attribute {
    #[inline]
    pub fn new(name: String, value: String) -> Box<Self> {
        Box::new(Attribute {
            name: name,
            value: value,
            next: None,
            prev: ptr::null_mut(),
        })
    }

    #[inline]
    pub fn name(&self) -> &String {
        &self.name
    }

    #[inline]
    pub fn value(&self) -> &String {
        &self.value
    }

    #[inline]
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    #[inline]
    pub fn set_value(&mut self, value: String) -> &mut Self {
        self.value = value;
        self
    }

    #[inline]
    pub fn next_attribute(&self) -> Option<&Self> {
        self.next.as_ref().map(|attr| attr.borrow())
    }

    #[inline]
    pub fn next_attribute_mut(&mut self) -> Option<&mut Self> {
        self.next.as_mut().map(|attr| attr.borrow_mut())
    }

    #[inline]
    pub fn previous_attribute(&self) -> Option<&Self> {
        unsafe { self.prev.as_ref().map(|attr| attr.borrow()) }
    }

    #[inline]
    pub fn previous_attribute_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.prev.as_mut().map(|attr| attr.borrow_mut()) }
    }
}

impl PartialEq for Attribute {
    #[inline]
    fn eq(&self, other: &Attribute) -> bool {
        self.name == other.name && self.value == other.value
    }
}

impl Node {
    #[inline]
    pub fn new(name: String) -> Box<Self> {
        Box::new(Node {
            name: name,
            node_type: NodeType::Element,
            value: String::from(""),
            next: None,
            prev: ptr::null_mut(),
            parent: ptr::null_mut(),
            first_child: None,
            last_child: ptr::null_mut(),
            first_attr: None,
            last_attr: ptr::null_mut(),
        })
    }

    #[inline]
    pub fn new_by_type(node_type: NodeType) -> Box<Self> {
        Box::new(Node {
            name: String::from(""),
            node_type: node_type,
            value: String::from(""),
            next: None,
            prev: ptr::null_mut(),
            parent: ptr::null_mut(),
            first_child: None,
            last_child: ptr::null_mut(),
            first_attr: None,
            last_attr: ptr::null_mut(),
        })
    }

    #[inline]
    pub fn name(&self) -> &String {
        &self.name
    }

    #[inline]
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    #[inline]
    pub fn value(&self) -> &String {
        &self.value
    }

    #[inline]
    pub fn set_value(&mut self, value: String) -> &mut Self {
        self.value = value;
        self
    }

    #[inline]
    pub fn node_type(&self) -> &NodeType {
        &self.node_type
    }

    #[inline]
    pub fn set_node_type(&mut self, node_type: NodeType) -> &mut Self {
        self.node_type = node_type;
        self
    }

    #[inline]
    pub fn next_sibling(&self) -> Option<&Self> {
        self.next.as_ref().map(|node| node.borrow())
    }

    #[inline]
    pub fn next_sibling_mut(&mut self) -> Option<&mut Self> {
        self.next.as_mut().map(|node| node.borrow_mut())
    }

    #[inline]
    pub fn previous_sibling(&self) -> Option<&Self> {
        unsafe { self.prev.as_ref().map(|node| node.borrow()) }
    }

    #[inline]
    pub fn previous_sibling_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.prev.as_mut().map(|node| node.borrow_mut()) }
    }

    #[inline]
    pub fn parent(&self) -> Option<&Self> {
        unsafe { self.parent.as_ref().map(|node| node.borrow()) }
    }

    #[inline]
    pub fn parent_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.parent.as_mut().map(|node| node.borrow_mut()) }
    }

    #[inline]
    pub fn first_child(&self) -> Option<&Self> {
        self.first_child.as_ref().map(|node| node.borrow())
    }

    #[inline]
    pub fn first_child_mut(&mut self) -> Option<&mut Self> {
        self.first_child.as_mut().map(|node| node.borrow_mut())
    }

    #[inline]
    pub fn last_child(&self) -> Option<&Self> {
        unsafe { self.last_child.as_ref().map(|node| node.borrow()) }
    }

    #[inline]
    pub fn last_child_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.last_child.as_mut().map(|node| node.borrow_mut()) }
    }

    #[inline]
    pub fn first_attribute(&self) -> Option<&Attribute> {
        self.first_attr.as_ref().map(|node| node.borrow())
    }

    #[inline]
    pub fn first_attribute_mut(&mut self) -> Option<&mut Attribute> {
        self.first_attr.as_mut().map(|node| node.borrow_mut())
    }

    #[inline]
    pub fn last_attribute(&self) -> Option<&Attribute> {
        unsafe { self.last_attr.as_ref().map(|node| node.borrow()) }
    }

    #[inline]
    pub fn last_attribute_mut(&mut self) -> Option<&mut Attribute> {
        unsafe { self.last_attr.as_mut().map(|node| node.borrow_mut()) }
    }

    #[inline]
    pub fn append_child(&mut self, name: String) -> &mut Self {
        let mut node = Node::new(name);
        let raw_ptr: *mut _ = &mut *node;
        node.parent = self;
        match unsafe { self.last_child.as_mut() } {
            Some(last_child) => {
                node.prev = last_child;
                last_child.next = Some(node);
                self.last_child = raw_ptr;
            }
            None => {
                self.first_child = Some(node);
                self.last_child = raw_ptr;
            }
        }
        unsafe { &mut *raw_ptr }
    }

    #[inline]
    pub fn prepend_child(&mut self, name: String) -> &mut Self {
        let mut node = Node::new(name);
        let raw_ptr: *mut _ = &mut *node;
        node.parent = self;
        match self.first_child.take() {
            Some(mut first_child) => {
                first_child.prev = raw_ptr;
                node.next = Some(first_child);
                self.first_child = Some(node);
            }
            None => {
                self.first_child = Some(node);
                self.last_child = raw_ptr;
            }
        }
        unsafe { &mut *raw_ptr }
    }

    #[inline]
    pub fn append_child_by_type(&mut self, node_type: NodeType) -> &mut Self {
        let mut node = Node::new_by_type(node_type);
        let raw_ptr: *mut _ = &mut *node;
        node.parent = self;
        match unsafe { self.last_child.as_mut() } {
            Some(last_child) => {
                node.prev = last_child;
                last_child.next = Some(node);
                self.last_child = raw_ptr;
            }
            None => {
                self.first_child = Some(node);
                self.last_child = raw_ptr;
            }
        }
        unsafe { &mut *raw_ptr }
    }

    #[inline]
    pub fn prepend_child_by_type(&mut self, node_type: NodeType) -> &mut Self {
        let mut node = Node::new_by_type(node_type);
        let raw_ptr: *mut _ = &mut *node;
        node.parent = self;
        match self.first_child.take() {
            Some(mut first_child) => {
                first_child.prev = raw_ptr;
                node.next = Some(first_child);
                self.first_child = Some(node);
            }
            None => {
                self.first_child = Some(node);
                self.last_child = raw_ptr;
            }
        }
        unsafe { &mut *raw_ptr }
    }

    #[inline]
    pub fn insert_child_after(&mut self, name: String, child: &mut Node) -> &mut Self {
        let mut node = Node::new(name);
        let raw_ptr: *mut _ = &mut *node;
        node.parent = self;
        node.prev = child;
        node.next = child.next.take();
        child.next = Some(node);
        unsafe { &mut *raw_ptr }
    }
    // xml_node xml_node::insert_child_before(const char_t* name, const xml_node& node);

    #[inline]
    pub fn append_attribute(&mut self, name: String, value: String) -> &mut Attribute {
        let mut attr = Attribute::new(name, value);
        let raw_ptr: *mut _ = &mut *attr;
        match unsafe { self.last_attr.as_mut() } {
            Some(last_attr) => {
                attr.prev = last_attr;
                last_attr.next = Some(attr);
                self.last_attr = raw_ptr;
            }
            None => {
                self.first_attr = Some(attr);
                self.last_attr = raw_ptr;
            }
        }
        unsafe { &mut *raw_ptr }
    }

    #[inline]
    pub fn prepend_attribute(&mut self, name: String, value: String) -> &mut Attribute {
        let mut attr = Attribute::new(name, value);
        let raw_ptr: *mut _ = &mut *attr;
        match self.first_attr.take() {
            Some(mut first_attr) => {
                first_attr.prev = raw_ptr;
                attr.next = Some(first_attr);
                self.first_attr = Some(attr);
            }
            None => {
                self.first_attr = Some(attr);
                self.last_attr = raw_ptr;
            }
        }
        unsafe { &mut *raw_ptr }
    }

    // xml_attribute xml_node::insert_attribute_after(const char_t* name, const xml_attribute& attr);
    // xml_attribute xml_node::insert_attribute_before(const char_t* name, const xml_attribute& attr);

    // typedef xml_node_iterator iterator;
    // iterator begin() const;
    // iterator end() const;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_node_test() {
        let mut node = Node::new("mpd".to_string());
        assert_eq!(node.name(), "mpd");
        node.append_child("child1".to_string());
        node.append_child("child2".to_string());

        {
            let c1 = node.first_child().unwrap();
            assert_eq!(c1.name(), "child1");

            let c2 = c1.next_sibling().unwrap();
            assert_eq!(c2.name(), "child2");

            let node = c2.parent().unwrap();
            assert_eq!(node.name(), "mpd");
        }

        node.prepend_child("child0".to_string());

        {
            let c1 = node.first_child().unwrap();
            assert_eq!(c1.name(), "child0");
        }

        node.append_child_by_type(NodeType::PcData)
            .set_value("text1".to_string());

        {
            let txt = node.last_child().unwrap();
            assert_eq!(txt.value(), "text1");
        }
    }

    #[test]
    fn function_chain_test() {
        let mut node = Node::new("mpd".to_string());
        assert_eq!(node.name(), "mpd");
        node.append_child("child1".to_string())
            .set_value("value1".to_string());

        let c1 = node.first_child().unwrap();
        assert_eq!(c1.name(), "child1");
        assert_eq!(c1.value(), "value1");
    }

    #[test]
    fn new_attribute_test() {
        let mut attr = Attribute::new("class".to_string(), "test".to_string());

        //borrow_mut
        {
            assert_eq!(attr.name(), "class");
            assert_eq!(attr.value(), "test");

            attr.set_name("id".to_string());
            attr.set_value("main".to_string());
        }
    }
}
