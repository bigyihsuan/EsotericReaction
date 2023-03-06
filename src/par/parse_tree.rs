use std::fmt::Debug;

use crate::intern::Symbol;

#[derive(Debug, Clone, PartialEq)]
pub struct Program<S: Symbol>(pub Vec<Equation<S>>);

#[derive(Debug, Clone, PartialEq)]
pub struct Equation<S: Symbol> {
    pub lhs: Reagent<S>,
    pub rhs: Reagent<S>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Reagent<S: Symbol>(pub Vec<Term<S>>);

#[derive(Clone, PartialEq)]
pub struct Term<S: Symbol> {
    pub coeff: Option<Number>,
    pub mol: Vec<Molecule<S>>,
}

impl<S: Symbol + Debug> Debug for Term<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(n) = &self.coeff {
            f.debug_struct("Term")
                .field("coeff", n)
                .field("mol", &self.mol)
                .finish()
        } else {
            f.debug_struct("Term").field("mol", &self.mol).finish()
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Molecule<S: Symbol> {
    pub mol: MoleculeContents<S>,
    pub sub: Option<Number>,
}

impl<S: Symbol + Debug> Debug for Molecule<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(n) = &self.sub {
            f.debug_struct("Molecule")
                .field("mol", &self.mol)
                .field("sub", n)
                .finish()
        } else {
            f.debug_struct("Molecule").field("mol", &self.mol).finish()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoleculeContents<S: Symbol> {
    Literal(Literal<S>),
    Nested(Box<Molecule<S>>),
    Element(Element<S>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<S: Symbol> {
    Number {
        hydrogen: Element<S>,
        // c: caret
        n: Number,
    },
    Boolean {
        hydrogen: Element<S>,
        b: Element<S>,
    },
    String {
        hydrogen: Element<S>,
        sulfur: Element<S>,
        s: String,
    },
    Pair {
        hydrogen: Element<S>,
        p: Box<(Literal<S>, Literal<S>)>,
    },
    List {
        hydrogen: Element<S>,
        l: Vec<Literal<S>>,
    },
    Map {
        hydrogen: Element<S>,
        m: Vec<(Literal<S>, Literal<S>)>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element<S: Symbol>(pub S);

#[derive(Debug, Clone, PartialEq)]
pub struct Number(pub i64);
