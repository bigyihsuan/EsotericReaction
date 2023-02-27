use crate::lex::tok::Token;

#[derive(Debug)]
pub enum ParseTree {
    None,
    // symbolics
    Program(Program),
    Equation(Equation),
    Compound(Compound),
    Periodic(Periodic),
    Element(Element),
    Subscript(Subscript),
    // literals
    NumberLiteral(Literal),
    DecimalNumberLiteral(DecimalNumberLiteral),
    StringLiteral(Literal),
    BooleanLiteral(Literal),
    PairLiteral(PairLiteral),
    ListLiteral(ListLiteral),
    MapLiteral(MapLiteral),
    // misc
    DecimalNumber(DecimalNumber),
}

#[derive(Debug)]
pub struct Program {
    pub equations: Vec<ParseTree>,
}

#[derive(Debug)]
pub struct Equation {
    pub lhs: Vec<ParseTree>,
    pub arrow: Token,
    pub rhs: Vec<ParseTree>,
    pub newline: Token,
}

#[derive(Debug)]
pub struct Compound {
    pub coeff: Option<Token>,
    pub elementals: Vec<ParseTree>,
}

#[derive(Debug)]
pub struct Periodic {
    pub element: Box<ParseTree>,
    pub subscript: Box<Option<ParseTree>>,
}

#[derive(Debug)]
pub struct Element {
    pub val: Token,
}

#[derive(Debug)]
pub struct Subscript {
    pub underscore: Token,
    pub val: Token,
}

#[derive(Debug)]
pub struct Literal {
    pub hydrogen: Token,
    pub val: Token,
}

#[derive(Debug)]
pub struct DecimalNumberLiteral {
    pub hydrogen: Token,
    pub caret: Token,
    pub val: Box<ParseTree>,
}

#[derive(Debug)]
pub struct PairLiteral {
    pub left: Box<ParseTree>,
    pub right: Box<ParseTree>,
}

#[derive(Debug)]
pub struct ListLiteral {
    pub items: Vec<ParseTree>,
}

#[derive(Debug)]
pub struct MapLiteral {
    pub items: Vec<(ParseTree, ParseTree)>,
}

#[derive(Debug)]
pub struct DecimalNumber {
    pub val: Token,
}
