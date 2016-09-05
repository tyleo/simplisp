use error::*;
use lisp::Environment;
use lisp::ExecutionTreeObject;

pub enum Symbol<TArg> {
    Object(ExecutionTreeObject),
    BuiltInFunc(unsafe fn(&TArg, &mut Environment<TArg>, Vec<&ExecutionTreeObject>) -> Result<ExecutionTreeObject>),
}

impl <TArg> Clone for Symbol<TArg> {
    fn clone(&self) -> Self {
        match self {
            &Symbol::Object(ref some) => Symbol::Object(some.clone()),
            &Symbol::BuiltInFunc(some) => Symbol::BuiltInFunc(some),
        }
    }
}
