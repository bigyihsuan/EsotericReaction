use std::collections::HashMap;

use petgraph::stable_graph::{EdgeIndex, NodeIndex, StableUnGraph};

use super::{atom_like::AtomLike, molecule::Molecule};

pub type AtomGraph = StableUnGraph<Molecule, ()>;

#[derive(Debug, Clone)]
pub struct Atoms {
    atoms: AtomGraph,
    pub head: NodeIndex,
}

impl Atoms {
    pub fn new() -> Self {
        Atoms {
            atoms: StableUnGraph::default(),
            head: NodeIndex::default(),
        }
    }

    pub fn atoms(&self) -> &AtomGraph {
        &self.atoms
    }
    pub fn mut_atoms(&mut self) -> &mut AtomGraph {
        &mut self.atoms
    }
}

impl AtomLike for Atoms {
    fn get_atoms(&self) -> &Atoms {
        &self
    }

    fn get_atoms_mut(&mut self) -> &mut Atoms {
        self
    }

    // recursively flatten this group of atoms by attaching the nodes of child molecules to their parent
    fn flatten(&self) -> Atoms {
        let mut flattened_atoms = self.clone();
        for node_idx in self.atoms.node_indices() {
            // for each neighbor...
            for neighbor_idx in self.atoms.neighbors(node_idx) {
                let neighbor = self.atoms.node_weight(neighbor_idx);
                // recursively flatten non-elemental molecules
                if let Some(Molecule::Fg(fg)) = &neighbor {
                    let child = fg.flatten();
                    let mut child_to_parent_indexes: HashMap<NodeIndex, NodeIndex> = HashMap::new();
                    // for each atom in the child, add it to the parent's graph
                    for child_idx in child.atoms.node_indices() {
                        let child_atom = child.atoms.node_weight(child_idx).unwrap().clone();
                        let parent_idx = flattened_atoms.add_node(child_atom);
                        // keep track of the mappings from child indexes to parent indexes
                        child_to_parent_indexes.insert(child_idx, parent_idx);
                    }
                    // rebuild all the edges of the child graph into the parent
                    for edge_idx in child.atoms.edge_indices() {
                        // get child edge
                        let edge = child.atoms.edge_endpoints(edge_idx);
                        if let Some((l_child, r_child)) = edge {
                            // get the new indexes of these nodes
                            let l_parent = child_to_parent_indexes.get(&l_child);
                            let r_parent = child_to_parent_indexes.get(&r_child);
                            if let (Some(&l), Some(&r)) = (l_parent, r_parent) {
                                flattened_atoms.add_edge(l, r);
                            }
                        }
                    }
                    // remove the old node
                    flattened_atoms.atoms.remove_node(neighbor_idx);
                    // connect the head of the new child nodes to the current node
                    let new_child_head = child_to_parent_indexes.get(&child.head).unwrap().clone();
                    flattened_atoms.add_edge(node_idx, new_child_head);
                }
            }
        }
        flattened_atoms
    }

    // add a molecule to this grouping of atoms.
    // if this is the first node in this grouping of atoms, also make it the head.
    fn add_node(&mut self, m: Molecule) -> NodeIndex {
        let m_idx = self.atoms.add_node(m);
        if self.atoms.node_count() == 1 {
            self.head = m_idx;
        }
        m_idx
    }

    // add an edge between 2 atoms.
    fn add_edge(&mut self, m: NodeIndex, n: NodeIndex) -> EdgeIndex {
        self.atoms.add_edge(m, n, ())
    }
}
