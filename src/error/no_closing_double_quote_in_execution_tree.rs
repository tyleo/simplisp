use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use string::error::no_closing_double_quote_in_execution_tree as string;

#[derive(Debug)]
pub struct NoClosingDoubleQuoteInExecutionTree {
    token: String,
}

impl NoClosingDoubleQuoteInExecutionTree {
    pub fn new(token: String) -> Self {
        NoClosingDoubleQuoteInExecutionTree {
            token: token,
        }
    }
}

impl Display for NoClosingDoubleQuoteInExecutionTree {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            string::display_1(),
            self.token,
            string::display_2(),
        )
    }
}

impl Error for NoClosingDoubleQuoteInExecutionTree {
    fn description(&self) -> &str {
        string::description()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
