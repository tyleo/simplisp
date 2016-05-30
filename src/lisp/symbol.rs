use lisp::Environment;
use lisp::ExecutionTreeObject;

pub enum Symbol<TArg> {
    Object(ExecutionTreeObject),
    BuiltInFuncNone(fn(&TArg, &mut Environment<TArg>) -> ExecutionTreeObject),
    BuiltInFuncInput(fn(&TArg, &mut Environment<TArg>, Vec<ExecutionTreeObject>) -> ExecutionTreeObject),
}

impl <TArg> Clone for Symbol<TArg> {
    fn clone(&self) -> Self {
        match self {
            &Symbol::Object(ref some) => Symbol::Object(some.clone()),
            &Symbol::BuiltInFuncNone(some) => Symbol::BuiltInFuncNone(some),
            &Symbol::BuiltInFuncInput(some) => Symbol::BuiltInFuncInput(some),
        }
    }
}
