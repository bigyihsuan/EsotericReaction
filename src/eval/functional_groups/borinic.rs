use petgraph::stable_graph::{EdgeIndex, NodeIndex};

use crate::eval::{
    atom_like::AtomLike,
    atoms::Atoms,
    molecule::Molecule,
    value::{Valuable, Value, Weighable},
};

#[derive(Debug, Clone)]
pub struct BorinicAcid(pub Atoms);

impl AtomLike for BorinicAcid {
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

impl Weighable for BorinicAcid {
    fn atomic_weight(&self) -> i64 {
        let atoms = &self.0;
        atoms
            .atoms()
            .neighbors(atoms.head)
            .map(|neighbor| atoms.atoms().node_weight(neighbor).unwrap().atomic_weight())
            .sum()
    }
}

impl Valuable for BorinicAcid {
    fn value(&self) -> Value {
        // true if the head has more than 1 neighbor
        let atoms = &self.0;
        Value::Boolean(atoms.atoms().neighbors(atoms.head).count() > 1)
    }
}
