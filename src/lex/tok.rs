use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Type {
    None,
    Comment(String),
    // whitespace
    Space,
    Newline,
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
    Caret,
    // literals
    Number(String),
    String(String),
    // elements
    Light,
    Heat,
    Element(String),
}

impl Default for Type {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Default)]
pub struct Token {
    pub token: Type,
    pub loc: Span,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{{{:?},{}}}", self.token, self.loc))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Span {
    pub start: Indexes, // (inclusive, exclusive)
    pub end: Indexes,   // (inclusive, exclusive)
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}-{}", self.start, self.end))
    }
}

type Idx = usize;
type Line = usize;
type Col = usize;

#[derive(Debug, Clone, Default)]
pub struct Indexes(pub Idx, pub Line, pub Col);

impl Display for Indexes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{}:{})", self.0, self.1, self.2))
    }
}
