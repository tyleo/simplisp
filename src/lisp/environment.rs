use error::LispResult as R;
use lisp::AbstractSyntaxTree;
use lisp::ExecutionTree;
use lisp::ExecutionTreeNode;
use lisp::ExecutionTreeObject;
use lisp::Frame;
use lisp::Symbol;

pub struct Environment {
    call_stack: Vec<Frame>,
    global_frame: Frame,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            call_stack: Vec::new(),
            global_frame: Frame::new(),
        }
    }

    pub fn execute(&mut self, execution_tree: ExecutionTree) -> R<String> {
        let execution_tree_root = execution_tree.into_root();
        let result = try!(self.evaluate_node(execution_tree_root));
        result.to_string()
    }

    pub fn get_current_frame(&mut self) -> R<&mut Frame> {
        let index_of_last = self.call_stack.len() - 1;
        match self.call_stack.get_mut(index_of_last) {
            Some(some) => Ok(some),
            None => panic!(),
        }
    }

    pub fn parse_and_execute(&mut self, source: &str) -> R<String> {
        let ast = AbstractSyntaxTree::new(source);
        {
            try!(ast.initialize());
        }

        let execution_tree = try!(ExecutionTree::new(&ast));

        self.execute(execution_tree)
    }

    fn evaluate(&mut self, object: ExecutionTreeObject, rest: Vec<ExecutionTreeObject>) -> R<ExecutionTreeObject> {
        self.call_stack.push(Frame::new());

        let result =
            match object {
                ExecutionTreeObject::Node(some) => {
                    if rest.len() == 0 {
                        self.evaluate_node(some)
                    } else {
                        panic!()
                    }
                 },
                ExecutionTreeObject::Symbol(some) => {
                    let symbol = try!(self.global_frame.get(&some));
                    match symbol {
                        Symbol::Object(some) => self.evaluate(some, rest),
                        Symbol::BuiltInFuncNone(some) => {
                            if rest.len() == 0 {
                                let result = some(self);
                                Ok(result)
                            } else {
                                panic!()
                            }
                        },
                        Symbol::BuiltInFuncInput(some) => Ok(some(self, rest)),
                    }
                },
                other => {
                    if rest.len() == 0 {
                        Ok(other)
                    } else {
                        panic!()
                    }
                },
            };

        self.call_stack.pop();
        result
    }

    fn evaluate_node(&mut self, node: ExecutionTreeNode) -> R<ExecutionTreeObject> {
        let objects = node.into_objects();

        if objects.len() == 0 {
            Ok(ExecutionTreeObject::Node(ExecutionTreeNode::new(Vec::new())))
        } else {
            let mut first = objects;
            let rest = first.split_off(1);
            let first =
                match first.into_iter().nth(0) {
                    Some(some) => some,
                    None => panic!(),
                };
            self.evaluate(first, rest)
        }
    }
}
