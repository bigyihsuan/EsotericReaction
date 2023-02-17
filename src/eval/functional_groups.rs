use super::alkane::Alkane;
use super::atom_like::AtomLike;
use super::atoms::{AtomGraph, Atoms};
use super::element::Element;
use super::molecule::Molecule;
use super::value::{Valuable, Value, Weighable};

#[derive(Debug, Clone)]
pub enum FunctionalGroup {
    Alkane(Alkane),
    Ether(Ether),
    BorinicAcid(BorinicAcid),
    Sulfide(Atoms),
    Amine(Atoms),
}
use FunctionalGroup as Fg;

impl FunctionalGroup {
    pub fn new_alkane() -> Self {
        let alkane = Alkane::new();
        Fg::Alkane(alkane)
    }
    pub fn new_ether(r: Molecule) -> Self {
        let mut ether = Atoms::new();
        let o = ether.add_node(Molecule::E(Element::O));
        let r_ = ether.add_node(r);
        ether.add_edge(o, r_);
        Fg::Ether(Ether(ether))
    }
    pub fn new_borinic_acid(r: Molecule) -> Self {
        let mut borinic_acid = Atoms::new();
        // add elements
        let b = borinic_acid.add_node(Molecule::E(Element::B));
        let o = borinic_acid.add_node(Molecule::E(Element::O));
        let h = borinic_acid.add_node(Molecule::E(Element::H));
        let r_ = borinic_acid.add_node(r);
        borinic_acid.add_edge(b, o);
        borinic_acid.add_edge(o, h);
        borinic_acid.add_edge(b, r_);
        Fg::BorinicAcid(BorinicAcid(borinic_acid))
    }
    pub fn new_sulfide() -> Self {
        let sulfide = Atoms::new();
        Fg::Sulfide(sulfide)
    }
    pub fn new_amine() -> Self {
        let amine = Atoms::new();
        Fg::Amine(amine)
    }

    pub fn unwrap(&self) -> &AtomGraph {
        match self {
            Fg::Ether(a) => a.0.atoms(),
            Fg::BorinicAcid(a) => a.0.atoms(),
            Fg::Sulfide(a) => a.atoms(),
            Fg::Amine(a) => a.atoms(),
            Fg::Alkane(a) => a.chain.atoms(),
        }
    }

    pub fn flatten(&self) -> Atoms {
        match self {
            Fg::Ether(a) => a.0.flatten(),
            Fg::BorinicAcid(a) => a.0.flatten(),
            Fg::Sulfide(a) => a.flatten(),
            Fg::Amine(a) => a.flatten(),
            Fg::Alkane(a) => a.flatten(), //&a.atoms,
        }
    }
}

impl Valuable for FunctionalGroup {
    fn value(&self) -> Value {
        match self {
            Fg::Alkane(_) => todo!(),
            Fg::Ether(eth) => eth.value(),
            Fg::BorinicAcid(bor) => bor.value(),
            Fg::Sulfide(_) => todo!(),
            Fg::Amine(_) => todo!(),
        }
    }
}

impl Weighable for FunctionalGroup {
    fn atomic_weight(&self) -> i64 {
        match self {
            Fg::Alkane(alk) => todo!(),
            Fg::Ether(eth) => eth.atomic_weight(),
            Fg::BorinicAcid(bor) => bor.atomic_weight(),
            Fg::Sulfide(sulf) => todo!(),
            Fg::Amine(am) => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ether(Atoms);

impl Weighable for Ether {
    fn atomic_weight(&self) -> i64 {
        let atoms = &self.0;
        atoms
            .atoms()
            .neighbors(atoms.head)
            .map(|neighbor| atoms.atoms().node_weight(neighbor).unwrap().atomic_weight())
            .sum()
    }
}

impl Valuable for Ether {
    fn value(&self) -> Value {
        // get the sum of the atomic numbers
        Value::Number(self.atomic_weight())
    }
}
#[derive(Debug, Clone)]
pub struct BorinicAcid(Atoms);

impl Weighable for BorinicAcid {
    fn atomic_weight(&self) -> i64 {
        let atoms = &self.0;
        atoms
            .atoms()
            .neighbors(atoms.head)
            .map(|neighbor| atoms.atoms().node_weight(neighbor).unwrap().atomic_weight())
            .sum()
    }
}

impl Valuable for BorinicAcid {
    fn value(&self) -> Value {
        // true if the head has more than 1 neighbor
        let atoms = &self.0;
        Value::Boolean(atoms.atoms().neighbors(atoms.head).count() > 1)
    }
}
