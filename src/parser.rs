use node::{Node, NodeType};
use std::borrow::{Borrow, BorrowMut};
use std::panic;

pub struct Document<'a> {
    root: Box<Node<'a>>,
}

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

impl<'a> Document<'a> {
    pub fn root(&self) -> &Node<'a> {
        self.root.borrow()
    }

    pub fn root_mut(&mut self) -> &mut Node<'a> {
        self.root.borrow_mut()
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Chartype {
    ParsePcData = 1,   // \0, &, \r, <
    ParseAttr = 2,     // \0, &, \r, ', "
    ParseAttrWs = 4,   // \0, &, \r, ', ", \n, tab
    Space = 8,         // \r, \n, space, tab
    ParseCData = 16,   // \0, ], >, \r
    ParseComment = 32, // \0, -, >, \r
    Symbol = 64,       // Any symbol > 127, a-z, A-Z, 0-9, _, :, -, .
    StartSymBol = 128, // Any symbol > 127, a-z, A-Z, _, :
}

const SPACE_AND_CLOSE_SIGN: u8 = Chartype::Space as u8 | Chartype::ParseCData as u8;

const CHARTYPE_TABLE: [u8; 256] = [
    55,  0,   0,   0,   0,   0,   0,   0,      0,   12,  12,  0,   0,   63,  0,   0,   // 0-15
    0,   0,   0,   0,   0,   0,   0,   0,      0,   0,   0,   0,   0,   0,   0,   0,   // 16-31
    8,   0,   6,   0,   0,   0,   7,   6,      0,   0,   0,   0,   0,   96,  64,  0,   // 32-47
    64,  64,  64,  64,  64,  64,  64,  64,     64,  64,  192, 0,   1,   0,   48,  0,   // 48-63
    0,   192, 192, 192, 192, 192, 192, 192,    192, 192, 192, 192, 192, 192, 192, 192, // 64-79
    192, 192, 192, 192, 192, 192, 192, 192,    192, 192, 192, 0,   0,   16,  0,   192, // 80-95
    0,   192, 192, 192, 192, 192, 192, 192,    192, 192, 192, 192, 192, 192, 192, 192, // 96-111
    192, 192, 192, 192, 192, 192, 192, 192,    192, 192, 192, 0, 0, 0, 0, 0,           // 112-127

    192, 192, 192, 192, 192, 192, 192, 192,    192, 192, 192, 192, 192, 192, 192, 192, // 128+
    192, 192, 192, 192, 192, 192, 192, 192,    192, 192, 192, 192, 192, 192, 192, 192,
    192, 192, 192, 192, 192, 192, 192, 192,    192, 192, 192, 192, 192, 192, 192, 192,
    192, 192, 192, 192, 192, 192, 192, 192,    192, 192, 192, 192, 192, 192, 192, 192,
    192, 192, 192, 192, 192, 192, 192, 192,    192, 192, 192, 192, 192, 192, 192, 192,
    192, 192, 192, 192, 192, 192, 192, 192,    192, 192, 192, 192, 192, 192, 192, 192,
    192, 192, 192, 192, 192, 192, 192, 192,    192, 192, 192, 192, 192, 192, 192, 192,
    192, 192, 192, 192, 192, 192, 192, 192,    192, 192, 192, 192, 192, 192, 192, 192
];

macro_rules! skip_chartype {
    ($contents: ident, $p: ident, $chartype: expr) => {
        while CHARTYPE_TABLE[$contents[$p] as usize] & $chartype as u8 > 0 {
            $p += 1;
        }
    };
}

macro_rules! skip_chartype_safe {
    ($contents: ident, $p: ident, $size: ident, $chartype: expr) => {
        while $p < $size && CHARTYPE_TABLE[$contents[$p] as usize] & $chartype as u8 > 0 {
            $p += 1;
        }
    };
}

macro_rules! scan_chartype {
    ($contents: ident, $p: ident, $chartype: expr) => {
        while CHARTYPE_TABLE[$contents[$p] as usize] & $chartype as u8 == 0 {
            $p += 1;
        }
    };
}

macro_rules! scan_char {
    ($contents: ident, $p: ident, $char: ident) => {
        while $contents[$p] != $char {
            $p += 1;
        }
    };
}

pub fn parse_str(contents: &str) -> Result<Document, ParseXmlError> {
    parse(contents.as_bytes())
}

pub fn parse_string(contents: &String) -> Result<Document, ParseXmlError> {
    parse(contents.as_bytes())
}

pub fn parse(contents: &[u8]) -> Result<Document, ParseXmlError> {
    let mut root: Box<Node> = Node::new("".to_string());

    let result = panic::catch_unwind(move || {
        parse_internal(contents, root.borrow_mut());
        root
    });

    match result {
        Ok(root) => Ok(Document { root: root }),
        Err(_) => Err(ParseXmlError::InvalidXml),
    }
}

