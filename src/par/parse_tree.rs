use crate::lex::tok::Token;

#[derive(Debug)]
pub enum ParseTree {
    None,
    // symbolics
    Program {
        equations: Vec<ParseTree>,
    },
    Equation {
        lhs: Vec<ParseTree>,
        arrow: Token,
        rhs: Vec<ParseTree>,
        newline: Token,
    },
    Compound {
        coeff: Option<Token>,
        elementals: Vec<ParseTree>,
    },
    Periodic {
        element: Box<ParseTree>,
        subscript: Box<Option<ParseTree>>,
    },
    Element {
        val: Token,
    },
    Subscript {
        underscore: Token,
        val: Token,
    },
    // literals
    NumberLiteral {
        hydrogen: Token,
        val: Token,
    },
    ElementalNumberLiteral {
        hydrogen: Token,
        oxygen: Token,
        vals: Box<ParseTree>,
    },
    DecimalNumberLiteral {
        hydrogen: Token,
        caret: Token,
        val: Box<ParseTree>,
    },
    StringLiteral {
        hydrogen: Token,
        val: Token,
    },
    BooleanLiteral {
        hydrogen: Token,
        val: Token,
    },
    PairLiteral {
        left: Box<ParseTree>,
        right: Box<ParseTree>,
    },
    ListLiteral {
        items: Vec<ParseTree>,
    },
    MapLiteral {
        items: Vec<(ParseTree, ParseTree)>,
    },
    // misc
    DecimalNumber {
        val: Token,
    },
}
