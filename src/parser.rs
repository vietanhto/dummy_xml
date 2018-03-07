use node::{Node, NodeType};
use std::borrow::{Borrow, BorrowMut};

pub struct Document {
    root: Box<Node>,
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

impl Document {
    pub fn root(&self) -> &Node {
        self.root.borrow()
    }

    pub fn root_mut(&mut self) -> &mut Node {
        self.root.borrow_mut()
    }
}

#[derive(Clone, Copy)]
enum Chartype {
    // ParsePcData = 1,   // \0, &, \r, <
    // ParseAttr = 2,     // \0, &, \r, ', "
    // ParseAttrWs = 4,   // \0, &, \r, ', ", \n, tab
    Space = 8, // \r, \n, space, tab
               // ParseCData = 16, // \0, ], >, \r
               // ParseComment = 32, // \0, -, >, \r
               // Symbol = 64,       // Any symbol > 127, a-z, A-Z, 0-9, _, :, -, .
               // StartSymBol = 128, // Any symbol > 127, a-z, A-Z, _, :
}

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
        while $p < $contents.len()
            && (CHARTYPE_TABLE[$contents[$p] as usize] & $chartype as u8 > 0)
        {
            $p += 1;
        }
    };
}

#[inline]
fn is_chartype(c: u8, chartype: Chartype) -> bool {
    CHARTYPE_TABLE[c as usize] & chartype as u8 > 0
}

// ' '     (0x20)    space (SPC)
// '\t'    (0x09)  horizontal tab (TAB)
// '\n'    (0x0a)  newline (LF)
// '\v'    (0x0b)  vertical tab (VT)
// '\f'    (0x0c)  feed (FF)
// '\r'    (0x0d)  carriage return (CR)
#[inline]
fn is_space(c: u8) -> bool {
    is_chartype(c, Chartype::Space)
}

pub fn parse_string(contents: &String) -> Result<Document, ParseXmlError> {
    parse(contents.as_bytes())
}

pub fn parse(contents: &[u8]) -> Result<Document, ParseXmlError> {
    let mut root: Box<Node> = Node::new("".to_string());
    parse_internal(contents, root.borrow_mut());
    if root.name().len() > 0 {
        Ok(Document { root: root })
    } else {
        Err(ParseXmlError::InvalidXml)
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
                while i < size && contents[i] != LESS_THAN {
                    i += 1;
                }

                State::ReadTag
            }
            State::ReadTag => {
                i += 1; // skip first '<'
                if i >= size {
                    State::End
                } else if contents[i] == SLASH {
                    i += 1;
                    State::ReadTagClose
                } else if contents[i] == EXCLAMATION_MARK || contents[i] == QUESTION_MARK {
                    while i < size && contents[i] != LESS_THAN {
                        i += 1;
                    }
                    State::ReadTag
                } else {
                    State::ReadTagOpen
                }
            }
            State::ReadTagOpen => {
                let start = i;
                while i < size && !is_space(contents[i]) && contents[i] != GREATER_THAN {
                    i += 1;
                }
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
                while i < size && contents[i] != GREATER_THAN {
                    i += 1;
                }
                current_parent = current_parent
                    .take()
                    .and_then(|old_parent| old_parent.parent_mut());

                if i >= size {
                    State::End
                } else {
                    match contents[i] {
                        GREATER_THAN => {
                            i += 1;
                            skip_chartype!(contents, i, Chartype::Space);
                            State::ReadTag
                        }
                        _ => State::ReadContent,
                    }
                }
            }
            State::ReadAttribute => {
                skip_chartype!(contents, i, Chartype::Space);
                let start = i;
                while i < size && !is_space(contents[i]) && contents[i] != GREATER_THAN
                    && contents[i] != EQUAL
                {
                    i += 1;
                }

                if i == start {
                    State::ReadContent
                } else if contents[i] != EQUAL {
                    //no-value attr
                    let name = String::from_utf8(contents[start..i].to_vec()).unwrap();
                    current_parent
                        .as_mut()
                        .map(|node| node.append_attribute(name, String::from("")));
                    State::ReadAttribute
                } else {
                    let end = i;
                    i += 1; //skip =
                    let quote = contents[i];
                    i += 1;
                    let value_start = i;
                    while i < size && contents[i] != quote {
                        i += 1;
                    }
                    let name = String::from_utf8(contents[start..end].to_vec()).unwrap();
                    let value = String::from_utf8(contents[value_start..i].to_vec()).unwrap();
                    current_parent
                        .as_mut()
                        .map(|node| node.append_attribute(name, value));
                    i += 1;
                    State::ReadAttribute
                }
            }
            State::ReadContent => {
                i += 1;
                skip_chartype!(contents, i, Chartype::Space);

                let start = i;
                while i < size && contents[i] != LESS_THAN {
                    i += 1;
                }
                if i > start {
                    current_parent = current_parent.take().map(|node| {
                        let txt = String::from_utf8(contents[start..i].to_vec()).unwrap();
                        node.append_child_by_type(NodeType::PcData).set_value(txt);
                        node
                    });
                }
                if i >= size {
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
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use std::borrow::Borrow;

    use test::Bencher;

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

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        let mut f = File::open("./xml/large.xml").expect("file not found");
        let mut contents = String::new();
        let result = f.read_to_string(&mut contents);
        assert_eq!(result.is_ok(), true);

        b.iter(|| {
            let _ = parse(contents.as_bytes());
        });
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
        assert_eq!(*root.name(), "note");
        assert_eq!(
            root.first_attribute().unwrap(),
            Attribute::new("id".to_string(), "1".to_string()).borrow()
        );
        assert_eq!(root.parent().is_none(), true);

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
        assert_eq!(is_space('\r' as u8), true);
    }
}
