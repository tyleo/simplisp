use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use string::error::no_closing_single_quote_in_execution_tree as string;

#[derive(Debug)]
pub struct NoClosingSingleQuoteInExecutionTree {
    token: String,
}

impl NoClosingSingleQuoteInExecutionTree {
    pub fn new(token: String) -> Self {
        NoClosingSingleQuoteInExecutionTree {
            token: token,
        }
    }
}

impl Display for NoClosingSingleQuoteInExecutionTree {
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

impl Error for NoClosingSingleQuoteInExecutionTree {
    fn description(&self) -> &str {
        string::description()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
