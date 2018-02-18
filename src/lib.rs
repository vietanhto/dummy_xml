mod attribute;
mod node;
mod document;
mod parser;

use attribute::Attribute;
use node::Node;
use document::Document;

extern crate time;

fn parse(contents: String) -> Document {
    let mut result = Document::new();

    let mut head = 0;
    let mut tail = 0;
    let mut action = false;
    for (i, c) in contents.chars().enumerate() {
        match c {
            '<' => {
                head = i + 1;
            },
            '>' => {
                tail = i;
                action = true;
            },
            _ => (),
        }
        if action {
            let tag_content = &contents[head..tail];
            action = false;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;
    use time::PreciseTime;
    use super::parse;

    #[test]
    fn it_works() {
        let mut f = File::open("./xml/data1.xml").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents);

        let start = PreciseTime::now();
        parse(contents);
        let end = PreciseTime::now();

        println!("{} seconds", start.to(end));
        assert_eq!(2 + 2, 4);
    }
}
