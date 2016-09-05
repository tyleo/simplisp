#[derive(Debug)]
pub enum LastCharType {
    CloseParen,
    OpenParen,
    Quote,
    WhiteSpace,
    Word,
}

impl LastCharType {
    pub fn close_paren_str() -> &'static str {
        "LastCharType::CloseParen"
    }

    pub fn open_paren_str() -> &'static str {
        "LastCharType::OpenParen"
    }

    pub fn quote_str() -> &'static str {
        "LastCharType::Quote"
    }

    pub fn white_space_str() -> &'static str {
        "LastCharType::WhiteSpace"
    }

    pub fn word_str() -> &'static str {
        "LastCharType::Word"
    }

    pub fn enum_to_string(&self) -> &'static str {
        match self {
            &LastCharType::CloseParen => Self::close_paren_str(),
            &LastCharType::OpenParen => Self::open_paren_str(),
            &LastCharType::Quote => Self::quote_str(),
            &LastCharType::WhiteSpace => Self::white_space_str(),
            &LastCharType::Word => Self::word_str(),
        }
    }
}
