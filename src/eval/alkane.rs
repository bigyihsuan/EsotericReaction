use std::{collections::VecDeque, ops::Index};

use petgraph::{
    data::Build,
    stable_graph::{NodeIndex, StableUnGraph},
};

use super::functional_groups::FunctionalGroup;

#[derive(Debug)]
pub struct Alkane {
    pub atoms: StableUnGraph<Molecule, ()>,
    current_atom: NodeIndex,
    backbone: VecDeque<NodeIndex>,
}

impl Alkane {
    pub fn new() -> Alkane {
        let mut atoms = StableUnGraph::default();
        let current_atom = atoms.add_node(Molecule::Alkane);
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
        self.atoms.add_edge(self.current_atom, carbon, ());
        // get the index to add after
        let current_index = self.get_current_atom_index();
        match current_index {
            Some(idx) => self.backbone.insert(idx + 1, carbon),
            None => self.backbone.push_back(carbon), // can't find the current atom, add to the end
        }
    }
    pub fn add_carbon_before(&mut self) {
        let carbon = self.atoms.add_node(Molecule::E(Element::Carbon));
        let previous_index = self.get_current_atom_index();
        match previous_index {
            Some(idx) if idx > 0 => {
                let prev_node = self.backbone.get(idx - 1).unwrap().clone();
                let prev_mol = self.atoms.node_weight(prev_node).unwrap();
                if let &Molecule::Alkane = prev_mol {
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

    fn get_current_atom_index(&self) -> Option<usize> {
        self.backbone.iter().position(|e| *e == self.current_atom)
    }
}

#[derive(Debug)]
pub enum Molecule {
    E(Element),
    Fg(FunctionalGroup),
    Alkane, // TODO: store the backbone carbon indexes in here
}

#[derive(Debug)]
pub enum Element {
    Hydrogen,
    Carbon,
    Oxygen,
}
