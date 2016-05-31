use error::LispResult as R;
use lisp::Environment;
use lisp::ExecutionTreeObject;

pub enum Symbol<TArg> {
    Object(ExecutionTreeObject),
    BuiltInFunc(fn(&TArg, &mut Environment<TArg>, Vec<ExecutionTreeObject>) -> R<ExecutionTreeObject>),
}

impl <TArg> Clone for Symbol<TArg> {
    fn clone(&self) -> Self {
        match self {
            &Symbol::Object(ref some) => Symbol::Object(some.clone()),
            &Symbol::BuiltInFunc(some) => Symbol::BuiltInFunc(some),
        }
    }
}
