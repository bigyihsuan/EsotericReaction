use std::collections::{HashMap, VecDeque};

use petgraph::{
    data::DataMap,
    graph::Node,
    stable_graph::{NodeIndex, StableUnGraph},
    visit::IntoNeighbors,
};

#[derive(Debug, Clone)]
pub struct Alkane {
    pub atoms: StableUnGraph<Molecule, ()>,
    current_atom: NodeIndex,
    backbone: VecDeque<NodeIndex>,
}

impl Alkane {
    pub fn new() -> Alkane {
        let mut atoms = StableUnGraph::default();
        let current_atom = atoms.add_node(Molecule::E(Element::AlkaneHeader));
        let mut backbone = VecDeque::new();
        backbone.push_back(current_atom);
        Alkane {
            atoms,
            current_atom,
            backbone,
        }
    }

    pub fn new_n_alkane(n: usize) -> Alkane {
        let mut alk = Alkane::new();
        for _ in 0..n {
            alk.add_carbon_after();
            alk.move_down();
        }
        alk
    }

    pub fn add_carbon_after(&mut self) {
        let carbon = self.atoms.add_node(Molecule::E(Element::Carbon));
        if let Some(Molecule::E(Element::AlkaneHeader)) = self.atoms.node_weight(self.current_atom)
        {
            // current atom is an alkane header; try to connect the new carbon with the first child
            if let Some(first_child) = self
                .atoms
                .neighbors(self.current_atom)
                .filter(|neighbor| {
                    if let Some(&Molecule::E(Element::Carbon)) = self.atoms.node_weight(*neighbor) {
                        true
                    } else {
                        false
                    }
                })
                .collect::<Vec<_>>()
                .first()
                .cloned()
            {
                // there is a child, put the carbon in between the header and the child
                self.atoms.add_edge(self.current_atom, carbon, ());
                self.atoms.add_edge(carbon, first_child, ());
                let existing_edge = self.atoms.find_edge(self.current_atom, first_child);
                if let Some(existing_edge) = existing_edge {
                    self.atoms.remove_edge(existing_edge);
                }
                return;
            }
            // there is no first child, do the regular thing
        }
        self.atoms.add_edge(self.current_atom, carbon, ());
        // get the index to add after
        let current_index = self.get_atom_index(self.current_atom);
        match current_index {
            Some(idx) => self.backbone.insert(idx + 1, carbon),
            None => self.backbone.push_back(carbon), // can't find the current atom, add to the end
        }
    }
    pub fn add_carbon_before(&mut self) {
        let carbon = self.atoms.add_node(Molecule::E(Element::Carbon));

        if let Some(Molecule::E(Element::AlkaneHeader)) = self.atoms.node_weight(self.current_atom)
        {
            // current atom is an alkane header; try to connect the new carbon with the first child
            if let Some(first_child) = self
                .atoms
                .neighbors(self.current_atom)
                .filter(|neighbor| {
                    if let Some(&Molecule::E(Element::Carbon)) = self.atoms.node_weight(*neighbor) {
                        true
                    } else {
                        false
                    }
                })
                .collect::<Vec<_>>()
                .first()
                .cloned()
            {
                // there is a child, put the carbon in between the header and the child
                self.atoms.add_edge(self.current_atom, carbon, ());
                self.atoms.add_edge(carbon, first_child, ());
                let existing_edge = self.atoms.find_edge(self.current_atom, first_child);
                if let Some(existing_edge) = existing_edge {
                    self.atoms.remove_edge(existing_edge);
                }
                return;
            }
            // there is no first child, do the regular thing
        }

        let previous_index = self.get_current_atom_index();
        match previous_index {
            Some(idx) if idx > 0 => {
                let prev_node = self.backbone.get(idx - 1).unwrap().clone();
                let prev_mol = self.atoms.node_weight(prev_node).unwrap();
                if let &Molecule::E(Element::AlkaneHeader) = prev_mol {
                    // the previous atom is an alkane
                    // detatch the alkane and this node
                    let (alkane, alkane_idx) = (prev_node, idx);
                    let alk_curr_edge = self.atoms.find_edge(alkane, self.current_atom).unwrap();
                    self.atoms.remove_edge(alk_curr_edge);
                    // connect the new carbon to the current atom
                    self.atoms.add_edge(carbon, self.current_atom, ());
                    // connect the alkane to the new carbon
                    self.atoms.add_edge(alkane, carbon, ());
                    self.backbone.insert(alkane_idx, carbon);
                } else {
                    // the previous atom is not an alkane
                    // detach previous and current
                    let prev_curr_edge =
                        self.atoms.find_edge(prev_node, self.current_atom).unwrap();
                    self.atoms.remove_edge(prev_curr_edge);
                    // insert the carbon
                    self.atoms.add_edge(prev_node, carbon, ());
                    self.atoms.add_edge(carbon, self.current_atom, ());
                    self.backbone.insert(idx, carbon);
                }
            }
            Some(_idx) => {
                // current atom is the first alkane
                // attach carbon to this
                self.atoms.add_edge(self.current_atom, carbon, ());
                self.backbone.push_back(carbon);
                // move to the new carbon
                self.current_atom = carbon;
            }
            None => panic!("shouldn't have nothing before an atom"),
        }
    }

