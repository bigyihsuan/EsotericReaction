use petgraph::stable_graph::{EdgeIndex, NodeIndex};

use super::{atoms::Atoms, molecule::Molecule};

pub trait AtomLike {
    fn flatten(&self) -> Atoms;
    fn add_node(&mut self, m: Molecule) -> NodeIndex;
    fn add_edge(&mut self, m: NodeIndex, n: NodeIndex) -> EdgeIndex;
}
