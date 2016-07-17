use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct ErrorWrapper {
    cause: Box<Error>
}

impl ErrorWrapper {
    pub fn new<TError>(cause: TError) -> Self
        where TError: Error + 'static {
        ErrorWrapper {
            cause: Box::new(cause),
        }
    }
}

impl Display for ErrorWrapper {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.cause.fmt(f)
    }
}

impl Error for ErrorWrapper {
    fn description(&self) -> &str {
        self.cause.description()
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.cause()
    }
}
