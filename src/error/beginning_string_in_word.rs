use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use string::error::beginning_string_in_word as string;

#[derive(Debug)]
pub struct BeginningStringInWord {
    character: char,
    index: usize
}

impl BeginningStringInWord {
    pub fn new(character: char, index: usize) -> Self {
        BeginningStringInWord {
            character: character,
            index: index,
        }
    }
}

impl Display for BeginningStringInWord {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            string::display_1(),
            self.character,
            string::display_2(),
            self.index,
            string::display_3(),
        )
    }
}

impl Error for BeginningStringInWord {
    fn description(&self) -> &str {
        string::description()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
