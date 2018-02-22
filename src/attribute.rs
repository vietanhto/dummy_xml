#[derive(Debug)]
pub struct Attribute {
    name: String,
    value: String,
    next: Option<Box<Attribute>>,
    prev: Option<Box<Attribute>>,
}

impl Attribute {
    fn new(name: String, value: String) -> Attribute {
        Attribute {
            name: name,
            value: value,
            next: None,
            prev: None,
        }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn value(&self) -> &String {
        &self.value
    }

    fn next(&self) -> &Option<Box<Attribute>> {
        &self.next
    }

    fn set_next(&mut self, value: Option<Box<Attribute>>) {
        self.next = value;
    }

    fn prev(&self) -> &Option<Box<Attribute>> {
        &self.prev
    }

    fn set_prev(&mut self, value: Option<Box<Attribute>>) {
        self.prev = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_object_test() {
        let mut attr = Attribute::new("class".to_string(), "test".to_string());
        assert_eq!(attr.name(), "class");
        assert_eq!(attr.value(), "test");

        let attr2 = Attribute::new("id".to_string(), "main".to_string());

        attr.set_next(Some(Box::new(attr2)));
    }
}
