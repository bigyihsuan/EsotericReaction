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
        elementals: Box<ParseTree>,
    },
    Elementals {
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
    ElementalNumberLiteral {
        hydrogen: Token,
        oxygen: Token,
        vals: Box<ParseTree>,
    },
    SugaredNumberLiteral {
        hydrogen: Token,
        caret: Token,
        val: Box<ParseTree>,
    },
    ElementalBooleanLiteral {
        hydrogen: Token,
        boron: Token,
        hydroxide: Box<ParseTree>,
        val: Box<ParseTree>,
    },
    SugaredBooleanLiteral {
        hydrogen: Token,
        val: Token,
    },
    ElementalStringLiteral {
        hydrogen: Token,
        sulfur: Token,
        val: Token,
    },
    SugaredStringLiteral {
        hydrogen: Token,
        val: Token,
    },
    ElementalPairLiteral {
        left: Box<ParseTree>,
        right: Box<ParseTree>,
    },
    SugaredPairLiteral {
        left: Box<ParseTree>,
        right: Box<ParseTree>,
    },
    ElementalListLiteral {
        items: Vec<ParseTree>,
    },
    SugaredListLiteral {
        items: Vec<ParseTree>,
    },
    ElementalMapLiteral {
        items: Vec<(ParseTree, ParseTree)>,
    },
    SugaredMapLiteral {
        items: Vec<(ParseTree, ParseTree)>,
    },
    // misc
    Number {
        val: Token,
    },
}
