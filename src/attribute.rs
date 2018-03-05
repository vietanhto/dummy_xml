use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<T>>>;

#[derive(Debug)]
pub struct Attribute {
    name: String,
    value: String,
    next: Link<Attribute>,
    prev: Link<Attribute>,
}

impl Attribute {
    pub fn new(name: String, value: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Attribute {
            name: name,
            value: value,
            next: None,
            prev: None,
        }))
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn set_value(&mut self, value: String) -> &mut Self {
        self.value = value;
        self
    }

    pub fn next_attribute(&self) -> Option<Ref<Attribute>> {
        self.next.as_ref().map(|attr| attr.borrow())
    }

    pub fn next_attribute_mut(&mut self) -> Option<RefMut<Attribute>> {
        self.next.as_mut().map(|attr| attr.borrow_mut())
    }

    pub fn set_next_attribute(&mut self, attr: Link<Attribute>) {
        self.next = attr;
    }

    pub fn previous_attribute(&self) -> Option<Ref<Attribute>> {
        self.prev.as_ref().map(|attr| attr.borrow())
    }

    pub fn previous_attribute_mut(&mut self) -> Option<RefMut<Attribute>> {
        self.prev.as_mut().map(|attr| attr.borrow_mut())
    }

    pub fn set_previous_attribute(&mut self, attr: Link<Attribute>) {
        self.prev = attr;
    }
}

impl PartialEq for Attribute {
    fn eq(&self, other: &Attribute) -> bool {
        self.name == other.name && self.value == other.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_object_test() {
        let attr = Attribute::new("class".to_string(), "test".to_string());

        //borrow_mut
        {
            let mut attr = attr.borrow_mut();
            assert_eq!(attr.name(), "class");
            assert_eq!(attr.value(), "test");

            attr.set_name("id".to_string());
            attr.set_value("main".to_string());
        }

        //borrow
        {
            let attr2 = attr.clone();
            assert_eq!(attr, attr2);
        }
    }
}
