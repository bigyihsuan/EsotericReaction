#[derive(Debug)]
pub enum Token {
    Comment(String),
    // symbols
    Plus,
    Underscore,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    LAngle,
    RAngle,
    Colon,
    Arrow,
    Comma,
    // literals
    Number(String),
    String(String),
    // elements
    Light,
    Heat,
    Element(String),
}

impl Token {}
