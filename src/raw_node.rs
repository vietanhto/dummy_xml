use std::ptr;
use std::borrow::{Borrow, BorrowMut};

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
}
