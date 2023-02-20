use std::collections::vec_deque::Iter;
use std::collections::{HashMap, VecDeque};

use petgraph::stable_graph::{EdgeIndex, NodeIndex};

use crate::eval::atom_like::AtomLike;
use crate::eval::atoms::Atoms;
use crate::eval::element::Element;
use crate::eval::molecule::Molecule;
use crate::eval::traits::Valuable;
use crate::eval::value::Value;

use super::{fg_macros, FunctionalGroup};

#[derive(Debug, Clone)]
pub enum AlkaneElement {
    E(Element),
    M(Molecule),
    F(FunctionalGroup),
}

#[derive(Debug, Clone)]
pub struct Alkane {
    chain: Atoms,
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
        let old_head = alk.head;
        for _ in 0..n {
            alk.add_carbon_after();
            alk.move_down();
        }
        alk.head = old_head;
        alk
    }

    pub fn new_with(stuff: Vec<AlkaneElement>) -> Alkane {
        let mut alk = Alkane::new_n_alkane(stuff.len());
        alk.fill(stuff);
        alk
    }

    pub fn add_carbon_after(&mut self) {
        let carbon = self.chain.mut_atoms().add_node(Molecule::E(Element::C));
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

        let previous_index = self.get_current_atom_index();
        match previous_index {
            Some(idx) if idx > 0 => {
                let prev_node = self.backbone.get(idx - 1).unwrap().clone();
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
            Some(_idx) => {
                // current atom is the first alkane
                // attach carbon to this
                self.chain.add_edge(self.current_atom, carbon);
                self.backbone.push_back(carbon);
            }
            None => panic!("shouldn't have nothing before an atom"),
        }
    }

    pub fn add(&mut self, stuff: AlkaneElement) {
        match stuff {
            AlkaneElement::E(e) => self.add_element(e),
            AlkaneElement::M(m) => self.add_molecule(m),
            AlkaneElement::F(f) => self.add_functional_group(f),
        }
    }

    pub fn add_molecule(&mut self, m: Molecule) {
        let m = self.chain.mut_atoms().add_node(m);
        self.chain.add_edge(self.current_atom, m);
    }
    pub fn add_element(&mut self, e: Element) {
        let e = self.chain.mut_atoms().add_node(Molecule::E(e));
        self.chain.add_edge(self.current_atom, e);
    }
    pub fn add_functional_group(&mut self, f: FunctionalGroup) {
        let f = self.chain.mut_atoms().add_node(Molecule::F(f));
        self.chain.add_edge(self.current_atom, f);
    }
    pub fn add_alkane(&mut self, alk: Alkane) {
        let a = self
            .chain
            .add_node(Molecule::F(FunctionalGroup::Alkane(alk)));
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

impl Valuable for Alkane {
    fn value(&self) -> Value {
        // 2 options:
        // - a map if and only if all functional groups are pairs
        // - a list otherwise

        // get the functional groups of all backbone atoms
        let backbone_bonds: Vec<Vec<&Molecule>> = self
            .backbone()
            .map(|carbon| self.get_bonded_molecules(carbon.clone()))
            .collect();
        let vec: Vec<Value> = backbone_bonds.iter().map(|v| v.iter().map(|mol| {
                let Molecule::F(fg) = mol else {panic!("inner alkanes should not have elements, got {:?}", mol)};
                fg.value()
            })).flatten().collect();

        // if everything in the vec is a pair...
        if vec.iter().all(|item| match item {
            Value::Pair(_, _) => true,
            _ => false,
        }) {
            // return hashmap instead
            let mut map: HashMap<Value, Value> = HashMap::new();
            vec.iter().for_each(|pair| {
                let Value::Pair(k, v) = pair else {panic!("expected only pairs in vec of pairs, got non-pair {:?}", pair)};
                map.insert(*k.clone(), *v.clone());
            });
            return Value::Map(map);
        }
        Value::List(vec)
    }
}

fg_macros::fg!(Alkane, chain);

impl From<Value> for Alkane {
    fn from(value: Value) -> Self {
        if let Value::List(v) = value {
            Alkane::new_with(
                v.iter()
                    .map(|e| AlkaneElement::F(FunctionalGroup::from(e.clone())))
                    .collect(),
            )
        } else if let Value::Map(m) = value {
            Alkane::new_with(
                m.iter()
                    .map(|(k, v)| {
                        AlkaneElement::F(FunctionalGroup::from(Value::from((k.clone(), v.clone()))))
                    })
                    .collect(),
            )
        } else {
            Alkane::new()
        }
    }
}

fg_macros::ops!(Alkane);
