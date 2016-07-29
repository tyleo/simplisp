use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use string::error::numeric_token_cannot_be_parsed as string;

#[derive(Debug)]
pub struct NumericTokenCannotBeParsed {
    expected_type: String,
    token: String,
}

impl NumericTokenCannotBeParsed {
    pub fn new(expected_type: String, token: String) -> Self {
        NumericTokenCannotBeParsed {
            expected_type: expected_type,
            token: token,
        }
    }
}

impl Display for NumericTokenCannotBeParsed {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            string::display_1(),
            self.token,
            string::display_2(),
            self.expected_type,
            string::display_3(),
        )
    }
}

impl Error for NumericTokenCannotBeParsed {
    fn description(&self) -> &str {
        string::description()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
