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
            ch: '0',
        }
    }
    pub fn read_char(&mut self) -> Result<char, ()> {
        if let Some(c) = self.code.get(self.idx) {
            self.ch = *c;
            self.idx += 1;
            Ok(self.ch)
        } else {
            Err(())
        }
    }
    pub fn put_back(&mut self) -> Result<char, ()> {
        self.idx -= 1;
        if let Some(c) = self.code.get(self.idx) {
            self.ch = *c;
            Ok(self.ch)
        } else {
            Err(())
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
    pub fn next_token(&mut self) -> Result<Token, ()> {
        self.read_char()?;
        while self.ch.is_whitespace() {
            self.read_char()?;
        }
        match self.ch {
            '+' => Ok(Token::Plus),
            '_' => Ok(Token::Underscore),
            '(' => Ok(Token::LParen),
            ')' => Ok(Token::RParen),
            ':' => Ok(Token::Colon),
            '-' => {
                self.read_char()?;
                if self.ch != '>' {
                    Err(())
                } else {
                    Ok(Token::Arrow)
                }
            }
            // comment
            ';' => {
                while self.ch != '\n' {
                    self.read_char()?;
                }
                Ok(Token::Comment)
            }
            // element
            'A'..='Z' | 'a'..='z' => {
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
            // string literal
            '\'' => {
                let mut chars = String::new();
                self.read_char()?;
                while self.ch != '\'' {
                    chars.push(self.ch);
                    if self.ch == '\\' {
                        self.read_char()?;
                        if self.ch == '\'' {
                            chars.push(self.ch)
                        } else {
                            self.put_back()?;
                        }
                    }
                    self.read_char()?;
                }
                Ok(Token::String(chars))
            }
            // number literal
            '0'..='9' => {
                let mut chars = String::new();
                chars.push(self.ch);
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
            _ => Err(()),
        }
    }
}
