use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use string::error::multiple_characters_in_single_quotes as string;

#[derive(Debug)]
pub struct MultipleCharactersInSingleQuotes {
    token: String,
}

impl MultipleCharactersInSingleQuotes {
    pub fn new(token: String) -> Self {
        MultipleCharactersInSingleQuotes {
            token: token,
        }
    }
}

impl Display for MultipleCharactersInSingleQuotes {
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

impl Error for MultipleCharactersInSingleQuotes {
    fn description(&self) -> &str {
        string::description()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
