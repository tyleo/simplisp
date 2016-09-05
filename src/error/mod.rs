use last_char_type::LastCharType;
use std::error::Error as StdError;

error_chain! {
    types { }

    links { }

    foreign_links { }

    errors {
        BeginningStringInWord(character: char, index: usize) {
            description("Error parsing lisp. Attempted to begin a string while parsing another word.")
            display(
                "{}{}{}{}{}",
                "Error parsing lisp. Attempted to begin a string while parsing another word. The character, '",
                character,
                "', was found at index, '",
                index,
                "' with no separation from the previous word.",
            )
        }

        EmptyEscapeSequence {
            description("Error escaping character. A character must follow a, '\\'.")
            display(
                "{}",
                "Error escaping character. A character must follow a, '\\'.",
            )
        }

        EmptyStringDetected(contents: String) {
            description("Error converting string to lisp object. The string is empty.")
            display(
                "{}{}{}",
                "Error converting string to lisp object. The string, '",
                contents,
                "', is empty.",
            )
        }

        ErrorWrapper(cause: Box<StdError + Send>) {
            description(cause.description())
            display(
                "{}",
                cause,
            )
        }

        InvalidEscapeSequence(escaped_character: char) {
            description("Error escaping character. The character cannot be escaped.")
            display(
                "{}{}{}",
                "Error escaping character. The character, '",
                escaped_character,
                "', cannot be escaped.",
            )
        }

        InvalidExecutionTreeObjectConversion(actual: String, expected: String) {
            description("Error converting from lisp execution tree object. The object is the incorrect type.")
            display(
                "{}{}{}{}{}",
                "Error converting from lisp execution tree object. The expected type of the object is, '",
                expected,
                "', but the actual type of the object is, '",
                actual,
                "'.",
            )
        }

        InvalidPreviousChar(character: char, index: usize, previous_char_type: LastCharType) {
            description("Error parsing lisp. The current character cannot follow the previous character.")
            display(
                "{}{}{}{}{}{}{}",
                "Error parsing lisp. The current character, '",
                character,
                "', at index, '",
                index,
                "', cannot follow the previous character which is type, '",
                previous_char_type.enum_to_string(),
                "'.",
            )
        }

        MultipleCharactersInSingleQuotes(token: String) {
            description("Error parsing lisp. Multiple characters found in single quotes in execution tree.")
            display(
                "{}{}{}",
                "Error parsing lisp. Multiple characters found in single quotes for token, '",
                token,
                "', in execution tree.",
            )
        }

        NoCharacterInSingleQuotes(token: String) {
            description("Error parsing lisp. No character found in single quotes in execution tree.")
            display(
                "{}{}{}",
                "Error parsing lisp. No character found in single quotes for token, '",
                token,
                "', in execution tree.",
            )
        }

        NoClosingDoubleQuoteInExecutionTree(token: String) {
            description("Error parsing lisp. No closing double-quote found in execution tree.")
            display(
                "{}{}{}",
                "Error parsing lisp. No closing double-quote found for token, '",
                token,
                "', in execution tree.",
            )
        }

        NoClosingDoubleQuote(text: String) {
            description("Error parsing lisp. No closing double-quote found.")
            display(
                "{}{}",
                "Error parsing lisp. No closing double-quote found for program:\n",
                text,
            )
        }

        NoClosingParenthesis(text: String) {
            description("Error parsing lisp. No closing parenthesis found.")
            display(
                "{}{}",
                "Error parsing lisp. No closing parenthesis found for program:\n",
                text,
            )
        }

        NoClosingSingleQuoteInExecutionTree(token: String) {
            description("Error parsing lisp. No closing single-quote found in execution tree.")
            display(
                "{}{}{}",
                "Error parsing lisp. No closing single-quote found for token, '",
                token,
                "', in execution tree.",
            )
        }

        NoClosingSingleQuote(text: String) {
            description("Error parsing lisp. No closing single-quote found.")
            display(
                "{}{}",
                "Error parsing lisp. No closing single-quote found for program:\n",
                text,
            )
        }

        NoLastChar(contents: String) {
            description("Error converting string to lisp object. The string has no last char.")
            display(
                "{}{}{}",
                "Error converting string to lisp object. The string, '",
                contents,
                "', has no last char.",
            )
        }

        NoProgramStartParenthesis(text: String) {
            description("Error parsing lisp. Program does not start with parenthesis.")
            display(
                "{}{}",
                "Error parsing lisp. Program does not start with parenthesis. Program text:\n",
                text,
            )
        }

        NumericTokenCannotBeParsed(expected_type: String, token: String) {
            description("Error parsing lisp. Numeric token cannot be parsed.")
            display(
                "{}{}{}{}{}",
                "Error parsing lisp. Numeric token, '",
                expected_type,
                "', cannot be parsed as, '",
                token,
                "'.",
            )
        }

        SymbolNotOnStack(symbol: String) {
            description("Error locating symbol. The symbol cannot be found on the stack.")
            display(
                "{}{}{}",
                "Error locating symbol. The symbol, '",
                symbol,
                "', cannot be found on the stack.",
            )
        }
    }
}
