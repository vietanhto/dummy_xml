use std::ptr;
use std::borrow::{Borrow, BorrowMut, Cow};

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
pub struct Node<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
    node_type: NodeType,
    next: Option<Box<Node<'a>>>,
    parent: *mut Node<'a>,
    prev: *mut Node<'a>,
    first_child: Option<Box<Node<'a>>>,
    last_child: *mut Node<'a>,
    first_attr: Option<Box<Attribute>>,
    last_attr: *mut Attribute,
}

impl Attribute {
    #[inline]
    pub fn new<S: Into<String>>(name: S, value: S) -> Box<Self> {
        Box::new(Attribute {
            name: name.into(),
            value: value.into(),
            next: None,
            prev: ptr::null_mut(),
        })
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }

    #[inline]
    pub fn set_name<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.name = name.into();
        self
    }

    #[inline]
    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.value = value.into();
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

const EMPTY_STRING: Cow<str> = Cow::Borrowed("");

impl<'a> Node<'a> {
    #[inline]
    pub fn new<S: Into<String>>(name: S) -> Box<Self> {
        Box::new(Node {
            name: Cow::Owned(name.into()),
            node_type: NodeType::Element,
            value: EMPTY_STRING,
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
            name: EMPTY_STRING,
            node_type: node_type,
            value: EMPTY_STRING,
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
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn set_name<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.name = Cow::Owned(name.into());
        self
    }

    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }

    #[inline]
    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.value = Cow::Owned(value.into());
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
    pub fn append_child<S: Into<String>>(&mut self, name: S) -> &mut Self {
        let mut node = Node::new(name.into());
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
    pub fn prepend_child<S: Into<String>>(&mut self, name: S) -> &mut Self {
        let mut node = Node::new(name.into());
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
    pub fn insert_child_after<S: Into<String>>(&mut self, name: S, child: &mut Self) -> &mut Self {
        let mut node = Node::new(name.into());
        let raw_ptr: *mut _ = &mut *node;
        node.parent = self;

        node.prev = child;
        node.next = child.next.take();
        node.next.as_mut().map(|n| n.prev = raw_ptr);
        child.next = Some(node);

        unsafe { &mut *raw_ptr }
    }

    #[inline]
    pub fn insert_child_before<S: Into<String>>(&mut self, name: S, child: &mut Self) -> &mut Self {
        let mut node = Node::new(name.into());
        let raw_ptr: *mut _ = &mut *node;
        node.parent = self;

        let prev_child = child.prev;

        node.prev = child.prev;
        child.prev = raw_ptr;
        unsafe {
            node.next = (*prev_child).next.take();
            (*prev_child).next = Some(node);
        }

        unsafe { &mut *raw_ptr }
    }

    #[inline]
    pub fn append_attribute<S: Into<String>>(&mut self, name: S, value: S) -> &mut Attribute {
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
    pub fn prepend_attribute<S: Into<String>>(&mut self, name: S, value: S) -> &mut Attribute {
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
        let mut node = Node::new("mpd");
        assert_eq!(node.name(), "mpd");
        node.append_child("child1");
        node.append_child("child2");

        {
            let c1 = node.first_child().unwrap();
            assert_eq!(c1.name(), "child1");

            let c2 = c1.next_sibling().unwrap();
            assert_eq!(c2.name(), "child2");

            let node = c2.parent().unwrap();
            assert_eq!(node.name(), "mpd");
        }

        node.prepend_child("child0");

        {
            let c1 = node.first_child().unwrap();
            assert_eq!(c1.name(), "child0");
        }

        node.append_child_by_type(NodeType::PcData)
            .set_value("text1");

        {
            let txt = node.last_child().unwrap();
            assert_eq!(txt.value(), "text1");
        }
    }

    #[test]
    fn function_chain_test() {
        let mut node = Node::new("mpd");
        assert_eq!(node.name(), "mpd");
        node.append_child("child1").set_value("value1");

        let c1 = node.first_child().unwrap();
        assert_eq!(c1.name(), "child1");
        assert_eq!(c1.value(), "value1");
    }

    #[test]
    fn new_attribute_test() {
        let mut attr = Attribute::new("class", "test");

        //borrow_mut
        {
            assert_eq!(attr.name(), "class");
            assert_eq!(attr.value(), "test");

            attr.set_name("id");
            attr.set_value("main");
        }
    }
}
