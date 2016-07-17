use error::LispResult as R;
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
                                _ => panic!(),
                            }
                        },
                        None => panic!(),
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
                        None => panic!(),
                    };
                let len = string.len();
                let last_char =
                    match string.chars().nth(len - 1) {
                        Some(some) => some,
                        None => panic!(),
                    };

                if first_char.is_digit(10) {
                    if len > 5 {
                        Self::visit_six_char_number_string(string)
                    } else if len > 3 {
                        Self::visit_four_char_number_string(string)
                    } else if len > 2 {
                        Self::visit_three_char_number_string(string)
                    } else {
                        Self::visit_two_char_number_string(string)
                    }
                } else {
                    match first_char {
                        '"' => {
                            if len < 2 {
                                panic!();
                            } else if last_char != '"' {
                                panic!();
                            } else {
                                let string = try!(Self::convert_escape_sequences(&string[1..(len - 1)]));
                                 Ok(ExecutionTreeObject::String(string))
                            }
                        },
                        '\'' => {
                            if len < 3 {
                                panic!();
                            } else if last_char != '\'' {
                                panic!();
                            } else {
                                let string = try!(Self::convert_escape_sequences(&string[1..(len - 1)]));
                                if string.len() != 1 {
                                    panic!();
                                } else {
                                    let character =
                                        match string.chars().nth(len - 1) {
                                            Some(some) => some,
                                            None => panic!(),
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
        let len = string.len();
        let first_chars = &string[0..(len - 5)];
        let last_five_chars = &string[(len - 5)..5];

        match last_five_chars {
            "isize" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::ISize(ok)),
                    Err(_) => panic!(),
                }
            },
            "usize" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::USize(ok)),
                    Err(_) => panic!(),
                }
            },
            _ => {
                Self::visit_four_char_number_string(string)
            }
        }
    }

    fn visit_four_char_number_string(string: &str) -> R<ExecutionTreeObject> {
        let len = string.len();
        let first_chars = &string[0..(len - 3)];
        let last_three_chars = &string[(len - 3)..3];

        match last_three_chars {
            "f32" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::F32(ok)),
                    Err(_) => panic!(),
                }
            },
            "f64" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::F64(ok)),
                    Err(_) => panic!(),
                }
            },
            "i16" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::I16(ok)),
                    Err(_) => panic!(),
                }
            },
            "i32" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::I32(ok)),
                    Err(_) => panic!(),
                }
            },
            "i64" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::I64(ok)),
                    Err(_) => panic!(),
                }
            },
            "u16" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::U16(ok)),
                    Err(_) => panic!(),
                }
            },
            "u32" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::U32(ok)),
                    Err(_) => panic!(),
                }
            },
            "u64" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::U64(ok)),
                    Err(_) => panic!(),
                }
            },
            _ => {
                Self::visit_three_char_number_string(string)
            }
        }
    }

    fn visit_three_char_number_string(string: &str) -> R<ExecutionTreeObject> {
        let len = string.len();
        let first_chars = &string[0..(len - 2)];
        let last_two_chars = &string[(len - 2)..2];

        match last_two_chars {
            "i8" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::I8(ok)),
                    Err(_) => panic!(),
                }
            },
            "u8" => {
                match first_chars.parse() {
                    Ok(ok) => Ok(ExecutionTreeObject::U8(ok)),
                    Err(_) => panic!(),
                }
            },
            _ => {
                Self::visit_two_char_number_string(string)
            }
        }
    }

    fn visit_two_char_number_string(string: &str) -> R<ExecutionTreeObject> {
        let does_contain_dot = string.chars().any(|item| item == '.');
        if does_contain_dot {
            match string.parse() {
                Ok(ok) => Ok(ExecutionTreeObject::F64(ok)),
                Err(_) => panic!(),
            }
        } else {
            match string.parse() {
                Ok(ok) => Ok(ExecutionTreeObject::I32(ok)),
                Err(_) => panic!(),
            }
        }
    }
}
