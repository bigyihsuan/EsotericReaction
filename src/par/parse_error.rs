use std::fmt::Display;

use crate::lex::tok::{Token, Type};

#[derive(Debug)]
pub struct ParseError {
    pub location: (String, u32),
    pub reason: Reason,
    pub last_seen_token: Token,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("ParseError: {}\n", self.reason))?;
        f.write_fmt(format_args!(
            "    at: {}, {}\n",
            self.location.0, self.location.1
        ))?;
        f.write_fmt(format_args!(
            "    last seen token: {}",
            self.last_seen_token
        ))
    }
}

#[derive(Debug)]
pub enum Reason {
    OutOfTokens,
    ExpectedDifferentToken { want: Vec<Type>, got: Type },
    NeedAtLeastOneElemental,
}

impl Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reason::OutOfTokens => f.write_str("out of tokens"),
            Reason::ExpectedDifferentToken { want, got } => {
                f.write_str("expected a different token:\n")?;
                f.write_fmt(format_args!("    want: {:?}\n", want))?;
                f.write_fmt(format_args!("    got:  {}", got))
            }
            Reason::NeedAtLeastOneElemental => f.write_str("need at least one elemental"),
        }
    }
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
