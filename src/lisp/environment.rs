use error::LispResult as R;
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

    pub fn execute(&mut self, arg: &TArg, execution_tree: ExecutionTree) -> R<String> {
        let execution_tree_root = execution_tree.into_root();
        let result = try!(self.evaluate_node(arg, execution_tree_root));
        result.to_string()
    }

    pub fn get_current_frame(&mut self) -> R<&mut Frame<TArg>> {
        let index_of_last = self.call_stack.len() - 1;
        match self.call_stack.get_mut(index_of_last) {
            Some(some) => Ok(some),
            None => panic!(),
        }
    }

    pub fn parse_and_execute(&mut self, arg: &TArg, source: &str) -> R<String> {
        let ast = AbstractSyntaxTree::new(source);
        {
            try!(ast.initialize());
        }

        let execution_tree = try!(ExecutionTree::new(&ast));

        self.execute(arg, execution_tree)
    }

    fn evaluate(&mut self, arg: &TArg, object: ExecutionTreeObject, rest: Vec<ExecutionTreeObject>) -> R<ExecutionTreeObject> {
        self.call_stack.push(Frame::new());

        let result =
            match object {
                ExecutionTreeObject::Node(some) => {
                    if rest.len() == 0 {
                        self.evaluate_node(arg, some)
                    } else {
                        panic!()
                    }
                 },
                ExecutionTreeObject::Symbol(some) => {
                    let symbol = try!(self.global_frame.get(&some));
                    match symbol {
                        Symbol::Object(some) => self.evaluate(arg, some, rest),
                        Symbol::BuiltInFuncNone(some) => {
                            if rest.len() == 0 {
                                let result = some(arg, self);
                                Ok(result)
                            } else {
                                panic!()
                            }
                        },
                        Symbol::BuiltInFuncInput(some) => Ok(some(arg, self, rest)),
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

    fn evaluate_node(&mut self, arg: &TArg, node: ExecutionTreeNode) -> R<ExecutionTreeObject> {
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
            self.evaluate(arg, first, rest)
        }
    }
}
