use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use string::error::symbol_not_on_stack as string;

#[derive(Debug)]
pub struct SymbolNotOnStack {
    symbol: String,
}

impl SymbolNotOnStack {
    pub fn new(symbol: String) -> Self {
        SymbolNotOnStack {
            symbol: symbol,
        }
    }
}

impl Display for SymbolNotOnStack {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            string::display_1(),
            self.symbol,
            string::display_2(),
        )
    }
}

impl Error for SymbolNotOnStack {
    fn description(&self) -> &str {
        string::description()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
