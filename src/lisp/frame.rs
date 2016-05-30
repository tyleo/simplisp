use error::LispResult as R;
use lisp::Symbol;
use std::collections::BTreeMap;

pub struct Frame {
    symbols: BTreeMap<String, Symbol>,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            symbols: BTreeMap::new(),
        }
    }

    pub fn get(&self, symbol: &str) -> R<Symbol> {
        match self.symbols.get(symbol) {
            Some(some) => Ok(some.clone()),
            None => panic!(),
        }
    }

    pub fn insert(&mut self, key: String, value: Symbol) -> Option<Symbol> {
        self.symbols.insert(key, value)
    }
}
