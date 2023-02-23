use std::fmt::Display;

#[derive(Debug)]
pub enum Type {
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

#[derive(Debug)]
pub struct Token {
    pub token: Type,
    pub loc: Location,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{{{:?},{}}}", self.token, self.loc))
    }
}

#[derive(Debug)]
pub struct Location {
    pub start: Indexes, // (inclusive, exclusive)
    pub end: Indexes,   // (inclusive, exclusive)
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}-{}", self.start, self.end))
    }
}

type Idx = usize;
type Line = usize;
type Col = usize;

#[derive(Debug)]
pub struct Indexes(pub Idx, pub Line, pub Col);

impl Display for Indexes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{}:{})", self.0, self.1, self.2))
    }
}
