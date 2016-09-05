use error::*;
use lisp::AbstractSyntaxTreeNode;
use lisp::AbstractSyntaxTreeObject;
use lisp::LastCharType;

pub struct AbstractSyntaxTree<'a> {
    root: AbstractSyntaxTreeNode<'a>,
}

impl <'a> AbstractSyntaxTree<'a> {
    pub fn new(program_text: &'a str) -> Result<Self> {
        let result =
            AbstractSyntaxTree {
                root: try!(Self::parse_program_text(program_text)),
            };
        Ok(result)
    }

    pub fn get_root(&self) -> &AbstractSyntaxTreeNode<'a> {
        &self.root
    }

    fn parse<TIterator>(enumerated_text: &mut TIterator, program_text: &'a str) -> Result<AbstractSyntaxTreeNode<'a>>
        where TIterator: Iterator<Item = (usize, char)> {
        let mut objects = Vec::new();

        let mut current_word_start = None;
        let mut last_char_type = LastCharType::OpenParen;

        while let Some((index, character)) = enumerated_text.next() {
            match character {
                '(' => {
                    match last_char_type {
                        LastCharType::Word | LastCharType::Quote => {
                            return Err(ErrorKind::InvalidPreviousChar(character, index, last_char_type).into());
                        },
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
                            LastCharType::CloseParen | LastCharType::Quote => {
                                return Err(ErrorKind::InvalidPreviousChar(character, index, last_char_type).into());
                            },
                            _ => { },
                        }

                        match current_word_start {
                            Some(_) => {
                                match character {
                                    '"' | '\'' => {
                                        return Err(ErrorKind::BeginningStringInWord(character, index).into());
                                    },
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

        Err(ErrorKind::NoClosingParenthesis(program_text.to_string()).into())
    }

    fn parse_double_quoted_text<TIterator>(start_index: usize, enumerated_text: &mut TIterator, program_text: &'a str) -> Result<AbstractSyntaxTreeObject<'a>>
        where TIterator: Iterator<Item = (usize, char)> {
        let mut is_escaped = false;

        while let Some((index, character)) = enumerated_text.next() {
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

        Err(ErrorKind::NoClosingDoubleQuote(program_text.to_string()).into())
    }

    pub fn parse_program_text(program_text: &'a str) -> Result<AbstractSyntaxTreeNode<'a>> {
        if program_text.as_bytes().get(0) != Some(&b'(') {
            Err(ErrorKind::NoProgramStartParenthesis(program_text.to_string()).into())
        } else {
            let mut enumerated_text = program_text.char_indices();
            enumerated_text.next();
            let root = try!(Self::parse(&mut enumerated_text, program_text));
            Ok(root)
        }
    }

    fn parse_single_quoted_text<TIterator>(start_index: usize, enumerated_text: &mut TIterator, program_text: &'a str) -> Result<AbstractSyntaxTreeObject<'a>>
        where TIterator: Iterator<Item = (usize, char)> {
        let mut is_escaped = false;

        while let Some((index, character)) = enumerated_text.next() {
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

        Err(ErrorKind::NoClosingSingleQuote(program_text.to_string()).into())
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
