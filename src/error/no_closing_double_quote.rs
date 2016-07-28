use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use string::error::no_closing_double_quote as string;

#[derive(Debug)]
pub struct NoClosingDoubleQuote {
    text: String,
}

impl NoClosingDoubleQuote {
    pub fn new(text: String) -> Self {
        NoClosingDoubleQuote {
            text: text,
        }
    }
}

impl Display for NoClosingDoubleQuote {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            string::display_1(),
            self.text,
        )
    }
}

impl Error for NoClosingDoubleQuote {
    fn description(&self) -> &str {
        string::description()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
