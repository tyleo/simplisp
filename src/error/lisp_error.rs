use error::*;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

define_error!(
    #[derive(Debug)]
    pub error LispError {
        suberror BeginningStringInWord,
        suberror EmptyEscapeSequence,
        suberror EmptyStringDetected,
        suberror ErrorWrapper,
        suberror InvalidEscapeSequence,
        suberror InvalidExecutionTreeObjectConversion,
        suberror InvalidPreviousChar,
        suberror MultipleCharactersInSingleQuotes,
        suberror NoCharacterInSingleQuotes,
        suberror NoClosingDoubleQuote,
        suberror NoClosingDoubleQuoteInExecutionTree,
        suberror NoClosingParenthesis,
        suberror NoClosingSingleQuote,
        suberror NoClosingSingleQuoteInExecutionTree,
        suberror NoLastChar,
        suberror NoProgramStartParenthesis,
        suberror NumericTokenCannotBeParsed,
        suberror SymbolNotOnStack
    }
);
