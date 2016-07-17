use error::*;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

define_error!(
    #[derive(Debug)]
    pub error LispError {
        suberror BeginningStringInWord,
        suberror ErrorWrapper,
        suberror InvalidPreviousChar
    }
);
