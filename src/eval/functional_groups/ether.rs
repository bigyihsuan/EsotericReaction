use petgraph::stable_graph::{EdgeIndex, NodeIndex};

use crate::eval::{
    atom_like::AtomLike,
    atoms::Atoms,
    element::Element,
    molecule::Molecule,
    value::{Valuable, Value, Weighable},
};

use super::FgElement;

#[derive(Debug, Clone)]
pub struct Ether(pub Atoms);

impl Ether {
    pub fn new() -> Ether {
        let mut atoms = Atoms::new();
        let e = atoms.add_node(Molecule::E(Element::O));
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

impl AtomLike for Ether {
    fn get_atoms(&self) -> &Atoms {
        &self.0
    }

    fn get_atoms_mut(&mut self) -> &mut Atoms {
        &mut self.0
    }

    fn flatten(&self) -> Atoms {
        self.0.flatten()
    }

    fn add_node(&mut self, m: Molecule) -> NodeIndex {
        self.0.add_node(m)
    }

    fn add_edge(&mut self, m: NodeIndex, n: NodeIndex) -> EdgeIndex {
        self.0.add_edge(m, n)
    }
}

impl Weighable for Ether {
    fn atomic_numbers(&self) -> i64 {
        let atoms = &self.0;
        atoms
            .atoms()
            .neighbors(atoms.head)
            .map(|neighbor| {
                atoms
                    .atoms()
                    .node_weight(neighbor)
                    .unwrap()
                    .atomic_numbers()
            })
            .sum()
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

macro_rules! from_for_ether {
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

from_for_ether!(u8);
from_for_ether!(u16);
from_for_ether!(u32);
from_for_ether!(u64);
