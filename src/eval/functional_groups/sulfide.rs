use petgraph::stable_graph::{EdgeIndex, NodeIndex};

use crate::eval::{
    atom_like::AtomLike,
    atoms::Atoms,
    molecule::Molecule,
    value::{Valuable, Value, Weighable},
};

use super::FunctionalGroup;

#[derive(Debug, Clone)]
pub struct Sulfide(pub Atoms);

impl Weighable for Sulfide {
    fn atomic_weight(&self) -> i64 {
        let atoms = &self.0;
        atoms
            .atoms()
            .neighbors(atoms.head)
            .map(|neighbor| atoms.atoms().node_weight(neighbor).unwrap().atomic_weight())
            .sum()
    }
}

impl Valuable for Sulfide {
    fn value(&self) -> Value {
        let mut chars = Vec::new();
        self.0
            .atoms()
            .neighbors(self.0.head)
            .map(|atom| self.0.atoms().node_weight(atom).unwrap())
            // sulfides should have either nothing, an ether, or an alkane
            // an element is a runtime error
            .for_each(|mol| {
                let Molecule::Fg(fg) = mol else {panic!("sulfides should not have elements, got {:?}", mol)};
                match fg {
                    FunctionalGroup::Alkane(a) => {
                        // alkanes are many characters
                        // iterate over the entire chain
                        let backbone_bonds: Vec<Vec<&Molecule>> = a
                            .backbone()
                            .map(|carbon| a.get_bonded_molecules(carbon.clone()))
                            .collect();

                        let mut cs: Vec<u8> = backbone_bonds
                            .iter()
                            // only take the first bond found
                            .filter_map(|bonds| bonds.first())
                            // convert each to chars
                            .map(|m| {
                                let Molecule::Fg(FunctionalGroup::Ether(e)) = m else {panic!("expected ethers on string alkane, instead got {:?}", m)};
                                let v = e.value();
                                let Value::Number(c) = v else {panic!("expected number from ether, got {:?}", v)};
                                c as u8
                            })
                            .collect();
                        chars.append(&mut cs);
                    },
                    FunctionalGroup::Ether(e) => {
                        // ethers are just a single character
                        let v = e.value();
                        let Value::Number(c) = v else {panic!("expected number from ether, got {:?}", v)};
                        chars.push(c as u8);
                    },
                    fg => panic!("expected nothing, ether, or alkane; got {:?}", fg)
                }
            });

        Value::String(chars)
    }
}

impl AtomLike for Sulfide {
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