    pub fn add_molecule(&mut self, mol: Molecule) {
        let m = self.atoms.add_node(mol);
        self.atoms.add_edge(self.current_atom, m, ());
    }
    pub fn add_element(&mut self, ele: Element) {
        let e = self.atoms.add_node(Molecule::E(ele));
        self.atoms.add_edge(self.current_atom, e, ());
    }
    pub fn add_functional_group(&mut self, fg: FunctionalGroup) {
        let f = self.atoms.add_node(Molecule::Fg(fg));
        self.atoms.add_edge(self.current_atom, f, ());
    }
    pub fn add_alkane(&mut self, alk: Alkane) {
        let a = self.atoms.add_node(Molecule::A(alk));
        self.atoms.add_edge(self.current_atom, a, ());
    }

    pub fn move_up(&mut self) -> bool {
        let current_index = self.get_current_atom_index();
        match current_index {
            Some(idx) if idx > 0 => {
                self.current_atom = *self.backbone.get(idx - 1).unwrap();
                true
            }
            Some(_) => false,
            None => false,
        }
    }
    pub fn move_down(&mut self) -> bool {
        let current_index = self.get_current_atom_index();
        match current_index {
            Some(idx) => {
                self.current_atom = *self.backbone.get(idx + 1).unwrap();
                true
            }
            None => false,
        }
    }

    fn get_atom_index(&self, target: NodeIndex) -> Option<usize> {
        self.backbone.iter().position(|e| *e == target)
    }
    fn get_current_atom_index(&self) -> Option<usize> {
        self.get_atom_index(self.current_atom)
    }

    // recursively flatten this alkane by attaching the nodes of child alkanes to their parent
    pub fn flatten(&self) -> StableUnGraph<Molecule, ()> {
        let mut alk = self.atoms.clone();
        self.atoms.node_indices().for_each(|node_idx| {
            if let Some(Molecule::A(child_alk)) = alk.node_weight(node_idx) {
                // found an alkane, get its header
                let header_idx = NodeIndex::from(0);
                let child_atoms = child_alk.flatten().clone();
                if let Some(Molecule::E(Element::AlkaneHeader)) =
                    child_atoms.node_weight(header_idx)
                {
                    // got its header
                    // insert all nodes in this alkane
                    let o2n_idx: HashMap<NodeIndex, NodeIndex> = child_atoms
                        .node_indices()
                        .map(|old_idx| {
                            let mut tup = (NodeIndex::default(), NodeIndex::default());
                            if let Some(old_node) = child_atoms.node_weight(old_idx) {
                                let new_idx = alk.add_node(old_node.clone());
                                tup = (old_idx, new_idx);
                            }
                            tup
                        })
                        .collect();
                    // for each edge in the child, recreate it in the parent
                    child_atoms.edge_indices().for_each(|edge_idx| {
                        let pair = child_atoms.edge_endpoints(edge_idx);
                        if let Some((l_old, r_old)) = pair {
                            let l_new = o2n_idx.get(&l_old);
                            let r_new = o2n_idx.get(&r_old);
                            if let (Some(&l_new), Some(&r_new)) = (l_new, r_new) {
                                alk.add_edge(l_new, r_new, ());
                            }
                        }
                    });
                    // attach the header to the main chain
                    let old_alk_idx = node_idx;
                    let header_new_idx = o2n_idx.get(&NodeIndex::from(0));
                    if let Some(&header_new_idx) = header_new_idx.clone() {
                        // remove the bond between the child alkane and the main chain

                        // check the neighbors of the child alkane for a carbon (should have exactly 1)
                        let chain_carbon = self
                            .atoms
                            .neighbors(old_alk_idx)
                            .filter(|node| {
                                if let Some(Molecule::E(Element::Carbon)) =
                                    self.atoms.node_weight(*node)
                                {
                                    true
                                } else {
                                    false
                                }
                            })
                            .collect::<Vec<_>>()
                            .get(0)
                            .unwrap()
                            .clone();
                        let old_edge = alk.find_edge(chain_carbon, old_alk_idx);
                        if let Some(old_edge) = old_edge.clone() {
                            alk.remove_edge(old_edge);
                        }
                        // remove the old alkane
                        alk.remove_node(old_alk_idx);
                        // attach header to the chain
                        alk.add_edge(chain_carbon, header_new_idx, ());
                    }
                }
            }
        });
        alk
    }
}

#[derive(Debug, Clone)]
pub enum Molecule {
    E(Element),
    Fg(FunctionalGroup),
    A(Alkane),
}

#[derive(Debug, Clone)]
pub enum Element {
    AlkaneHeader,
    Hydrogen,
    Carbon,
    Oxygen,
}

#[derive(Debug, Clone)]
pub enum FunctionalGroup {
    Hydride,
    Ether,
    Borinic,
    Sulfide,
    Amine,
}
