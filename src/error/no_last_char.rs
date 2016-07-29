use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use string::error::no_last_char as string;

#[derive(Debug)]
pub struct NoLastChar {
    contents: String,
}

impl NoLastChar {
    pub fn new(contents: String) -> Self {
        NoLastChar {
            contents: contents,
        }
    }
}

impl Display for NoLastChar {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            string::display_1(),
            self.contents,
            string::display_2(),
        )
    }
}

impl Error for NoLastChar {
    fn description(&self) -> &str {
        string::description()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
