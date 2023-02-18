use petgraph::stable_graph::{EdgeIndex, NodeIndex};

use crate::eval::{
    atom_like::AtomLike,
    atoms::Atoms,
    molecule::Molecule,
    value::{Valuable, Value, Weighable},
};

#[derive(Debug, Clone)]
pub struct Amine(pub Atoms);

impl Valuable for Amine {
    fn value(&self) -> Value {
        // turn this into a pair
        // the smaller-indexed molecule is the first of the pair
        // the larger-indexed molecule is the second of the pair
        let molecules: Vec<&Molecule> = self
            .0
            .atoms()
            .neighbors(self.0.head)
            .map(|atom| self.0.atoms().node_weight(atom).unwrap())
            .collect();
        let Some(Molecule::F(p1)) = molecules.first() else {panic!("failed to get first of pair")};
        let Some(Molecule::F(p2)) = molecules.last() else {panic!("failed to get second of pair")};
        let p1 = p1.value();
        let p2 = p2.value();
        Value::Pair((Box::new(p1), Box::new(p2)))
    }
}

impl AtomLike for Amine {
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

impl Weighable for Amine {
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
