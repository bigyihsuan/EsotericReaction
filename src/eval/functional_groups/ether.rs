use petgraph::stable_graph::{EdgeIndex, NodeIndex};

use crate::eval::{
    atom_like::AtomLike,
    atoms::Atoms,
    element::Element,
    molecule::Molecule,
    value::{Valuable, Value, Weighable},
};

use super::{fg_macros, FgElement};

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

impl From<char> for Ether {
    fn from(value: char) -> Ether {
        let mut ether = Ether::new();
        let e = Element::from(value as u32);
        let e = ether.0.add_node(Molecule::E(e));
        ether.0.add_edge(ether.0.head, e);
        ether
    }
}

fg_macros::fg!(Ether);

macro_rules! impl_from_for_ether {
    ($t:ty) => {
        impl From<$t> for Ether {
            fn from(value: $t) -> Ether {
                let mut ether = Ether::new();
                let e = Element::from(value);
                let e = ether.0.add_node(Molecule::E(e));
                ether.0.add_edge(ether.0.head, e);
                ether
            }
        }
    };
}

impl_from_for_ether!(u8);
impl_from_for_ether!(u16);
impl_from_for_ether!(u32);
impl_from_for_ether!(u64);
impl_from_for_ether!(i8);
impl_from_for_ether!(i16);
impl_from_for_ether!(i32);
impl_from_for_ether!(i64);