fn parse_internal(contents: &[u8], root: &mut Node) {
    let mut current_parent: Option<&mut Node> = Some(root);
    let mut state = State::Start;
    let mut i = 0;
    let size = contents.len();

    loop {
        state = match state {
            State::Start => {
                scan_char!(contents, i, LESS_THAN);
                State::ReadTag
            }
            State::ReadTag => {
                i += 1; // skip first '<'
                if i >= size {
                    State::End
                } else {
                    match contents[i] {
                        SLASH => {
                            i += 1;
                            State::ReadTagClose
                        }
                        EXCLAMATION_MARK | QUESTION_MARK => {
                            scan_char!(contents, i, LESS_THAN);
                            State::ReadTag
                        }
                        _ => State::ReadTagOpen,
                    }
                }
            }
            State::ReadTagOpen => {
                let start = i;
                scan_chartype!(contents, i, SPACE_AND_CLOSE_SIGN);

                let tag_name = String::from_utf8(contents[start..i].to_vec()).unwrap();
                current_parent = current_parent.take().map(|old_parent| {
                    if old_parent.name().len() == 0 {
                        old_parent.set_name(tag_name)
                    } else {
                        old_parent.append_child(tag_name)
                    }
                });

                State::ReadAttribute
            }
            State::ReadTagClose => {
                scan_char!(contents, i, GREATER_THAN);
                current_parent = current_parent
                    .take()
                    .and_then(|old_parent| old_parent.parent_mut());

                match contents[i] {
                    GREATER_THAN => {
                        i += 1;
                        skip_chartype_safe!(contents, i, size, Chartype::Space);
                        State::ReadTag
                    }
                    _ => State::ReadContent,
                }
            }
            State::ReadAttribute => {
                skip_chartype!(contents, i, Chartype::Space);
                match contents[i] {
                    SLASH => {
                        i += 1;
                        current_parent = current_parent
                            .take()
                            .and_then(|old_parent| old_parent.parent_mut());
                        State::ReadContent
                    }
                    GREATER_THAN => State::ReadContent,
                    _ => {
                        let start = i;
                        scan_char!(contents, i, EQUAL);
                        let end = i;
                        i += 1; //skip =
                        let quote = contents[i];
                        i += 1;
                        let value_start = i;
                        scan_char!(contents, i, quote);
                        let name = String::from_utf8(contents[start..end].to_vec()).unwrap();
                        let value = String::from_utf8(contents[value_start..i].to_vec()).unwrap();
                        current_parent
                            .as_mut()
                            .map(|node| node.append_attribute(name, value));
                        i += 1;
                        State::ReadAttribute
                    }
                }
            }
            State::ReadContent => {
                i += 1;
                skip_chartype!(contents, i, Chartype::Space);

                let start = i;
                scan_char!(contents, i, LESS_THAN);
                if i > start {
                    current_parent = current_parent.take().map(|node| {
                        let txt = String::from_utf8(contents[start..i].to_vec()).unwrap();
                        node.append_child_by_type(NodeType::PcData).set_value(txt);
                        node
                    });
                }
                State::ReadTag
            }
            State::End => {
                break;
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use std::borrow::Borrow;

    use node::Attribute;

    use super::*;

    #[test]
    fn test_parse() {
        let mut f = File::open("./xml/data1.xml").expect("file not found");
        let mut contents = String::new();
        let result = f.read_to_string(&mut contents);
        assert_eq!(result.is_ok(), true);

        let _ = parse(contents.as_bytes());

        assert_eq!('a' as u8, 97u8);
    }

    #[test]
    fn bench_parse() {
        let mut f = File::open("./xml/large.xml").expect("file not found");
        let mut contents = String::new();
        let result = f.read_to_string(&mut contents);
        assert_eq!(result.is_ok(), true);

        let _ = parse(contents.as_bytes());
    }

    #[test]
    fn test_parse_note_xml() {
        let mut f = File::open("./xml/note.xml").expect("file not found");
        let mut contents = String::new();
        let result = f.read_to_string(&mut contents);
        assert_eq!(result.is_ok(), true);

        let result = parse(contents.as_bytes());

        assert_eq!(result.is_ok(), true);
        let doc = result.unwrap();

        let root = doc.root();
        assert_eq!(root.name(), "note");
        assert_eq!(
            root.first_attribute().unwrap(),
            Attribute::new("id".to_string(), "1".to_string()).borrow()
        );
        assert_eq!(root.parent().is_none(), true);

        let first = root.first_child().unwrap();
        assert_eq!(first.name(), "to");
        let to_txt = first.first_child().unwrap();
        assert_eq!(to_txt.name(), "");
        assert_eq!(to_txt.value(), "Tove");

        let second = first.next_sibling().unwrap();
        assert_eq!(second.name(), "from");
        assert_eq!(
            second.first_attribute().unwrap(),
            Attribute::new("value".to_string(), "Jani".to_string()).borrow()
        );

        let fourth = root.last_child().unwrap();
        assert_eq!(fourth.name(), "body");
        let body_txt = fourth.first_child().unwrap();
        assert_eq!(body_txt.name(), "");
        assert_eq!(body_txt.value(), "Don't forget me this weekend!");

        let third = fourth.previous_sibling().unwrap();
        assert_eq!(third.name(), "heading");
        let heading_txt = third.first_child().unwrap();
        assert_eq!(heading_txt.name(), "");
        assert_eq!(heading_txt.value(), "Reminder");
    }
}
