use std::fmt::Display;

use crate::lexer::TokenLocation;

pub type Result<T> = std::result::Result<T, SyntaxError>;

#[derive(Debug)]
pub enum SyntaxError {
    InvalidToken {
        token: String,
        filename: Option<std::path::PathBuf>,
        line: usize,
        col: usize,
    },
    UnexpectedToken {
        token: String,
        filename: Option<std::path::PathBuf>,
        line: usize,
        col: usize,
    },
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidToken {
                token,
                filename,
                line,
                col,
            } => {
                let filename = filename
                    .as_ref()
                    .map(|path| path.as_path().display().to_string())
                    .unwrap_or("<>".to_string());

                write!(f, "invalid token '{token}' @ {filename}:{line}:{col}")
            }
            Self::UnexpectedToken {
                token,
                filename,
                line,
                col,
            } => {
                let filename = filename
                    .as_ref()
                    .map(|path| path.as_path().display().to_string())
                    .unwrap_or("<>".to_string());

                write!(f, "unexpected token '{token}' @ {filename}:{line}:{col}")
            }
        }
    }
}

impl std::error::Error for SyntaxError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

type ParseError = peg::error::ParseError<TokenLocation>;

impl From<ParseError> for SyntaxError {
    fn from(err: ParseError) -> Self {
        let TokenLocation {
            filename,
            linecol: (line, col),
            token,
        } = err.location;

        Self::UnexpectedToken {
            token: token.map(|tok| tok.to_string()).unwrap_or("<>".to_string()),
            filename,
            line,
            col,
        }
    }
}
