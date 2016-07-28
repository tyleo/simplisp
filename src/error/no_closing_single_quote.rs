use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use string::error::no_closing_single_quote as string;

#[derive(Debug)]
pub struct NoClosingSingleQuote {
    text: String,
}

impl NoClosingSingleQuote {
    pub fn new(text: String) -> Self {
        NoClosingSingleQuote {
            text: text,
        }
    }
}

impl Display for NoClosingSingleQuote {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            string::display_1(),
            self.text,
        )
    }
}

impl Error for NoClosingSingleQuote {
    fn description(&self) -> &str {
        string::description()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
