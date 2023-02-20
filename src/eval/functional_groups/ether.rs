use crate::eval::{
    atom_like::AtomLike, atoms::Atoms, element::Element, molecule::Molecule, traits::Valuable,
    value::Value,
};

use super::{fg_macros, FgElement, FunctionalGroup};

#[derive(Debug, Clone)]
pub struct Ether(pub Atoms);

impl Ether {
    pub fn new() -> Ether {
        let mut atoms = Atoms::new();
        atoms.add_node(Molecule::E(Element::O));
        Ether(atoms)
    }

    pub fn new_with(r: FgElement) -> Ether {
        let mut atoms = Atoms::new();
        let o = atoms.add_node(Molecule::E(Element::O));
        let r = atoms.add_node(r.as_molecule());
        atoms.add_edge(o, r);
        Ether(atoms)
    }
}

impl Valuable for Ether {
    fn value(&self) -> Value {
        // get the sum of the atomic numbers
        Value::Number(self.atomic_numbers())
    }
}

fg_macros::fg!(Ether);

impl From<char> for Ether {
    fn from(value: char) -> Ether {
        let mut ether = Ether::new();
        let e = Element::from(value as u32);
        let e = ether.add_node(Molecule::E(e));
        let head = ether.head;
        ether.add_edge(head, e);
        ether
    }
}

impl From<Value> for Ether {
    fn from(value: Value) -> Self {
        if let Value::Number(n) = value {
            Ether::from(n)
        } else {
            Ether::from(0)
        }
    }
}

macro_rules! impl_from_for_ether {
    ($t:ty) => {
        impl From<$t> for Ether {
            fn from(value: $t) -> Ether {
                let mut ether = Ether::new();
                let e = Element::from(value);
                let e = ether.add_node(Molecule::E(e));
                ether.0.add_edge(ether.head, e);
                ether
            }
        }
    };
}

impl_from_for_ether!(u8);
impl_from_for_ether!(u16);
impl_from_for_ether!(u32);
impl_from_for_ether!(u64);
impl_from_for_ether!(u128);
impl_from_for_ether!(i8);
impl_from_for_ether!(i16);
impl_from_for_ether!(i32);
impl_from_for_ether!(i64);
impl_from_for_ether!(i128);

fg_macros::ops!(Ether);
