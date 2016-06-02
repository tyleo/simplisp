use error::LispResult as R;
use lisp::AbstractSyntaxTreeNode;
use lisp::AbstractSyntaxTreeObject;
use lisp::LastCharType;
use std::cell::Ref;
use std::cell::RefCell;

pub struct AbstractSyntaxTree<'a> {
    program_text: &'a str,
    root: RefCell<Option<AbstractSyntaxTreeNode<'a>>>,
}

impl <'a> AbstractSyntaxTree<'a> {
    pub fn new(program_text: &'a str) -> Self {
        AbstractSyntaxTree {
            program_text: program_text,
            root: RefCell::new(None),
        }
    }

    pub fn get_root(&self) -> R<Ref<AbstractSyntaxTreeNode<'a>>> {
        let root = self.root.borrow();
        if let None = *root {
            panic!();
        }
        let ast_node = Ref::map(root, |item| item.as_ref().unwrap());
        Ok(ast_node)
    }

    pub fn initialize(&'a self) -> R<()> {
        let program_text = self.program_text;

        if program_text.as_bytes().get(0) != Some(&b'(') {
            panic!()
        } else {
            let mut enumerated_text = program_text.chars().zip(0..program_text.len()).into_iter();
            enumerated_text.next();
            let root = try!(Self::parse(&mut enumerated_text, program_text));
            *self.root.borrow_mut() = Some(root);
            Ok(())
        }
    }

    fn parse<TIterator>(enumerated_text: &mut TIterator, program_text: &'a str) -> R<AbstractSyntaxTreeNode<'a>>
        where TIterator: Iterator<Item = (char, usize)> {
        let mut objects = Vec::new();

        let mut current_word_start = None;
        let mut last_char_type = LastCharType::OpenParen;

        while let Some((character, index)) = enumerated_text.next() {
            match character {
                '(' => {
                    match last_char_type {
                        LastCharType::Word => panic!(),
                        LastCharType::Quote => panic!(),
                        _ => { },
                    }

                    let inner_node = try!(Self::parse(enumerated_text, program_text));
                    let node_object = AbstractSyntaxTreeObject::Node(inner_node);
                    objects.push(node_object);

                    last_char_type = LastCharType::CloseParen;
                },
                ')' => {
                    Self::try_end_current_word(&mut current_word_start, index, &mut objects, program_text);
                    return Ok(AbstractSyntaxTreeNode::new(objects));
                },
                character => {
                    if character.is_whitespace() {
                        Self::try_end_current_word(&mut current_word_start, index, &mut objects, program_text);

                        last_char_type = LastCharType::WhiteSpace;
                    } else {
                        match last_char_type {
                            LastCharType::CloseParen => panic!(),
                            LastCharType::Quote => panic!(),
                            _ => { },
                        }

                        match current_word_start {
                            Some(_) => {
                                match character {
                                    '"' => panic!(),
                                    '\'' => panic!(),
                                    _ => { },
                                }

                                last_char_type = LastCharType::Word;
                            },
                            None => {
                                match character {
                                    '"' => {
                                        let string_object = try!(Self::parse_double_quoted_text(index, enumerated_text, program_text));
                                        objects.push(string_object);
                                        last_char_type = LastCharType::Quote;
                                    },
                                    '\'' => {
                                        let string_object = try!(Self::parse_single_quoted_text(index, enumerated_text, program_text));
                                        objects.push(string_object);
                                        last_char_type = LastCharType::Quote;
                                    },
                                    _ => {
                                        current_word_start = Some(index);
                                        last_char_type = LastCharType::Word;
                                    }
                                }
                            },
                        }
                    }
                }
            }
        }

        panic!()
    }

    fn parse_double_quoted_text<TIterator>(start_index: usize, enumerated_text: &mut TIterator, program_text: &'a str) -> R<AbstractSyntaxTreeObject<'a>>
        where TIterator: Iterator<Item = (char, usize)> {
        let mut is_escaped = false;

        while let Some((character, index)) = enumerated_text.next() {
            if character == '"' && !is_escaped {
                let end_index = index + 1;
                let word = &program_text[start_index..end_index];
                let string_object = AbstractSyntaxTreeObject::String(word);
                return Ok(string_object);
            } else if character == '\\' {
                is_escaped = true;
            } else {
                is_escaped = false;
            }
        }

        panic!();
    }

    fn parse_single_quoted_text<TIterator>(start_index: usize, enumerated_text: &mut TIterator, program_text: &'a str) -> R<AbstractSyntaxTreeObject<'a>>
        where TIterator: Iterator<Item = (char, usize)> {
        let mut is_escaped = false;

        while let Some((character, index)) = enumerated_text.next() {
            if character == '\'' && !is_escaped {
                let end_index = index + 1;
                let word = &program_text[start_index..end_index];
                let string_object = AbstractSyntaxTreeObject::String(word);
                return Ok(string_object);
            } else if character == '\\' {
                is_escaped = true;
            } else {
                is_escaped = false;
            }
        }

        panic!();
    }

    fn try_end_current_word(current_word_start_option: &mut Option<usize>, current_word_end: usize, objects: &mut Vec<AbstractSyntaxTreeObject<'a>>, program_text: &'a str) {
        match *current_word_start_option {
            Some(current_word_start) => {
                let current_word = &program_text[current_word_start..current_word_end];
                let string_object = AbstractSyntaxTreeObject::String(current_word);
                objects.push(string_object);
                *current_word_start_option = None;
            },
            None => { },
        }
    }
}
