use error::EmptyEscapeSequence;
use error::EmptyStringDetected;
use error::InvalidEscapeSequence;
use error::LispError as E;
use error::LispResult as R;
use error::MultipleCharactersInSingleQuotes;
use error::NoCharacterInSingleQuotes;
use error::NoClosingDoubleQuoteInExecutionTree;
use error::NoClosingSingleQuoteInExecutionTree;
use error::NoLastChar;
use error::NumericTokenCannotBeParsed;
use lisp::AbstractSyntaxTree;
use lisp::AbstractSyntaxTreeNode;
use lisp::AbstractSyntaxTreeObject;
use lisp::ExecutionTreeNode;
use lisp::ExecutionTreeObject;
use std::ops::Deref;

pub struct ExecutionTree {
    root: ExecutionTreeNode,
}

impl ExecutionTree {
    pub fn new(syntax_tree: &AbstractSyntaxTree) -> R<Self> {
        let abstract_root = syntax_tree.get_root();
        let root = try!(Self::visit_node(abstract_root.deref()));
        let result =
            ExecutionTree {
                root: root,
            };
        Ok(result)
    }

    pub fn get_root(&self) -> &ExecutionTreeNode {
        &self.root
    }

    pub fn into_root(self) -> ExecutionTreeNode {
        self.root
    }

    fn convert_escape_sequences(string: &str) -> R<String> {
        let mut result = String::with_capacity(string.len());

        let mut chars = string.chars().into_iter();

        while let Some(character) = chars.next() {
            let char_to_push =
                if character == '\\' {
                    match chars.next() {
                        Some(escaped_character) => {
                            match escaped_character {
                                'n' => '\n',
                                'r' => '\r',
                                't' => '\t',
                                '\\' => '\\',
                                '0' => '\0',
                                escaped_character => {
                                    let err = InvalidEscapeSequence::new(escaped_character);
                                    return Err(E::from(err));
                                },
                            }
                        },
                        None => {
                            let err = EmptyEscapeSequence::new();
                            return Err(E::from(err));
                        },
                    }
                } else {
                    character
                };

            result.push(char_to_push);
        }

        Ok(result)
    }

    fn visit_node(current_node: &AbstractSyntaxTreeNode) -> R<ExecutionTreeNode> {
        let mut execution_objects = Vec::new();

        for object in current_node.get_objects() {
            match *object {
                AbstractSyntaxTreeObject::Node(ref inner_node) => {
                    let new_node = try!(Self::visit_node(inner_node));
                    let new_object = ExecutionTreeObject::Node(new_node);
                    execution_objects.push(new_object);
                },
                AbstractSyntaxTreeObject::String(string) => {
                    let new_object = try!(Self::visit_string(string));
                    execution_objects.push(new_object);
                },
            }
        }

        let result = ExecutionTreeNode::new(execution_objects);
        Ok(result)
    }

    fn visit_string(string: &str) -> R<ExecutionTreeObject> {
        match string {
            "true" => Ok(ExecutionTreeObject::Bool(true)),
            "false" => Ok(ExecutionTreeObject::Bool(false)),
            string => {
                let first_char =
                    match string.chars().nth(0) {
                        Some(some) => some,
                        None => {
                            let err = EmptyStringDetected::new(string.to_string());
                            return Err(E::from(err));
                        },
                    };
                let char_len = string.chars().count();
                let byte_len = string.len();
                let last_char =
                    match string.chars().nth(char_len - 1) {
                        Some(some) => some,
                        None => {
                            let err = NoLastChar::new(string.to_string());
                            return Err(E::from(err));
                        },
                    };

                if first_char.is_digit(10) {
                    if char_len > 5 {
                        Self::visit_six_char_number_string(string)
                    } else if char_len > 3 {
                        Self::visit_four_char_number_string(string)
                    } else if char_len > 2 {
                        Self::visit_three_char_number_string(string)
                    } else {
                        Self::visit_two_char_number_string(string)
                    }
                } else {
                    match first_char {
                        '"' => {
                            if char_len < 2 || last_char != '"' {
                                let err = NoClosingDoubleQuoteInExecutionTree::new(string.to_string());
                                return Err(E::from(err));
                            } else {
                                let string = try!(Self::convert_escape_sequences(&string[1..(byte_len - 1)]));
                                 Ok(ExecutionTreeObject::String(string))
                            }
                        },
                        '\'' => {
                            if char_len < 3 {
                                let err = NoCharacterInSingleQuotes::new(string.to_string());
                                return Err(E::from(err));
                            } else if last_char != '\'' {
                                let err = NoClosingSingleQuoteInExecutionTree::new(string.to_string());
                                return Err(E::from(err));
                            } else {
                                let string = try!(Self::convert_escape_sequences(&string[1..(byte_len - 1)]));
                                if string.chars().count() != 1 {
                                    let err = MultipleCharactersInSingleQuotes::new(string);
                                    return Err(E::from(err));
                                } else {
                                    let character =
                                        match string.chars().nth(0) {
                                            Some(some) => some,
                                            None => {
                                                let err = NoCharacterInSingleQuotes::new(string);
                                                return Err(E::from(err));
                                            },
                                        };
                                    Ok(ExecutionTreeObject::Char(character))
                                }
                            }
                        },
                        _ => {
                            Ok(ExecutionTreeObject::Symbol(string.to_string()))
                        },
                    }
                }
            }
        }
    }

