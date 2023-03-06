mod tok;
use std::fmt::Display;

use crate::prelude::{Result, SyntaxError};

pub use self::tok::Token;

use line_col::LineColLookup;
use logos::{Logos, Span};

pub struct TokenStream<'source> {
    filename: Option<std::path::PathBuf>,
    size: usize,
    tokens: Vec<(Token, Span)>,
    linecol_lookup: LineColLookup<'source>,
}

impl<'source> TokenStream<'source> {
    pub fn new(filename: Option<std::path::PathBuf>, input: &'source str) -> Result<Self> {
        let token_stream = Self {
            filename,
            size: input.len(),
            tokens: Token::lexer(input).spanned().collect(),
            linecol_lookup: LineColLookup::new(input),
        };

        for (token, span) in token_stream.tokens.iter() {
            if let Token::Error = token {
                let (line, col) = token_stream.linecol_lookup.get(span.start);

                return Err(SyntaxError::InvalidToken {
                    token: format!("{token}"),
                    filename: token_stream.filename,
                    line,
                    col,
                });
            }
        }

        Ok(token_stream)
    }

    pub fn tokens(&self) -> &Vec<(Token, Span)> {
        &self.tokens
    }
}

impl<'source> peg::Parse for TokenStream<'source> {
    type PositionRepr = TokenLocation;

    fn start<'input>(&'input self) -> usize {
        0
    }

    fn is_eof<'input>(&'input self, pos: usize) -> bool {
        pos >= self.tokens.len()
    }

    fn position_repr<'input>(&'input self, pos: usize) -> Self::PositionRepr {
        let (token, linecol) = match self.tokens.get(pos) {
            Some((token, span)) => (Some(token.clone()), self.linecol_lookup.get(span.start)),
            None => (None, self.linecol_lookup.get(self.size)),
        };

        Self::PositionRepr {
            filename: self.filename.clone(),
            linecol,
            token,
        }
    }
}

impl<'source, 'input> peg::ParseElem<'input> for TokenStream<'source> {
    type Element = &'input Token;

    fn parse_elem(&'input self, pos: usize) -> peg::RuleResult<Self::Element> {
        match self.tokens.get(pos) {
            Some((token, _)) => peg::RuleResult::Matched(pos + 1, token),
            None => peg::RuleResult::Failed,
        }
    }
}

impl<'source> peg::ParseLiteral for TokenStream<'source> {
    fn parse_string_literal(&self, pos: usize, literal: &str) -> peg::RuleResult<()> {
        match self.tokens.get(pos) {
            // string literals in the grammar rules will match elements
            Some((Token::Element(ele), _)) if ele == literal => {
                peg::RuleResult::Matched(pos + 1, ())
            }
            _ => peg::RuleResult::Failed,
        }
    }
}

impl<'source, 'input> peg::ParseSlice<'input> for TokenStream<'source> {
    type Slice = Vec<&'input Token>;

    fn parse_slice(&'input self, start_pos: usize, end_pos: usize) -> Self::Slice {
        self.tokens[start_pos..end_pos]
            .into_iter()
            .map(|(tok, _)| tok)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenLocation {
    pub filename: Option<std::path::PathBuf>,
    pub linecol: (usize, usize),
    pub token: Option<Token>,
}

impl Display for TokenLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (line, col) = self.linecol;
        let filename = self
            .filename
            .as_ref()
            .map(|path| path.as_path().display().to_string())
            .unwrap_or("<>".to_string());

        write!(f, "{}:{}:{}", filename, line, col)
    }
}
