use petgraph::stable_graph::{EdgeIndex, NodeIndex};

use crate::eval::{
    atom_like::AtomLike,
    atoms::Atoms,
    molecule::Molecule,
    value::{Valuable, Value, Weighable},
};

#[derive(Debug, Clone)]
pub struct Ether(pub Atoms);

impl Weighable for Ether {
    fn atomic_weight(&self) -> i64 {
        let atoms = &self.0;
        atoms
            .atoms()
            .neighbors(atoms.head)
            .map(|neighbor| atoms.atoms().node_weight(neighbor).unwrap().atomic_weight())
            .sum()
    }
}

impl Valuable for Ether {
    fn value(&self) -> Value {
        // get the sum of the atomic numbers
        Value::Number(self.atomic_weight())
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