    fn visit_six_char_number_string(string: &str) -> R<ExecutionTreeObject> {
        if let Some((last_five_index, _)) = string.char_indices().rev().take(5).nth(4) {
            let (first_chars, last_five_chars) = string.split_at(last_five_index);

            match last_five_chars {
                "isize" => {
                    match first_chars.parse() {
                        Ok(ok) => Ok(ExecutionTreeObject::ISize(ok)),
                        Err(_) => {
                            let err = NumericTokenCannotBeParsed::new(ExecutionTreeObject::isize_str().to_string(), string.to_string());
                            Err(E::from(err))
                        },
                    }
                },
                "usize" => {
                    match first_chars.parse() {
                        Ok(ok) => Ok(ExecutionTreeObject::USize(ok)),
                        Err(_) => {
                            let err = NumericTokenCannotBeParsed::new(ExecutionTreeObject::usize_str().to_string(), string.to_string());
                            Err(E::from(err))
                        },
                    }
                },
                _ => {
                    Self::visit_four_char_number_string(string)
                }
            }
        } else {
            panic!();
        }
    }

    fn visit_four_char_number_string(string: &str) -> R<ExecutionTreeObject> {
        if let Some((last_three_index, _)) = string.char_indices().rev().take(3).nth(2) {
            let (first_chars, last_three_chars) = string.split_at(last_three_index);

            match last_three_chars {
                "f32" => {
                    match first_chars.parse() {
                        Ok(ok) => Ok(ExecutionTreeObject::F32(ok)),
                        Err(_) => {
                            let err = NumericTokenCannotBeParsed::new(ExecutionTreeObject::f32_str().to_string(), string.to_string());
                            Err(E::from(err))
                        },
                    }
                },
                "f64" => {
                    match first_chars.parse() {
                        Ok(ok) => Ok(ExecutionTreeObject::F64(ok)),
                        Err(_) => {
                            let err = NumericTokenCannotBeParsed::new(ExecutionTreeObject::f64_str().to_string(), string.to_string());
                            Err(E::from(err))
                        },
                    }
                },
                "i16" => {
                    match first_chars.parse() {
                        Ok(ok) => Ok(ExecutionTreeObject::I16(ok)),
                        Err(_) => {
                            let err = NumericTokenCannotBeParsed::new(ExecutionTreeObject::i16_str().to_string(), string.to_string());
                            Err(E::from(err))
                        },
                    }
                },
                "i32" => {
                    match first_chars.parse() {
                        Ok(ok) => Ok(ExecutionTreeObject::I32(ok)),
                        Err(_) => {
                            let err = NumericTokenCannotBeParsed::new(ExecutionTreeObject::i32_str().to_string(), string.to_string());
                            Err(E::from(err))
                        },
                    }
                },
                "i64" => {
                    match first_chars.parse() {
                        Ok(ok) => Ok(ExecutionTreeObject::I64(ok)),
                        Err(_) => {
                            let err = NumericTokenCannotBeParsed::new(ExecutionTreeObject::i64_str().to_string(), string.to_string());
                            Err(E::from(err))
                        },
                    }
                },
                "u16" => {
                    match first_chars.parse() {
                        Ok(ok) => Ok(ExecutionTreeObject::U16(ok)),
                        Err(_) => {
                            let err = NumericTokenCannotBeParsed::new(ExecutionTreeObject::u16_str().to_string(), string.to_string());
                            Err(E::from(err))
                        },
                    }
                },
                "u32" => {
                    match first_chars.parse() {
                        Ok(ok) => Ok(ExecutionTreeObject::U32(ok)),
                        Err(_) => {
                            let err = NumericTokenCannotBeParsed::new(ExecutionTreeObject::u32_str().to_string(), string.to_string());
                            Err(E::from(err))
                        },
                    }
                },
                "u64" => {
                    match first_chars.parse() {
                        Ok(ok) => Ok(ExecutionTreeObject::U64(ok)),
                        Err(_) => {
                            let err = NumericTokenCannotBeParsed::new(ExecutionTreeObject::u64_str().to_string(), string.to_string());
                            Err(E::from(err))
                        },
                    }
                },
                _ => {
                    Self::visit_three_char_number_string(string)
                }
            }
        } else {
            panic!();
        }
    }

    fn visit_three_char_number_string(string: &str) -> R<ExecutionTreeObject> {
        if let Some((last_two_index, _)) = string.char_indices().rev().take(2).nth(1) {
            let (first_chars, last_two_chars) = string.split_at(last_two_index);

            match last_two_chars {
                "i8" => {
                    match first_chars.parse() {
                        Ok(ok) => Ok(ExecutionTreeObject::I8(ok)),
                        Err(_) => {
                            let err = NumericTokenCannotBeParsed::new(ExecutionTreeObject::i8_str().to_string(), string.to_string());
                            Err(E::from(err))
                        },
                    }
                },
                "u8" => {
                    match first_chars.parse() {
                        Ok(ok) => Ok(ExecutionTreeObject::U8(ok)),
                        Err(_) => {
                            let err = NumericTokenCannotBeParsed::new(ExecutionTreeObject::u8_str().to_string(), string.to_string());
                            Err(E::from(err))
                        },
                    }
                },
                _ => {
                    Self::visit_two_char_number_string(string)
                }
            }
        } else {
            panic!();
        }
    }

    fn visit_two_char_number_string(string: &str) -> R<ExecutionTreeObject> {
        let does_contain_dot = string.chars().any(|item| item == '.');
        if does_contain_dot {
            match string.parse() {
                Ok(ok) => Ok(ExecutionTreeObject::F64(ok)),
                Err(_) => {
                    let err = NumericTokenCannotBeParsed::new(ExecutionTreeObject::f64_str().to_string(), string.to_string());
                    Err(E::from(err))
                },
            }
        } else {
            match string.parse() {
                Ok(ok) => Ok(ExecutionTreeObject::I32(ok)),
                Err(_) => {
                    let err = NumericTokenCannotBeParsed::new(ExecutionTreeObject::i32_str().to_string(), string.to_string());
                    Err(E::from(err))
                },
            }
        }
    }
}
