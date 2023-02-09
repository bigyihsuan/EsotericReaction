use std::{collections::VecDeque, ops::Index};

use petgraph::stable_graph::{NodeIndex, StableUnGraph};

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
        let current_atom = atoms.add_node(Molecule::E(Element::Hydrogen));
        let backbone = VecDeque::new();
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
        }
        alk
    }

    pub fn add_carbon_after(&mut self) {
        if let Some(&Molecule::E(Element::Hydrogen)) = self.atoms.node_weight(self.current_atom) {
            // remove the empty hydrogen
            self.atoms.remove_node(self.current_atom);
            self.current_atom = self.atoms.add_node(Molecule::E(Element::Carbon));
            self.backbone.push_back(self.current_atom)
        } else {
            let new_atom = self.atoms.add_node(Molecule::E(Element::Carbon));
            self.atoms.add_edge(self.current_atom, new_atom, ());
            // get the index to add after
            let current_index = self.get_current_atom_index();
            match current_index {
                Some(idx) => self.backbone.insert(idx + 1, new_atom),
                None => self.backbone.push_back(new_atom), // can't find the current atom, add to the end
            }
            // self.atoms.add_edge(new_atom, self.current_atom, ());
        }
    }
    pub fn add_carbon_before(&mut self) {
        if let Some(&Molecule::E(Element::Hydrogen)) = self.atoms.node_weight(self.current_atom) {
            // remove the empty hydrogen
            self.atoms.remove_node(self.current_atom);
            self.current_atom = self.atoms.add_node(Molecule::E(Element::Carbon));
            self.backbone.push_back(self.current_atom)
        } else {
            let new_atom = self.atoms.add_node(Molecule::E(Element::Carbon));
            self.atoms.add_edge(new_atom, self.current_atom, ());
            // get the index to add before
            let current_index = self.get_current_atom_index();
            match current_index {
                Some(idx) => self.backbone.insert(idx, new_atom),
                None => self.backbone.push_front(new_atom), // can't find the current atom, add to the front
            }
            // self.atoms.add_edge(self.current_atom, new_atom, ());
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

    pub fn move_up(&mut self) {
        let current_index = self.get_current_atom_index();
        match current_index {
            Some(idx) if idx > 0 => self.current_atom = *self.backbone.get(idx - 1).unwrap(),
            Some(_) => self.current_atom = NodeIndex::default(),
            None => self.current_atom = NodeIndex::default(),
        }
    }
    pub fn move_down(&mut self) {
        let current_index = self.get_current_atom_index();
        match current_index {
            Some(idx) if idx < self.backbone.len() => {
                self.current_atom = *self.backbone.get(idx + 1).unwrap()
            }
            Some(_) => self.current_atom = NodeIndex::default(),
            None => self.current_atom = NodeIndex::default(),
        }
    }

    fn get_current_atom_index(&self) -> Option<usize> {
        self.backbone.iter().position(|e| e == &self.current_atom)
    }
}

#[derive(Debug)]
pub enum Molecule {
    E(Element),
    Fg(FunctionalGroup),
    A(Alkane),
}

#[derive(Debug)]
pub enum Element {
    Hydrogen,
    Carbon,
    Oxygen,
}
