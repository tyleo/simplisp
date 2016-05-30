use error::LispResult as R;
use lisp::Symbol;
use std::collections::BTreeMap;

pub struct Frame<TArg> {
    symbols: BTreeMap<String, Symbol<TArg>>,
}

impl <TArg> Frame<TArg> {
    pub fn new() -> Self {
        Frame {
            symbols: BTreeMap::new(),
        }
    }

    pub fn get(&self, symbol: &str) -> R<Symbol<TArg>> {
        match self.symbols.get(symbol) {
            Some(some) => Ok(some.clone()),
            None => panic!(),
        }
    }

    pub fn insert(&mut self, key: String, value: Symbol<TArg>) -> Option<Symbol<TArg>> {
        self.symbols.insert(key, value)
    }
}
