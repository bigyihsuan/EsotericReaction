use crate::tok::{Indexes, Location, Token, Type};
#[derive(Debug)]
pub struct Lexer {
    code: Vec<char>,
    pub idx: usize,
    pub line: usize,
    pub col: usize,
    pub ch: char,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer {
            idx: 0,
            line: 1,
            col: 0,
            code: code.chars().collect(),
            ch: '\0',
        }
    }
    pub fn read_char(&mut self) -> Result<char, String> {
        if let Some(c) = self.code.get(self.idx) {
            self.ch = *c;
            if self.ch == '\n' {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
            self.idx += 1;
            Ok(self.ch)
        } else {
            Err("could not get another char".to_string())
        }
    }
    pub fn put_back(&mut self) -> Result<char, String> {
        self.idx -= 1;
        self.col -= 1;
        if let Some(c) = self.code.get(self.idx) {
            self.ch = *c;
            if self.ch == '\n' {
                self.line -= 1;
            }
            Ok(self.ch)
        } else {
            Err("could not put back char".to_string())
        }
    }
    pub fn all_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.idx < self.code.len() {
            if let Ok(t) = self.next_token() {
                tokens.push(t);
            }
        }
        tokens
    }
    pub fn next_token(&mut self) -> Result<Token, String> {
        self.read_char()?;
        while self.ch.is_whitespace() {
            self.read_char()?;
        }
        match self.ch {
            '+' => ok_token!(Type::Plus, self.idx, self.line, self.col),
            '_' => ok_token!(Type::Underscore, self.idx, self.line, self.col),
            '(' => ok_token!(Type::LParen, self.idx, self.line, self.col),
            ')' => ok_token!(Type::RParen, self.idx, self.line, self.col),
            '[' => ok_token!(Type::LBracket, self.idx, self.line, self.col),
            ']' => ok_token!(Type::RBracket, self.idx, self.line, self.col),
            '{' => ok_token!(Type::LBrace, self.idx, self.line, self.col),
            '}' => ok_token!(Type::RBrace, self.idx, self.line, self.col),
            '<' => ok_token!(Type::LAngle, self.idx, self.line, self.col),
            '>' => ok_token!(Type::RAngle, self.idx, self.line, self.col),
            ':' => ok_token!(Type::Colon, self.idx, self.line, self.col),
            ',' => ok_token!(Type::Comma, self.idx, self.line, self.col),
            '-' => {
                start_vars!(self, idx, line, col);
                self.read_char()?;
                if self.ch != '>' {
                    Err(format!("expected `>` after `-`, got {}", self.ch))
                } else {
                    ok_token!(Type::Arrow, idx, line, col)
                }
            }
            // comment
            ';' => self.comment(),
            // element
            'A'..='Z' => self.element(),
            // string literal
            '\"' => self.string_literal(),
            // number literal
            '^' => self.number(),
            c => Err(format!("unknown character: {c}")),
        }
    }

    fn number(&mut self) -> Result<Token, String> {
        let mut chars = String::new();
        start_vars!(self, idx, line, col);
        // ignore leading caret
        self.read_char()?;
        // begin digits
        if self.ch == '-' {
            chars.push(self.ch);
        }
        while self.ch.is_digit(10) {
            self.read_char()?;
            if self.ch.is_digit(10) {
                chars.push(self.ch);
            } else {
                self.put_back()?;
                break;
            }
        }
        ok_token!(self, Type::Number(chars), idx, line, col)
    }

    fn string_literal(&mut self) -> Result<Token, String> {
        let mut chars = String::new();
        start_vars!(self, idx, line, col);
        self.read_char()?;
        while self.ch != '\"' {
            chars.push(self.ch);
            if self.ch == '\\' {
                // maybe escaped quote
                self.read_char()?;
                if self.ch == '\"' {
                    // escaped quote
                    chars.push(self.ch)
                } else {
                    self.put_back()?;
                }
            }
            self.read_char()?;
        }
        ok_token!(self, Type::String(chars), idx, line, col)
    }

    fn element(&mut self) -> Result<Token, String> {
        let mut lexeme = String::new();
        start_vars!(self, idx, line, col);
        lexeme.push(self.ch);
        self.read_char()?;
        while self.ch.is_ascii_lowercase() {
            if lexeme == String::from("light") {
                return ok_token!(self, Type::Light, idx, line, col);
            } else if lexeme == String::from("heat") {
                return ok_token!(self, Type::Heat, idx, line, col);
            } else {
                lexeme.push(self.ch);
                self.read_char()?;
            }
        }
        self.put_back()?;
        ok_token!(self, Type::Element(lexeme), idx, line, col)
    }

    fn comment(&mut self) -> Result<Token, String> {
        let mut comment = String::new();
        start_vars!(self, idx, line, col);
        while self.ch != '\n' {
            comment.push(self.ch);
            self.read_char()?;
        }
        ok_token!(self, Type::Comment(comment), idx, line, col)
    }
}

macro_rules! ok_token {
    ($self:ident, $token_type:expr, $start_idx:expr, $start_line:expr, $start_col:expr) => {
        Ok(Token {
            token: $token_type,
            loc: Location {
                start: Indexes($start_idx, $start_line, $start_col),
                end: Indexes($self.idx + 1, $self.line, $self.col),
            },
        })
    };
    ($token_type:expr, $idx:expr, $line:expr, $col:expr) => {
        Ok(Token {
            token: $token_type,
            loc: Location {
                start: Indexes($idx, $line, $col),
                end: Indexes($idx + 1, $line, $col),
            },
        })
    };
}

macro_rules! start_vars {
    ($self:ident, $idx:ident, $line:ident, $col:ident) => {
        let $idx = $self.idx;
        let $line = $self.line;
        let $col = $self.col;
    };
}

use ok_token;
use start_vars;
