#[derive(Debug)]
pub enum Token {
    Comment,
    // symbols
    Plus,
    Underscore,
    LParen,
    RParen,
    Colon,
    Arrow,
    // literals
    String(String),
    Number(String),
    // elements
    Light,
    Heat,
    Element(String),
}

impl Token {}
