use std::fmt::Display;

use logos::Logos;
use snailquote::unescape;

#[derive(Logos, Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // ignore whitespace
    #[error]
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Error,
    #[regex(r";[^\n]*\n?", logos::skip)]
    Comment,
    // symbols
    #[token("+")]
    Plus,
    #[token("_")]
    Underscore,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("<")]
    LAngle,
    #[token(">")]
    RAngle,
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,
    #[token(",")]
    Comma,
    #[token(".")]
    Period,
    #[token("^")]
    Caret,
    // literals
    #[regex("-?[0-9]+", |lex| parse_int::parse::<i64>(lex.slice()))]
    Number(i64),
    #[regex("\"(?:[^\"]|\\\\\")*\"", |lex| unescape(lex.slice()))]
    String(String),
    // elements
    #[regex("[A-Z][a-z]*", |lex| lex.slice().parse(), priority=2)]
    Element(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}
