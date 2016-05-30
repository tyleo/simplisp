use lisp::Environment;
use lisp::ExecutionTreeObject;

pub enum Symbol {
    Object(ExecutionTreeObject),
    BuiltInFuncNone(fn(&mut Environment) -> ExecutionTreeObject),
    BuiltInFuncInput(fn(&mut Environment, Vec<ExecutionTreeObject>) -> ExecutionTreeObject),
}

impl Clone for Symbol {
    fn clone(&self) -> Self {
        match self {
            &Symbol::Object(ref some) => Symbol::Object(some.clone()),
            &Symbol::BuiltInFuncNone(some) => Symbol::BuiltInFuncNone(some),
            &Symbol::BuiltInFuncInput(some) => Symbol::BuiltInFuncInput(some),
        }
    }
}
