use crate::lex::tok::{Token, Type};

#[derive(Debug)]
pub struct ParseError {
    pub location: (String, u32),
    pub reason: Reason,
    pub last_seen_token: Token,
}

#[derive(Debug)]
pub enum Reason {
    OutOfTokens,
    ExpectedDifferentToken { want: Vec<Type>, got: Type },
    NeedAtLeastOneElemental,
}

macro_rules! parse_error {
    ($reason:expr, $last_seen_token:expr) => {
        Err(ParseError {
            location: (stdext::function_name!().to_string(), line!()),
            reason: $reason,
            last_seen_token: $last_seen_token,
        })
    };
}

pub(super) use parse_error;
