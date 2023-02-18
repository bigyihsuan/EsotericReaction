use petgraph::stable_graph::{EdgeIndex, NodeIndex};

use super::{atoms::Atoms, molecule::Molecule};

pub trait AtomLike {
    fn get_atoms(&self) -> &Atoms;
    fn get_atoms_mut(&mut self) -> &mut Atoms;
    fn flatten(&self) -> Atoms;
    fn add_node(&mut self, m: Molecule) -> NodeIndex;
    fn add_edge(&mut self, m: NodeIndex, n: NodeIndex) -> EdgeIndex;
    // fn atomic_weight(&self) -> i64;
}
