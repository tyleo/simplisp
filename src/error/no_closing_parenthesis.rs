use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use string::error::no_closing_parenthesis as string;

#[derive(Debug)]
pub struct NoClosingParenthesis {
    text: String,
}

impl NoClosingParenthesis {
    pub fn new(text: String) -> Self {
        NoClosingParenthesis {
            text: text,
        }
    }
}

impl Display for NoClosingParenthesis {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            string::display_1(),
            self.text,
        )
    }
}

impl Error for NoClosingParenthesis {
    fn description(&self) -> &str {
        string::description()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
