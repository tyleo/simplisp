use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use string::error::no_character_in_single_quotes as string;

#[derive(Debug)]
pub struct NoCharacterInSingleQuotes {
    token: String,
}

impl NoCharacterInSingleQuotes {
    pub fn new(token: String) -> Self {
        NoCharacterInSingleQuotes {
            token: token,
        }
    }
}

impl Display for NoCharacterInSingleQuotes {
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

impl Error for NoCharacterInSingleQuotes {
    fn description(&self) -> &str {
        string::description()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
