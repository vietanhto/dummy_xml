use document::Document;

pub struct Parser {

}

impl Parser {
    pub fn new() -> Parser {
        Parser{}
    }

    pub fn parse(&self, contents: String) -> Document {
        Document::new()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;
    use time::PreciseTime;
    use super::*;

    #[test]
    fn it_works() {
        let mut f = File::open("./xml/data1.xml").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents);

        let start = PreciseTime::now();
        let parser = Parser::new();
        let doc = parser.parse(contents);
        let end = PreciseTime::now();

        println!("{} seconds", start.to(end));
        assert_eq!(2 + 2, 4);
    }
}