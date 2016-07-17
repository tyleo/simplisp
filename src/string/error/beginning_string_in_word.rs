pub fn description() -> &'static str {
    "Error parsing lisp. Attempted to begin a string while parsing another word."
}

pub fn display_1() -> &'static str {
    "Error parsing lisp. Attempted to begin a string while parsing another word. The character, '"
}

pub fn display_2() -> &'static str {
    "', was found at index, '"
}

pub fn display_3() -> &'static str {
    "' with no separation from the previous word."
}
