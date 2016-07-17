use last_char_type::LastCharType;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use string::error::invalid_previous_char as string;

#[derive(Debug)]
pub struct InvalidPreviousChar {
    character: char,
    index: usize,
    previous_char_type: String,
}

impl InvalidPreviousChar {
    pub fn new(character: char, index: usize, previous_char_type: LastCharType) -> Self {
        InvalidPreviousChar {
            character: character,
            index: index,
            previous_char_type: previous_char_type.enum_to_string().to_string(),
        }
    }
}

impl Display for InvalidPreviousChar {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}",
            string::display_1(),
            self.character,
            string::display_2(),
            self.index,
            string::display_3(),
            self.previous_char_type,
            string::display_4(),
        )
    }
}

impl Error for InvalidPreviousChar {
    fn description(&self) -> &str {
        string::description()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
