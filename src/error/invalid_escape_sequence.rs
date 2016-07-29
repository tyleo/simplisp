use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use string::error::invalid_escape_sequence as string;

#[derive(Debug)]
pub struct InvalidEscapeSequence {
    escaped_character: char,
}

impl InvalidEscapeSequence {
    pub fn new(escaped_character: char) -> Self {
        InvalidEscapeSequence {
            escaped_character: escaped_character,
        }
    }
}

impl Display for InvalidEscapeSequence {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            string::display_1(),
            self.escaped_character,
            string::display_2(),
        )
    }
}

impl Error for InvalidEscapeSequence {
    fn description(&self) -> &str {
        string::description()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
