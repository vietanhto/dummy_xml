use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Attribute {
    name: String,
    value: String,
}

impl Attribute {
    pub fn new(name: String, value: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Attribute {
            name: name,
            value: value,
        }))
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
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
