use crate::lex::tok::Token;

#[derive(Debug)]
pub enum Ast {
    None,
    // literals
    NumberLiteral(Literal),
    StringLiteral(Literal),
    BooleanLiteral(Literal),
    PairLiteral(Literal),
    ListLiteral(Literal),
    MapLiteral(Literal),
    // symbolics
    Subscript(Subscript),
    Element(Element),
    Compound(Compound),
    Equation(Equation),
    Program(Program),
}

#[derive(Debug)]
pub struct Literal {
    hydrogen: Token,
    val: Token,
}

#[derive(Debug)]
pub struct Subscript {
    underscore: Token,
    subscript: Option<Token>,
}

#[derive(Debug)]
pub struct Element {
    element: Token,
    subscript: Option<Box<Ast>>,
}

#[derive(Debug)]
pub struct Compound {
    coeff: Option<Token>,
    elements: Vec<Ast>,
}

#[derive(Debug)]
pub struct Equation {
    lhs: Vec<Ast>,
    arrow: Token,
    rhs: Vec<Ast>,
}

#[derive(Debug)]
pub struct Program {
    equations: Vec<Ast>,
}
