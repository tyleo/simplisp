use error::LispError as E;
use error::LispResult as R;
use error::SymbolNotOnStack;
use lisp::AbstractSyntaxTree;
use lisp::ExecutionTree;
use lisp::ExecutionTreeNode;
use lisp::ExecutionTreeObject;
use lisp::Frame;
use lisp::Symbol;

pub struct Environment<TArg> {
    call_stack: Vec<Frame<TArg>>,
    global_frame: Frame<TArg>,
}

impl <TArg> Environment<TArg> {
    pub fn new() -> Self {
        Environment {
            call_stack: Vec::new(),
            global_frame: Frame::new(),
        }
    }

    pub unsafe fn execute(&mut self, arg: &TArg, execution_tree: ExecutionTree) -> R<String> {
        let execution_tree_root = execution_tree.into_root();
        let execution_tree_root_object = ExecutionTreeObject::Node(execution_tree_root);
        let result = try!(self.evaluate(arg, execution_tree_root_object));
        result.to_string()
    }

    pub fn get_global_frame(&mut self) -> &mut Frame<TArg> {
        &mut self.global_frame
    }

    pub unsafe fn parse_and_execute(&mut self, arg: &TArg, source: &str) -> R<String> {
        let ast = try!(AbstractSyntaxTree::new(source));

        let execution_tree = try!(ExecutionTree::new(&ast));

        self.execute(arg, execution_tree)
    }

    pub unsafe fn evaluate(&mut self, arg: &TArg, object: ExecutionTreeObject) -> R<ExecutionTreeObject> {
        self.call_stack.push(Frame::new());

        let result =
            match object {
                ExecutionTreeObject::Node(node) => {
                    let inner_objects = node.into_objects();
                    let inner_objects_len = inner_objects.len();
                    if inner_objects_len > 1 {
                        self.evaluate_list(arg, inner_objects)
                    } else {
                        if let Some((first, _)) = Self::split(inner_objects.into_iter()) {
                            self.evaluate(arg, first)
                        } else {
                            let nil = ExecutionTreeObject::nil();
                            Ok(nil)
                        }
                    }
                },

                ExecutionTreeObject::Symbol(symbol) => {
                    let symbol =
                        match self.global_frame.try_get(&symbol) {
                            Some(symbol) => symbol,
                            None => try!(self.symbol_stack_search(&symbol)),
                        };
                    match symbol {
                        Symbol::BuiltInFunc(func) => {
                            let empty_args = Vec::new();
                            func(arg, self, empty_args)
                        },
                        Symbol::Object(object) => Ok(object),
                    }
                }

                other => Ok(other),
            };

        self.call_stack.pop();
        result
    }

    unsafe fn evaluate_list(&mut self, arg: &TArg, list: Vec<ExecutionTreeObject>) -> R<ExecutionTreeObject> {
        self.call_stack.push(Frame::new());
        let size = list.len();

        let result =
            if let Some((first, rest)) = Self::split(list.into_iter()) {
                match first {
                    ExecutionTreeObject::Node(node) => {
                        let mut result = Vec::with_capacity(size);
                        let node_object = ExecutionTreeObject::Node(node);
                        result.push(try!(self.evaluate(arg, node_object)));
                        
                        for object in rest {
                            result.push(try!(self.evaluate(arg, object)));
                        }
                        Ok(ExecutionTreeObject::Node(ExecutionTreeNode::new(result)))
                    },

                    ExecutionTreeObject::Symbol(symbol) => {
                        let symbol =
                            match self.global_frame.try_get(&symbol) {
                                Some(symbol) => symbol,
                                None => try!(self.symbol_stack_search(&symbol)),
                            };
                        match symbol {
                            Symbol::BuiltInFunc(func) => {
                                func(arg, self, rest.collect())
                            },

                            Symbol::Object(object) => {
                                let mut result = Vec::with_capacity(size);
                                result.push(object);
                                for object in rest {
                                    result.push(try!(self.evaluate(arg, object)));
                                }
                                Ok(ExecutionTreeObject::Node(ExecutionTreeNode::new(result)))
                            },
                        }
                    }

                    other => Ok(other),
                }
            } else {
                Ok(ExecutionTreeObject::Node(ExecutionTreeNode::new(Vec::new())))
            };

        self.call_stack.pop();
        result
    }

    fn split<TObjects>(mut objects: TObjects) -> Option<(ExecutionTreeObject, TObjects)>
        where TObjects: Iterator<Item = ExecutionTreeObject> {
        if let Some(first) = objects.next() {
            Some((first, objects))
        } else {
            None
        }
    }

    fn symbol_stack_search(&self, symbol: &str) -> R<Symbol<TArg>> {
        let call_stack = &self.call_stack;
        for stack_frame in call_stack.into_iter().rev() {
            if let Some(symbol) = stack_frame.try_get(&symbol) {
                return Ok(symbol);
            }
        }

        let err = SymbolNotOnStack::new(symbol.to_string());
        Err(E::from(err))
    }
}
