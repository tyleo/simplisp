use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use string::error::no_program_start_parenthesis as string;

#[derive(Debug)]
pub struct NoProgramStartParenthesis {
    text: String,
}

impl NoProgramStartParenthesis {
    pub fn new(text: String) -> Self {
        NoProgramStartParenthesis {
            text: text,
        }
    }
}

impl Display for NoProgramStartParenthesis {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            string::display_1(),
            self.text,
        )
    }
}

impl Error for NoProgramStartParenthesis {
    fn description(&self) -> &str {
        string::description()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
