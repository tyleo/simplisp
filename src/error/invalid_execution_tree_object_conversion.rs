use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use string::error::invalid_execution_tree_object_conversion as string;

#[derive(Debug)]
pub struct InvalidExecutionTreeObjectConversion {
    actual: String,
    expected: String,
}

impl InvalidExecutionTreeObjectConversion {
    pub fn new(actual: String, expected: String) -> Self {
        InvalidExecutionTreeObjectConversion {
            actual: actual,
            expected: expected,
        }
    }
}

impl Display for InvalidExecutionTreeObjectConversion {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            string::display_1(),
            self.expected,
            string::display_2(),
            self.actual,
            string::display_3(),
        )
    }
}

impl Error for InvalidExecutionTreeObjectConversion {
    fn description(&self) -> &str {
        string::description()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
