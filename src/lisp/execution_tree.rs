use error::*;
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
    pub fn new(syntax_tree: &AbstractSyntaxTree) -> Result<Self> {
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

    fn convert_escape_sequences(string: &str) -> Result<String> {
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
                                    return Err(ErrorKind::InvalidEscapeSequence(escaped_character).into());
                                },
                            }
                        },
                        None => {
                            return Err(ErrorKind::EmptyEscapeSequence.into());
                        },
                    }
                } else {
                    character
                };

            result.push(char_to_push);
        }

        Ok(result)
    }

    fn visit_node(current_node: &AbstractSyntaxTreeNode) -> Result<ExecutionTreeNode> {
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

    fn visit_string(string: &str) -> Result<ExecutionTreeObject> {
        match string {
            "true" => Ok(ExecutionTreeObject::Bool(true)),
            "false" => Ok(ExecutionTreeObject::Bool(false)),
            string => {
                let first_char =
                    match string.chars().nth(0) {
                        Some(some) => some,
                        None => {
                            return Err(ErrorKind::EmptyStringDetected(string.to_string()).into());
                        },
                    };
                let char_len = string.chars().count();
                let byte_len = string.len();
                let last_char =
                    match string.chars().nth(char_len - 1) {
                        Some(some) => some,
                        None => {
                            return Err(ErrorKind::NoLastChar(string.to_string()).into());
                        },
                    };

                if first_char.is_digit(10) ||
                   (first_char == '-' && char_len > 1) ||
                   (first_char == '+' && char_len > 1) {
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
                                Err(ErrorKind::NoClosingDoubleQuoteInExecutionTree(string.to_string()).into())
                            } else {
                                let string = try!(Self::convert_escape_sequences(&string[1..(byte_len - 1)]));
                                 Ok(ExecutionTreeObject::String(string))
                            }
                        },
                        '\'' => {
                            if char_len < 3 {
                                Err(ErrorKind::NoCharacterInSingleQuotes(string.to_string()).into())
                            } else if last_char != '\'' {
                                Err(ErrorKind::NoClosingSingleQuoteInExecutionTree(string.to_string()).into())
                            } else {
                                let string = try!(Self::convert_escape_sequences(&string[1..(byte_len - 1)]));
                                if string.chars().count() != 1 {
                                    Err(ErrorKind::MultipleCharactersInSingleQuotes(string).into())
                                } else {
                                    let character =
                                        match string.chars().nth(0) {
                                            Some(some) => some,
                                            None => {
                                                return Err(ErrorKind::NoCharacterInSingleQuotes(string).into());
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

    fn visit_six_char_number_string(string: &str) -> Result<ExecutionTreeObject> {
        if let Some((last_five_index, _)) = string.char_indices().rev().take(5).nth(4) {
            let (first_chars, last_five_chars) = string.split_at(last_five_index);

            match last_five_chars {
                "isize" => {
                    match first_chars.parse() {
                        Ok(ok) => Ok(ExecutionTreeObject::ISize(ok)),
                        Err(_) => {
                            Err(ErrorKind::NumericTokenCannotBeParsed(ExecutionTreeObject::isize_str().to_string(), string.to_string()).into())
                        },
                    }
                },
                "usize" => {
                    match first_chars.parse() {
                        Ok(ok) => Ok(ExecutionTreeObject::USize(ok)),
                        Err(_) => {
                            Err(ErrorKind::NumericTokenCannotBeParsed(ExecutionTreeObject::usize_str().to_string(), string.to_string()).into())
                        },
                    }
                },
                _ => {
                    Self::visit_four_char_number_string(string)
                }
            }
        } else {
            Err(ErrorKind::Msg("Error creating number from 6 or more characters. Could not find last 5 characters.".to_string()).into())
        }
    }

    fn visit_four_char_number_string(string: &str) -> Result<ExecutionTreeObject> {
        if let Some((last_three_index, _)) = string.char_indices().rev().take(3).nth(2) {
            let (first_chars, last_three_chars) = string.split_at(last_three_index);

        match last_three_chars {
            "f32" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::F32(ok)),
                    Err(_) => {
                        Err(ErrorKind::NumericTokenCannotBeParsed(ExecutionTreeObject::f32_str().to_string(), string.to_string()).into())
                    },
                }
            },
            "f64" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::F64(ok)),
                    Err(_) => {
                        Err(ErrorKind::NumericTokenCannotBeParsed(ExecutionTreeObject::f64_str().to_string(), string.to_string()).into())
                    },
                }
            },
            "i16" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::I16(ok)),
                    Err(_) => {
                        Err(ErrorKind::NumericTokenCannotBeParsed(ExecutionTreeObject::i16_str().to_string(), string.to_string()).into())
                    },
                }
            },
            "i32" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::I32(ok)),
                    Err(_) => {
                        Err(ErrorKind::NumericTokenCannotBeParsed(ExecutionTreeObject::i32_str().to_string(), string.to_string()).into())
                    },
                }
            },
            "i64" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::I64(ok)),
                    Err(_) => {
                        Err(ErrorKind::NumericTokenCannotBeParsed(ExecutionTreeObject::i64_str().to_string(), string.to_string()).into())
                    },
                }
            },
            "u16" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::U16(ok)),
                    Err(_) => {
                        Err(ErrorKind::NumericTokenCannotBeParsed(ExecutionTreeObject::u16_str().to_string(), string.to_string()).into())
                    },
                }
            },
            "u32" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::U32(ok)),
                    Err(_) => {
                        Err(ErrorKind::NumericTokenCannotBeParsed(ExecutionTreeObject::u32_str().to_string(), string.to_string()).into())
                    },
                }
            },
            "u64" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::U64(ok)),
                    Err(_) => {
                        Err(ErrorKind::NumericTokenCannotBeParsed(ExecutionTreeObject::u64_str().to_string(), string.to_string()).into())
                    },
                }
            },
            _ => {
                Self::visit_three_char_number_string(string)
            }
        }
        } else {
            Err(ErrorKind::Msg("Error creating number from 4 or more characters. Could not find last 3 characters.".to_string()).into())
        }
    }

    fn visit_three_char_number_string(string: &str) -> Result<ExecutionTreeObject> {
        if let Some((last_two_index, _)) = string.char_indices().rev().take(2).nth(1) {
            let (first_chars, last_two_chars) = string.split_at(last_two_index);

            match last_two_chars {
                "i8" => {
                    match first_chars.parse() {
                        Ok(ok) => Ok(ExecutionTreeObject::I8(ok)),
                        Err(_) => {
                            Err(ErrorKind::NumericTokenCannotBeParsed(ExecutionTreeObject::i8_str().to_string(), string.to_string()).into())
                        },
                    }
                },
                "u8" => {
                    match first_chars.parse() {
                        Ok(ok) => Ok(ExecutionTreeObject::U8(ok)),
                        Err(_) => {
                            Err(ErrorKind::NumericTokenCannotBeParsed(ExecutionTreeObject::u8_str().to_string(), string.to_string()).into())
                        },
                    }
                },
                _ => {
                    Self::visit_two_char_number_string(string)
                }
            }
        } else {
            Err(ErrorKind::Msg("Error creating number from 3 or more characters. Could not find last 2 characters.".to_string()).into())
        }
    }

    fn visit_two_char_number_string(string: &str) -> Result<ExecutionTreeObject> {
        let does_contain_dot = string.chars().any(|item| item == '.');
        if does_contain_dot {
            match string.parse() {
                Ok(ok) => Ok(ExecutionTreeObject::F64(ok)),
                Err(_) => {
                    Err(ErrorKind::NumericTokenCannotBeParsed(ExecutionTreeObject::f64_str().to_string(), string.to_string()).into())
                },
            }
        } else {
            match string.parse() {
                Ok(ok) => Ok(ExecutionTreeObject::I32(ok)),
                Err(_) => {
                    Err(ErrorKind::NumericTokenCannotBeParsed(ExecutionTreeObject::i32_str().to_string(), string.to_string()).into())
                },
            }
        }
    }
}
