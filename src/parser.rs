use std::collections::LinkedList;

use document::Document;
use node::Node;

pub struct Parser {}

enum State {
    Start,
    ReadTag,
    ReadAttribute,
    ReadContent,
    End,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse(&self, contents: &[u8]) -> Document {
        let mut tag_stack: LinkedList<Node> = LinkedList::new();
        let mut state = State::Start;
        let mut i = 0;

        const LESS_THAN: u8 = '<' as u8;
        const GREATER_THAN: u8 = '>' as u8;
        const SLASH: u8 = '/' as u8;
        const EQUAL: u8 = '=' as u8;

        loop {
            state = match state {
                State::Start => {
                    while i < contents.len() && contents[i] != LESS_THAN {
                        i += 1;
                    }

                    if i >= contents.len() {
                        break;
                    }

                    State::ReadTag
                }
                State::ReadTag => {
                    i += 1; // skip first '<'
                    if contents[i] == SLASH {
                        //end tag
                        while i < contents.len() && !is_space(contents[i])
                            && contents[i] != GREATER_THAN
                        {
                            i += 1;
                        }

                        State::ReadTag
                    } else {
                        //open tag
                        let start = i;
                        while i < contents.len() && !is_space(contents[i])
                            && contents[i] != GREATER_THAN
                        {
                            i += 1;
                        }
                        let tag_name = String::from_utf8(contents[start..i].to_vec()).unwrap();
                        let tag = Node::new(tag_name);
                        tag_stack.push_back(tag);
                        println!("{:?}", tag_stack.back());

                        State::ReadAttribute
                    }
                }
                State::ReadAttribute => {
                    while i < contents.len() && is_space(contents[i]) {
                        i += 1;
                    }
                    while i < contents.len() && !is_space(contents[i])
                        && contents[i] != GREATER_THAN
                        && contents[i] != EQUAL
                    {
                        i += 1;
                    }

                    State::ReadContent
                }
                State::ReadContent => {
                    while i < contents.len() && contents[i] != LESS_THAN {
                        i += 1;
                    }

                    if i >= contents.len() {
                        State::End
                    } else {
                        State::ReadTag
                    }
                }
                State::End => {
                    break;
                }
            };
        }
        Document::new()
    }
}

// ' '     (0x20)    space (SPC)
// '\t'    (0x09)  horizontal tab (TAB)
// '\n'    (0x0a)  newline (LF)
// '\v'    (0x0b)  vertical tab (VT)
// '\f'    (0x0c)  feed (FF)
// '\r'    (0x0d)  carriage return (CR)
fn is_space(c: u8) -> bool {
    c == 0x20 || c == 0x09 || c == 0x0a || c == 0x0b || c == 0x0c || c == 0x0d
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;
    use time::PreciseTime;
    use super::*;

    #[test]
    fn test_parse() {
        let mut f = File::open("./xml/data1.xml").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents);

        let start = PreciseTime::now();
        let parser = Parser::new();
        let doc = parser.parse(contents.as_bytes());
        let end = PreciseTime::now();

        println!("{} seconds", start.to(end));
        assert_eq!('a' as u8, 97u8);
    }

    #[test]
    fn test_is_space() {
        assert_eq!(is_space('c' as u8), false);
        assert_eq!(is_space(' ' as u8), true);
        assert_eq!(is_space('\t' as u8), true);
        assert_eq!(is_space('\n' as u8), true);
        assert_eq!(is_space(0x0b), true);
        assert_eq!(is_space(0x0c), true);
        assert_eq!(is_space('\r' as u8), true);
    }
}
