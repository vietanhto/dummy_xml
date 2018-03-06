use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use raw_node::{Node, NodeType};
use std::borrow::{Borrow, BorrowMut};

pub struct Document {
    root: Box<Node>,
}

pub struct Parser {}

#[derive(Debug)]
pub enum ParseXmlError {
    InvalidXml,
}

enum State {
    Start,
    ReadTag,
    ReadTagOpen,
    ReadTagClose,
    ReadAttribute,
    ReadContent,
    End,
}

const LESS_THAN: u8 = '<' as u8;
const GREATER_THAN: u8 = '>' as u8;
const SLASH: u8 = '/' as u8;
const EQUAL: u8 = '=' as u8;
const EXCLAMATION_MARK: u8 = '!' as u8;
const QUESTION_MARK: u8 = '?' as u8;

// ' '     (0x20)    space (SPC)
// '\t'    (0x09)  horizontal tab (TAB)
// '\n'    (0x0a)  newline (LF)
// '\v'    (0x0b)  vertical tab (VT)
// '\f'    (0x0c)  feed (FF)
// '\r'    (0x0d)  carriage return (CR)
#[inline]
fn is_space(c: u8) -> bool {
    c == 0x20 || c == 0x09 || c == 0x0a || c == 0x0b || c == 0x0c || c == 0x0d
}

#[inline]
fn skip_spaces(contents: &[u8], p: &mut usize) {
    while *p < contents.len() && is_space(contents[*p]) {
        *p += 1;
    }
}

impl Document {
    pub fn root(&self) -> &Node {
        self.root.first_child().unwrap()
    }

    pub fn root_mut(&mut self) -> &mut Node {
        self.root.first_child_mut().unwrap()
    }
}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse(&self, contents: &[u8]) -> Result<Document, ParseXmlError> {
        let mut root: Box<Node> = Node::new("root".to_string());
        //parsing scope
        {
            let mut current_parent: Option<&mut Node> = Some(root.borrow_mut());
            let mut state = State::Start;
            let mut i = 0;

            loop {
                state = match state {
                    State::Start => {
                        while i < contents.len() && contents[i] != LESS_THAN {
                            i += 1;
                        }

                        State::ReadTag
                    }
                    State::ReadTag => {
                        i += 1; // skip first '<'
                        if i >= contents.len() {
                            State::End
                        } else if contents[i] == SLASH {
                            i += 1;
                            State::ReadTagClose
                        } else if contents[i] == EXCLAMATION_MARK || contents[i] == QUESTION_MARK {
                            while i < contents.len() && contents[i] != LESS_THAN {
                                i += 1;
                            }
                            State::ReadTag
                        } else {
                            State::ReadTagOpen
                        }
                    }
                    State::ReadTagOpen => {
                        //open tag
                        let start = i;
                        while i < contents.len() && !is_space(contents[i])
                            && contents[i] != GREATER_THAN
                        {
                            i += 1;
                        }
                        let tag_name = String::from_utf8(contents[start..i].to_vec()).unwrap();
                        current_parent = current_parent
                            .take()
                            .map(|old_parent| old_parent.append_child(tag_name));

                        State::ReadAttribute
                    }
                    State::ReadTagClose => {
                        while i < contents.len() && contents[i] != GREATER_THAN {
                            i += 1;
                        }
                        current_parent = current_parent
                            .take()
                            .and_then(|old_parent| old_parent.parent_mut());

                        if i >= contents.len() {
                            State::End
                        } else {
                            match contents[i] {
                                GREATER_THAN => {
                                    i += 1;
                                    skip_spaces(contents, &mut i);
                                    State::ReadTag
                                }
                                _ => State::ReadContent,
                            }
                        }
                    }
                    State::ReadAttribute => {
                        skip_spaces(contents, &mut i);
                        let start = i;
                        while i < contents.len() && !is_space(contents[i])
                            && contents[i] != GREATER_THAN
                        {
                            i += 1;
                        }

                        State::ReadContent
                    }
                    State::ReadContent => {
                        i += 1;
                        skip_spaces(contents, &mut i);

                        let start = i;
                        while i < contents.len() && contents[i] != LESS_THAN {
                            i += 1;
                        }
                        if i > start {
                            current_parent = current_parent.take().map(|node| {
                                let txt = String::from_utf8(contents[start..i].to_vec()).unwrap();
                                node.append_child_by_type(NodeType::PcData).set_value(txt);
                                node
                            });
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
        } //parsing scope - end

        // match root {
        //     Some(root) => Ok(Document { root: root }),
        //     None => Err(ParseXmlError::InvalidXml),
        // }
        Ok(Document { root: root })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;
    use test::Bencher;
    use super::*;

    #[test]
    fn test_parse() {
        let mut f = File::open("./xml/data1.xml").expect("file not found");
        let mut contents = String::new();
        let result = f.read_to_string(&mut contents);
        assert_eq!(result.is_ok(), true);

        let parser = Parser::new();
        let _ = parser.parse(contents.as_bytes());

        assert_eq!('a' as u8, 97u8);
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        let mut f = File::open("./xml/large.xml").expect("file not found");
        let mut contents = String::new();
        let result = f.read_to_string(&mut contents);
        assert_eq!(result.is_ok(), true);

        let parser = Parser::new();
        b.iter(|| {
            let _ = parser.parse(contents.as_bytes());
        });
    }

    #[test]
    fn test_parse_note_xml() {
        let mut f = File::open("./xml/note.xml").expect("file not found");
        let mut contents = String::new();
        let result = f.read_to_string(&mut contents);
        assert_eq!(result.is_ok(), true);

        let parser = Parser::new();
        let result = parser.parse(contents.as_bytes());

        assert_eq!(result.is_ok(), true);
        let doc = result.unwrap();

        let root = doc.root();
        assert_eq!(*root.name(), "note");

        let first = root.first_child().unwrap();
        assert_eq!(*first.name(), "to");
        let to_txt = first.first_child().unwrap();
        assert_eq!(*to_txt.name(), "");
        assert_eq!(*to_txt.value(), "Tove");

        let second = first.next_sibling().unwrap();
        assert_eq!(*second.name(), "from");
        let from_txt = second.first_child().unwrap();
        assert_eq!(*from_txt.name(), "");
        assert_eq!(*from_txt.value(), "Jani");

        let fourth = root.last_child().unwrap();
        assert_eq!(*fourth.name(), "body");
        let body_txt = fourth.first_child().unwrap();
        assert_eq!(*body_txt.name(), "");
        assert_eq!(*body_txt.value(), "Don't forget me this weekend!");

        let third = fourth.previous_sibling().unwrap();
        assert_eq!(*third.name(), "heading");
        let heading_txt = third.first_child().unwrap();
        assert_eq!(*heading_txt.name(), "");
        assert_eq!(*heading_txt.value(), "Reminder");
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
