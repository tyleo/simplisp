use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use string::error::empty_string_detected as string;

#[derive(Debug)]
pub struct EmptyStringDetected {
    contents: String,
}

impl EmptyStringDetected {
    pub fn new(contents: String) -> Self {
        EmptyStringDetected {
            contents: contents,
        }
    }
}

impl Display for EmptyStringDetected {
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

impl Error for EmptyStringDetected {
    fn description(&self) -> &str {
        string::description()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
