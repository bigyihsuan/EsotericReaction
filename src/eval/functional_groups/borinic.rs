use std::ops::Add;

use crate::eval::{
    atom_like::AtomLike, atoms::Atoms, element::Element, molecule::Molecule, traits::Valuable,
    value::Value,
};

use super::{fg_macros, FgElement};

#[derive(Debug, Clone)]
pub struct BorinicAcid(Atoms);

impl BorinicAcid {
    pub fn new() -> BorinicAcid {
        let mut atoms = Atoms::new();
        atoms.add_node(Molecule::E(Element::B));
        atoms.add_node(Molecule::E(Element::O));
        atoms.add_node(Molecule::E(Element::H));
        BorinicAcid(atoms)
    }

    pub fn new_with(r: FgElement) -> BorinicAcid {
        let mut atoms = Atoms::new();
        let b = atoms.add_node(Molecule::E(Element::B));
        let o = atoms.add_node(Molecule::E(Element::O));
        let h = atoms.add_node(Molecule::E(Element::H));
        let r = atoms.add_node(r.as_molecule());
        atoms.add_edge(b, o);
        atoms.add_edge(o, h);
        atoms.add_edge(b, r);
        BorinicAcid(atoms)
    }
}

impl Valuable for BorinicAcid {
    fn value(&self) -> Value {
        // true if the head has more than 1 neighbor
        let atoms = &self;
        Value::Boolean(atoms.atoms().neighbors(atoms.head).count() > 1)
    }
}

fg_macros::fg!(BorinicAcid);

impl From<Value> for BorinicAcid {
    fn from(value: Value) -> Self {
        if let Value::Boolean(b) = value {
            BorinicAcid::from(b)
        } else {
            BorinicAcid::from(false)
        }
    }
}

impl From<bool> for BorinicAcid {
    fn from(value: bool) -> Self {
        let mut bor = BorinicAcid::new();
        let head = bor.head;
        if value {
            let h = bor.add_node(Molecule::E(Element::H));
            bor.add_edge(head, h);
        }
        bor
    }
}

impl Add for BorinicAcid {
    type Output = BorinicAcid;

    fn add(self, rhs: Self) -> Self::Output {
        let l = self.value();
        let r = rhs.value();
        let v = l + r;
        BorinicAcid::from(v)
    }
}
