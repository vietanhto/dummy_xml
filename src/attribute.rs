use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

#[derive(Debug)]
pub struct Attribute {
    name: String,
    value: String,
    next: Option<Rc<RefCell<Attribute>>>,
    prev: Option<Rc<RefCell<Attribute>>>,
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

    pub fn next(&self) -> Option<Ref<Self>> {
        self.next.as_ref().map(|attr| attr.borrow())
    }

    pub fn set_next(&mut self, value: Option<Rc<RefCell<Self>>>) {
        self.next = value;
    }

    pub fn prev(&self) -> Option<Ref<Self>> {
        self.prev.as_ref().map(|attr| attr.borrow())
    }

    pub fn set_prev(&mut self, value: Option<Rc<RefCell<Self>>>) {
        self.prev = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_object_test() {
        let attr = Attribute::new("class".to_string(), "test".to_string());
        assert_eq!(attr.borrow().name(), "class");
        assert_eq!(attr.borrow().value(), "test");

        let attr2 = Attribute::new("id".to_string(), "main".to_string());

        attr.borrow_mut().set_next(Some(attr2));

        let attr = attr.borrow();
        let attr2 = attr.next().unwrap();

        assert_eq!(attr2.name(), "id");
        assert_eq!(attr2.value(), "main");
    }
}
