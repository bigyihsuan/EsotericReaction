use std::collections::vec_deque::Iter;
use std::collections::VecDeque;

use petgraph::stable_graph::{EdgeIndex, NodeIndex};
use petgraph::visit::IntoNeighbors;

use super::atom_like::AtomLike;
use super::atoms::Atoms;
use super::element::{Element, ElementHeader};
use super::functional_groups::FunctionalGroup;
use super::molecule::Molecule;

#[derive(Debug, Clone)]
pub enum AlkaneElement {
    E(Element),
    M(Molecule),
    F(FunctionalGroup),
}

#[derive(Debug, Clone)]
pub struct Alkane {
    pub chain: Atoms,
    pub current_atom: NodeIndex,
    backbone: VecDeque<NodeIndex>,
}

impl Alkane {
    pub fn new() -> Alkane {
        let mut atoms = Atoms::new();
        let current_atom = atoms.add_node(Molecule::E(Element::C));
        let mut backbone = VecDeque::new();
        backbone.push_back(current_atom);
        Alkane {
            chain: atoms,
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
        let carbon = self.chain.mut_atoms().add_node(Molecule::E(Element::C));
        if let Some(Molecule::E(Element::Header(ElementHeader::AlkaneHeader))) =
            self.chain.atoms().node_weight(self.current_atom)
        {
            // current atom is an alkane header; try to connect the new carbon with the first child
            if let Some(first_child) = self
                .chain
                .atoms()
                .neighbors(self.current_atom)
                .filter(|neighbor| {
                    if let Some(&Molecule::E(Element::C)) =
                        self.chain.atoms().node_weight(*neighbor)
                    {
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
                self.chain
                    .mut_atoms()
                    .add_edge(self.current_atom, carbon, ());
                self.chain.add_edge(carbon, first_child);
                let existing_edge = self.chain.atoms().find_edge(self.current_atom, first_child);
                if let Some(existing_edge) = existing_edge {
                    self.chain.mut_atoms().remove_edge(existing_edge);
                }
                return;
            }
            // there is no first child, do the regular thing
        }
        self.chain
            .mut_atoms()
            .add_edge(self.current_atom, carbon, ());
        // get the index to add after
        let current_index = self.get_atom_index(self.current_atom);
        match current_index {
            Some(idx) => self.backbone.insert(idx + 1, carbon),
            None => self.backbone.push_back(carbon), // can't find the current atom, add to the end
        }
    }
    pub fn add_carbon_before(&mut self) {
        let carbon = self.chain.mut_atoms().add_node(Molecule::E(Element::C));

        if let Some(Molecule::E(Element::Header(ElementHeader::AlkaneHeader))) =
            self.chain.atoms().node_weight(self.current_atom)
        {
            // current atom is an alkane header; try to connect the new carbon with the first child
            if let Some(first_child) = self
                .chain
                .atoms()
                .neighbors(self.current_atom)
                .filter(|neighbor| {
                    if let Some(&Molecule::E(Element::C)) =
                        self.chain.atoms().node_weight(*neighbor)
                    {
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
                self.chain
                    .mut_atoms()
                    .add_edge(self.current_atom, carbon, ());
                self.chain.mut_atoms().add_edge(carbon, first_child, ());
                let existing_edge = self.chain.atoms().find_edge(self.current_atom, first_child);
                if let Some(existing_edge) = existing_edge {
                    self.chain.mut_atoms().remove_edge(existing_edge);
                }
                return;
            }
            // there is no first child, do the regular thing
        }

        let previous_index = self.get_current_atom_index();
        match previous_index {
            Some(idx) if idx > 0 => {
                let prev_node = self.backbone.get(idx - 1).unwrap().clone();
                let prev_mol = self.chain.atoms().node_weight(prev_node).unwrap();
                if let &Molecule::E(Element::Header(ElementHeader::AlkaneHeader)) = prev_mol {
                    // the previous atom is an alkane
                    // detatch the alkane and this node
                    let (alkane, alkane_idx) = (prev_node, idx);
                    let alk_curr_edge = self
                        .chain
                        .atoms()
                        .find_edge(alkane, self.current_atom)
                        .unwrap();
                    self.chain.mut_atoms().remove_edge(alk_curr_edge);
                    // connect the new carbon to the current atom
                    self.chain.add_edge(carbon, self.current_atom);
                    // connect the alkane to the new carbon
                    self.chain.add_edge(alkane, carbon);
                    self.backbone.insert(alkane_idx, carbon);
                } else {
                    // the previous atom is not an alkane
                    // detach previous and current
                    let prev_curr_edge = self
                        .chain
                        .atoms()
                        .find_edge(prev_node, self.current_atom)
                        .unwrap();
                    self.chain.mut_atoms().remove_edge(prev_curr_edge);
                    // insert the carbon
                    self.chain.add_edge(prev_node, carbon);
                    self.chain.add_edge(carbon, self.current_atom);
                    self.backbone.insert(idx, carbon);
                }
            }
            Some(_idx) => {
                // current atom is the first alkane
                // attach carbon to this
                self.chain.add_edge(self.current_atom, carbon);
                self.backbone.push_back(carbon);
            }
            None => panic!("shouldn't have nothing before an atom"),
        }
    }

    pub fn add_molecule(&mut self, mol: Molecule) {
        let m = self.chain.mut_atoms().add_node(mol);
        self.chain.add_edge(self.current_atom, m);
    }
    pub fn add_element(&mut self, ele: Element) {
        let e = self.chain.mut_atoms().add_node(Molecule::E(ele));
        self.chain.add_edge(self.current_atom, e);
    }
    pub fn add_functional_group(&mut self, fg: FunctionalGroup) {
        let f = self.chain.mut_atoms().add_node(Molecule::Fg(fg));
        self.chain.add_edge(self.current_atom, f);
    }
    pub fn add_alkane(&mut self, alk: Alkane) {
        let a = self
            .chain
            .add_node(Molecule::Fg(FunctionalGroup::Alkane(alk)));
        self.chain.add_edge(self.current_atom, a);
    }

    pub fn move_up(&mut self) -> bool {
        let current_index = self.get_current_atom_index();
        match current_index {
            Some(idx) if idx > 0 => {
                self.current_atom = *self.backbone.get(idx - 1).unwrap_or(&NodeIndex::default());
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
                self.current_atom = *self.backbone.get(idx + 1).unwrap_or(&NodeIndex::default());
                true
            }
            None => false,
        }
    }

    pub fn fill(&mut self, stuff: Vec<AlkaneElement>) {
        let old_current = self.current_atom.clone();
        // go to the head
        self.current_atom = *self.backbone.front().unwrap_or(&NodeIndex::default());
        // start attaching molecules
        for e in stuff {
            let e = e.clone();
            match e {
                AlkaneElement::E(e) => self.add_element(e),
                AlkaneElement::M(m) => self.add_molecule(m),
                AlkaneElement::F(f) => self.add_functional_group(f),
            }
            self.move_down();
        }
        self.current_atom = old_current
    }

    fn get_atom_index(&self, target: NodeIndex) -> Option<usize> {
        self.backbone.iter().position(|e| *e == target)
    }
    fn get_current_atom_index(&self) -> Option<usize> {
        self.get_atom_index(self.current_atom)
    }
    pub fn get_currently_bonded_molecules(&self) -> Vec<&Molecule> {
        self.get_bonded_molecules(self.current_atom)
    }
    pub fn get_bonded_molecules(&self, idx: NodeIndex) -> Vec<&Molecule> {
        self.chain
            .atoms()
            .neighbors(idx)
            .filter_map(|neighbor| {
                self.chain
                    .atoms()
                    .node_weight(neighbor)
                    .filter(|_| !self.backbone.contains(&neighbor))
            })
            .collect()
    }
    pub fn backbone(&self) -> Iter<NodeIndex> {
        self.backbone.iter()
    }
}

impl AtomLike for Alkane {
    fn get_atoms(&self) -> &Atoms {
        &self.chain
    }
    fn get_atoms_mut(&mut self) -> &mut Atoms {
        &mut self.chain
    }
    fn flatten(&self) -> Atoms {
        self.chain.flatten()
    }

    fn add_node(&mut self, m: Molecule) -> NodeIndex {
        self.chain.add_node(m)
    }

    fn add_edge(&mut self, m: NodeIndex, n: NodeIndex) -> EdgeIndex {
        self.chain.add_edge(m, n)
    }
}
