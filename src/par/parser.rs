use itertools::{PeekingNext, PutBack};

use crate::lex::tok::{Token, Type};

use super::{
    parse_error::{parse_error, ParseError, Reason},
    parse_tree::{
        Compound, DecimalNumber, DecimalNumberLiteral, Element, Equation, Literal, ParseTree,
        Periodic, Program, Subscript,
    },
};

#[derive(Debug, Clone)]
pub enum EquationSide {
    Left,
    Right,
}

pub struct Parser {
    pub tokens: PutBack<Box<dyn Iterator<Item = Token>>>,
    pub used_tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let tokens = tokens.clone().into_iter();
        let toks: Box<dyn Iterator<Item = Token>> = Box::new(tokens);
        Parser {
            tokens: itertools::put_back(toks),
            used_tokens: Vec::new(),
        }
    }

    fn next_token(&mut self) -> Result<Token, ParseError> {
        dbg!(stdext::function_name!());
        let next = self.tokens.next();
        if let Some(token) = next {
            self.used_tokens.push(token.clone());
            dbg!(&token);
            Ok(token)
        } else {
            parse_error!(
                Reason::OutOfTokens,
                self.used_tokens.last().unwrap_or(&Token::default()).clone()
            )
        }
    }

    fn put_back_token(&mut self) -> Result<bool, ParseError> {
        dbg!(stdext::function_name!());
        if self.used_tokens.len() > 0 {
            let Some(last_token) = self.used_tokens.pop() else {
                return parse_error!(
                    Reason::OutOfTokens,
                    self.used_tokens.last().unwrap_or(&Token::default()).clone()
                )
            };
            self.tokens.put_back(last_token);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn peek_token(&mut self) -> Result<Token, ParseError> {
        dbg!(stdext::function_name!());
        if let Some(peeked) = self.tokens.peeking_next(|_token| true) {
            self.tokens.put_back(peeked.clone());
            dbg!(&peeked);
            Ok(peeked)
        } else {
            parse_error!(
                Reason::OutOfTokens,
                self.used_tokens.last().unwrap_or(&Token::default()).clone()
            )
        }
    }

    pub fn parse(&mut self) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!());
        self.program()
    }

    pub fn program(&mut self) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!());
        let mut equations = Vec::new();
        while self.peek_token().is_ok() {
            equations.push(self.equation()?);
        }
        Ok(ParseTree::Program(Program { equations }))
    }

    pub fn equation(&mut self) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!());
        let side = EquationSide::Left;
        let mut lhs = Vec::new();
        let mut rhs = Vec::new();
        lhs.push(self.compound(side.clone())?);
        loop {
            let tok = self.peek_token()?;
            if let Type::Plus = tok.token {
                let _plus = self.next_token(); // discard plus
                lhs.push(self.compound(side.clone())?);
            } else {
                break;
            }
        }
        let arrow = self.next_token()?;
        match arrow.token {
            Type::Arrow => {
                let arrow = arrow;
                let side = EquationSide::Right;
                rhs.push(self.compound(side.clone())?);
                loop {
                    let tok = self.peek_token()?;
                    dbg!(&tok);
                    if let Type::Newline = tok.token {
                        break;
                    } else if let Type::Plus = tok.token {
                        let _plus = self.next_token(); // discard plus
                        rhs.push(self.compound(side.clone())?);
                    } else {
                        break;
                    }
                }
                let newline = self.next_token()?;
                if let Type::Newline = newline.token {
                    dbg!("finished equation:", &lhs, &arrow, &rhs, &newline);
                    Ok(ParseTree::Equation(Equation {
                        lhs,
                        arrow,
                        rhs,
                        newline,
                    }))
                } else {
                    parse_error!(
                        Reason::ExpectedDifferentToken {
                            want: vec![Type::Newline],
                            got: newline.clone().token
                        },
                        newline
                    )
                }
            }
            _ => {
                parse_error!(
                    Reason::ExpectedDifferentToken {
                        want: vec![Type::Arrow],
                        got: arrow.clone().token
                    },
                    arrow
                )
            }
        }
    }

    pub fn compound(&mut self, side: EquationSide) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!(), &side);
        let coeff = self.coeff()?;
        let elementals = self.elementals(side)?;
        Ok(ParseTree::Compound(Compound { coeff, elementals }))
    }

    pub fn elementals(&mut self, side: EquationSide) -> Result<Vec<ParseTree>, ParseError> {
        dbg!(stdext::function_name!(), &side);
        let mut elementals = Vec::new();
        let first = self.elemental(side.clone())?;
        if let Some(first) = first {
            elementals.push(first);
        } else {
            return parse_error!(
                Reason::NeedAtLeastOneElemental,
                self.used_tokens.last().unwrap_or(&Token::default()).clone()
            );
        }
        if let Type::Newline = self.peek_token()?.clone().token {
            return Ok(elementals);
        }
        loop {
            let elemental = self.elemental(side.clone())?;
            if let Some(e) = elemental {
                elementals.push(e);
            } else if let None = elemental {
                break;
            }
        }
        Ok(elementals)
    }

    pub fn elemental(&mut self, side: EquationSide) -> Result<Option<ParseTree>, ParseError> {
        dbg!(stdext::function_name!(), &side);
        // limit the right hand side to only periodics
        if let EquationSide::Right = side {
            let periodic = self.periodic()?;
            return Ok(Some(periodic));
        }
        let token = self.peek_token()?;
        match token.token {
            Type::Element(symbol) => {
                if symbol.as_str() == "H" {
                    let hydrogen = self.next_token()?;
                    let literal = self.literal(hydrogen)?;
                    Ok(Some(literal))
                } else {
                    let periodic = self.periodic()?;
                    Ok(Some(periodic))
                }
            }
            _ => Ok(None),
        }
    }

    fn literal(&mut self, hydrogen: Token) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!());
        let token = self.peek_token()?;
        let t = token.clone();
        match token.token {
            Type::Caret => self.decimal_number_literal(hydrogen),
            Type::String(_) => self.string_literal(hydrogen),
            Type::LAngle => self.pair_literal(hydrogen),
            Type::LBracket => self.list_literal(hydrogen),
            Type::LBrace => self.map_literal(hydrogen),
            Type::Element(symbol) => match symbol.as_str() {
                "Tr" | "Fa" => self.boolean_literal(hydrogen),
                "O" => self.elemental_number_literal(hydrogen),
                "S" => self.elemental_string_literal(hydrogen),
                "N" => self.elemental_pair_literal(hydrogen),
                "C" => self.elemental_list_map_literal(hydrogen),
                _ => parse_error!(
                    Reason::ExpectedDifferentToken {
                        want: vec![
                            Type::Element(String::from("Tr")),
                            Type::Element(String::from("Fa")),
                            Type::Element(String::from("O")),
                            Type::Element(String::from("S")),
                            Type::Element(String::from("N")),
                            Type::Element(String::from("C")),
                        ],
                        got: t.clone().token
                    },
                    t
                ),
            },
            _ => {
                return parse_error!(
                    Reason::ExpectedDifferentToken {
                        want: vec![
                            Type::Element(String::from("Tr")),
                            Type::Element(String::from("Fa")),
                            Type::Caret,
                            Type::String(String::new()),
                            Type::LAngle,
                            Type::LBracket,
                            Type::LBrace,
                        ],
                        got: token.clone().token
                    },
                    token
                )
            }
        }
    }

    fn periodic(&mut self) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!());
        let element = self.element()?;
        let element = Box::new(element);
        let subscript = self.subscript()?;
        let subscript = Box::new(subscript);
        Ok(ParseTree::Periodic(Periodic { element, subscript }))
    }

    pub fn element(&mut self) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!());
        let val = self.next_token()?;
        if let Type::Element(_) = val.token {
            Ok(ParseTree::Element(Element { val }))
        } else {
            parse_error!(
                Reason::ExpectedDifferentToken {
                    want: vec![Type::Element("".to_string())],
                    got: val.clone().token
                },
                val
            )
        }
    }

    pub fn coeff(&mut self) -> Result<Option<Token>, ParseError> {
        dbg!(stdext::function_name!());
        let tok = self.peek_token()?;
        if let Type::Number(_) = tok.token {
            let coeff = self.next_token()?;
            Ok(Some(coeff))
        } else {
            Ok(None)
        }
    }
    pub fn subscript(&mut self) -> Result<Option<ParseTree>, ParseError> {
        dbg!(stdext::function_name!());
        let underscore = self.peek_token()?;
        if let Type::Underscore = underscore.token {
            let underscore = self.next_token()?;
            let val = self.next_token()?;
            if let Type::Number(_) = val.token {
                Ok(Some(ParseTree::Subscript(Subscript { underscore, val })))
            } else {
                parse_error!(
                    Reason::ExpectedDifferentToken {
                        want: vec![Type::Number("".to_string())],
                        got: val.clone().token
                    },
                    val
                )
            }
        } else {
            Ok(None)
        }
    }

    pub fn number_literal(&mut self, hydrogen: Token) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!());
        let token = self.peek_token()?;
        if let Type::Caret = token.token {
            self.decimal_number_literal(hydrogen)
        } else if let Type::Element(_) = token.token {
            todo!()
        } else {
            parse_error!(
                Reason::ExpectedDifferentToken {
                    want: vec![Type::Caret, Type::Element("".to_string())],
                    got: token.clone().token
                },
                token
            )
        }
    }

    fn decimal_number(&mut self, val: Token) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!());
        let token = val.clone();
        if let Type::Number(_) = token.token {
            Ok(ParseTree::DecimalNumber(DecimalNumber { val }))
        } else {
            parse_error!(
                Reason::ExpectedDifferentToken {
                    want: vec![Type::Number("".to_string())],
                    got: token.clone().token
                },
                token
            )
        }
    }

    pub fn decimal_number_literal(&mut self, hydrogen: Token) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!());
        // hydrogen caret number
        let caret = self.next_token()?;
        let val = self.next_token()?;
        if let Type::Number(_) = val.token {
            let num = self.decimal_number(val)?;
            let num = Box::new(num);
            Ok(ParseTree::DecimalNumberLiteral(DecimalNumberLiteral {
                hydrogen,
                caret,
                val: num,
            }))
        } else {
            parse_error!(
                Reason::ExpectedDifferentToken {
                    want: vec![Type::Number("".to_string())],
                    got: val.clone().token
                },
                val
            )
        }
    }

    fn elemental_number_literal(&mut self, hydrogen: Token) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!());
        // hydrogen oxygen elements
        let oxygen = self.next_token()?;
        let elements = self.compound(EquationSide::Left)?;
        todo!()
    }

    fn boolean_literal(&mut self, hydrogen: Token) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!());
        // HTr or HFa
        let token = self.next_token()?;
        let val = token.clone();
        if let Type::Element(element) = token.token {
            if element == String::from("Tr") || element == String::from("Fa") {
                Ok(ParseTree::BooleanLiteral(Literal { hydrogen, val }))
            } else {
                parse_error!(
                    Reason::ExpectedDifferentToken {
                        want: vec![
                            Type::Element(String::from("Tr")),
                            Type::Element(String::from("Fa")),
                        ],
                        got: val.clone().token
                    },
                    val
                )
            }
        } else {
            parse_error!(
                Reason::ExpectedDifferentToken {
                    want: vec![
                        Type::Element(String::from("Tr")),
                        Type::Element(String::from("Fa")),
                    ],
                    got: val.clone().token
                },
                val
            )
        }
    }

    fn string_literal(&mut self, hydrogen: Token) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!());
        todo!()
    }

    fn elemental_string_literal(&mut self, hydrogen: Token) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!());
        todo!()
    }

    fn pair_literal(&mut self, hydrogen: Token) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!());
        todo!()
    }

    fn elemental_pair_literal(&mut self, hydrogen: Token) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!());
        todo!()
    }

    fn elemental_list_map_literal(&mut self, hydrogen: Token) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!());
        todo!()
    }

    fn list_literal(&mut self, hydrogen: Token) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!());
        todo!()
    }

    fn map_literal(&mut self, hydrogen: Token) -> Result<ParseTree, ParseError> {
        dbg!(stdext::function_name!());
        todo!()
    }
}
