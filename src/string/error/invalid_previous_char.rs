pub fn description() -> &'static str {
    "Error parsing lisp. The current character cannot follow the previous character."
}

pub fn display_1() -> &'static str {
    "Error parsing lisp. The current character, '"
}

pub fn display_2() -> &'static str {
    "', at index, '"
}

pub fn display_3() -> &'static str {
    "', cannot follow the previous character which is type, '"
}

pub fn display_4() -> &'static str {
    "'."
}
