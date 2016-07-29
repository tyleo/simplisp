use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use string::error::empty_escape_sequence as string;

#[derive(Debug)]
pub struct EmptyEscapeSequence { }

impl EmptyEscapeSequence {
    pub fn new() -> Self {
        EmptyEscapeSequence { }
    }
}

impl Display for EmptyEscapeSequence {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            string::display_1(),
        )
    }
}

impl Error for EmptyEscapeSequence {
    fn description(&self) -> &str {
        string::description()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
