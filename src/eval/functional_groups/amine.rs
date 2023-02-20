use crate::eval::{
    atom_like::AtomLike, atoms::Atoms, element::Element, molecule::Molecule, traits::Valuable,
    value::Value,
};

use super::{
    fg_macros::{self},
    FgElement, FunctionalGroup,
};

#[derive(Debug, Clone)]
pub struct Amine(pub Atoms);

impl Amine {
    pub fn new() -> Amine {
        let mut atoms = Atoms::new();
        atoms.add_node(Molecule::E(Element::N));
        Amine(atoms)
    }

    pub fn new_with(k: FgElement, v: FgElement) -> Amine {
        let mut atoms = Atoms::new();
        let n = atoms.add_node(Molecule::E(Element::N));
        let k = atoms.add_node(k.as_molecule());
        let v = atoms.add_node(v.as_molecule());
        // weirdly adding them in value-key order fixes the ordering when displaying
        atoms.add_edge(n, v);
        atoms.add_edge(n, k);
        Amine(atoms)
    }
}

impl Valuable for Amine {
    fn value(&self) -> Value {
        // turn this into a pair
        // the smaller-indexed molecule is the first of the pair
        // the larger-indexed molecule is the second of the pair
        let molecules: Vec<&Molecule> = self
            .atoms()
            .neighbors(self.head)
            .map(|atom| self.atoms().node_weight(atom).unwrap())
            .collect();
        let Some(Molecule::F(p1)) = molecules.first() else {panic!("failed to get first of pair")};
        let Some(Molecule::F(p2)) = molecules.last() else {panic!("failed to get second of pair")};
        let p1 = p1.value();
        let p2 = p2.value();
        Value::Pair(Box::new(p1), Box::new(p2))
    }
}

fg_macros::fg!(Amine);

impl From<Value> for Amine {
    fn from(value: Value) -> Self {
        if let Value::Pair(l, r) = value {
            Amine::from((*l, *r))
        } else {
            Amine::new()
        }
    }
}

impl From<(Value, Value)> for Amine {
    fn from((k, v): (Value, Value)) -> Self {
        let k = FgElement::F(FunctionalGroup::from(k));
        let v = FgElement::F(FunctionalGroup::from(v));
        Amine::new_with(k, v)
    }
}

fg_macros::ops!(Amine);
