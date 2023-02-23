use crate::tok::Token;
#[derive(Debug)]
pub struct Lexer {
    code: Vec<char>,
    pub idx: usize,
    pub ch: char,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer {
            idx: 0,
            code: code.chars().collect(),
            ch: '\0',
        }
    }
    pub fn read_char(&mut self) -> Result<char, String> {
        if let Some(c) = self.code.get(self.idx) {
            self.ch = *c;
            self.idx += 1;
            Ok(self.ch)
        } else {
            Err("could not get another char".to_string())
        }
    }
    pub fn put_back(&mut self) -> Result<char, String> {
        self.idx -= 1;
        if let Some(c) = self.code.get(self.idx) {
            self.ch = *c;
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
            '+' => Ok(Token::Plus),
            '_' => Ok(Token::Underscore),
            '(' => Ok(Token::LParen),
            ')' => Ok(Token::RParen),
            '[' => Ok(Token::LBracket),
            ']' => Ok(Token::RBracket),
            '{' => Ok(Token::LBrace),
            '}' => Ok(Token::RBrace),
            ':' => Ok(Token::Colon),
            ',' => Ok(Token::Comma),
            '-' => {
                self.read_char()?;
                if self.ch != '>' {
                    Err(format!("expected `>` after `-`, got {}", self.ch))
                } else {
                    Ok(Token::Arrow)
                }
            }
            // comment
            ';' => {
                let mut comment = String::new();
                while self.ch != '\n' {
                    comment.push(self.ch);
                    self.read_char()?;
                }
                Ok(Token::Comment(comment))
            }
            // element
            'A'..='Z' | 'a'..='z' => self.element(),
            // string literal
            '\"' => self.string_literal(),
            // number literal
            '^' => self.number(),
            c => Err(format!("unknown character: {c}")),
        }
    }

    fn number(&mut self) -> Result<Token, String> {
        let mut chars = String::new();
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
        Ok(Token::Number(chars))
    }

    fn string_literal(&mut self) -> Result<Token, String> {
        let mut chars = String::new();
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
        Ok(Token::String(chars))
    }

    fn element(&mut self) -> Result<Token, String> {
        let mut lexeme = String::new();
        lexeme.push(self.ch);
        self.read_char()?;
        while self.ch.is_ascii_lowercase() {
            if lexeme == String::from("light") {
                return Ok(Token::Light);
            } else if lexeme == String::from("heat") {
                return Ok(Token::Heat);
            } else {
                lexeme.push(self.ch);
                self.read_char()?;
            }
        }
        self.put_back()?;
        Ok(Token::Element(lexeme))
    }
}
