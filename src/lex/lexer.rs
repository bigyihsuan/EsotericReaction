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
            self.idx += 1;
            self.col += 1;
            if self.ch == '\n' {
                self.line += 1;
                self.col = 1;
            }
            Ok(self.ch)
        } else {
            Err("could not get another char".to_string())
        }
    }
    pub fn put_back(&mut self) -> Result<char, String> {
        self.idx -= 1;
        let old_col = self.col;
        if self.col > 0 {
            self.col -= 1;
        }
        if let Some(c) = self.code.get(self.idx) {
            self.ch = *c;
            if self.ch == '\n' {
                self.line -= 1;
                self.col = old_col
            }
            Ok(self.ch)
        } else {
            Err("could not put back char".to_string())
        }
    }
    /*
    pub fn all_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.idx < self.code.len() {
            if let Ok(t) = self.next_token() {
                tokens.push(t);
            }
        }
        let tokens: Vec<Token> = tokens
            .iter()
            // no comments or spaces
            .filter(|tok| match tok.token {
                Type::Comment(_) | Type::Space => false,
                _ => true,
            })
            // remove duplicate newlines from removing comments
            .dedup_by(|l, r| l.token == r.token)
            .map(|tok| tok.clone())
            .collect();
        // remove "empty" lines (newlines at the start of the token list)
        let first = tokens.first();
        if first.is_some() && first.unwrap().clone().token == Type::Newline {
            tokens.split_at(1).1.to_vec()
        } else {
            tokens
        }
    }
    pub fn next_token(&mut self) -> Result<Token, String> {
        self.read_char()?;
        match self.ch {
            ' ' => ok_token!(Type::Space, self.idx, self.line, self.col),
            '\n' => {
                start_vars!(self, idx, line, col);
                self.line += 1;
                self.col = 0;
                ok_token!(Type::Newline, idx, line, col)
            }
            '+' => ok_token!(self, Type::Plus),
            '_' => ok_token!(self, Type::Underscore),
            '(' => ok_token!(self, Type::LParen),
            ')' => ok_token!(self, Type::RParen),
            '[' => ok_token!(self, Type::LBracket),
            ']' => ok_token!(self, Type::RBracket),
            '{' => ok_token!(self, Type::LBrace),
            '}' => ok_token!(self, Type::RBrace),
            '<' => ok_token!(self, Type::LAngle),
            '>' => ok_token!(self, Type::RAngle),
            ':' => ok_token!(self, Type::Colon),
            ',' => ok_token!(self, Type::Comma),
            '^' => ok_token!(self, Type::Caret),
            '-' => {
                start_vars!(self, idx, line, col);
                self.read_char()?;
                if self.ch.is_digit(10) {
                    self.put_back()?; // digit
                    self.put_back()?; // minus
                    self.number()
                } else if self.ch != '>' {
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
            '\"' => self.string(),
            // number literal
            c if c.is_digit(10) => {
                self.put_back()?;
                self.number()
            }
            c => Err(format!("unknown character: {c}")),
        }
    }

    fn comment(&mut self) -> Result<Token, String> {
        let mut comment = String::new();
        start_vars!(self, idx, line, col);
        let mut end_col = col;
        while self.ch != '\n' {
            comment.push(self.ch);
            end_col = self.col;
            self.read_char()?;
        }
        self.put_back()?;
        ok_token!(
            self,
            Type::Comment(comment),
            idx,
            line,
            col,
            self.idx,
            line,
            end_col + 1
        )
    }

    fn number(&mut self) -> Result<Token, String> {
        let mut chars = String::new();
        start_vars!(self, idx, line, col);
        // begin digits
        if self.ch == '-' {
            chars.push(self.ch);
        }
        self.read_char()?;
        while self.ch.is_digit(10) {
            chars.push(self.ch);
            self.read_char()?;
        }
        self.put_back()?;
        ok_token!(self, Type::Number(chars), idx, line, col)
    }

    fn string(&mut self) -> Result<Token, String> {
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
    */
}

macro_rules! ok_token {
    ($self:ident, $token_type:expr, $start_idx:expr, $start_line:expr, $start_col:expr) => {
        Ok(Token {
            token: $token_type,
            loc: Span {
                start: Indexes($start_idx, $start_line, $start_col),
                end: Indexes($self.idx + 1, $self.line, $self.col),
            },
        })
    };
    ($self:ident, $token_type:expr) => {
        Ok(Token {
            token: $token_type,
            loc: Span {
                start: Indexes($self.idx, $self.line, $self.col),
                end: Indexes($self.idx + 1, $self.line, $self.col),
            },
        })
    };
    ($self:ident, $token_type:expr, $start_idx:expr, $start_line:expr, $start_col:expr, $end_idx:expr, $end_line:expr, $end_col:expr) => {
        Ok(Token {
            token: $token_type,
            loc: Span {
                start: Indexes($start_idx, $start_line, $start_col),
                end: Indexes($end_idx + 1, $end_line, $end_col),
            },
        })
    };
    ($token_type:expr, $idx:expr, $line:expr, $col:expr) => {
        Ok(Token {
            token: $token_type,
            loc: Span {
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

use itertools::Itertools;
use ok_token;
use start_vars;
