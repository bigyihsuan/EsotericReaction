use crate::eval::{
    atom_like::AtomLike,
    atoms::Atoms,
    element::Element,
    molecule::Molecule,
    value::{Valuable, Value, Weighable},
};

use super::{
    alkane::{Alkane, AlkaneElement},
    ether::Ether,
    fg_macros, FgElement, FunctionalGroup,
};

#[derive(Debug, Clone)]
pub struct Sulfide(pub Atoms);

impl Sulfide {
    pub fn new() -> Sulfide {
        let mut atoms = Atoms::new();
        atoms.add_node(Molecule::E(Element::S));
        Sulfide(atoms)
    }

    pub fn new_with(r: FgElement) -> Sulfide {
        let mut atoms = Atoms::new();
        let s = atoms.add_node(Molecule::E(Element::S));
        let r = atoms.add_node(r.as_molecule());
        atoms.add_edge(s, r);
        Sulfide(atoms)
    }
}

impl Valuable for Sulfide {
    fn value(&self) -> Value {
        let mut chars = Vec::new();
        self
            .atoms()
            .neighbors(self.head)
            .map(|atom| self.atoms().node_weight(atom).unwrap())
            // sulfides should have either nothing, an ether, or an alkane
            // an element is a runtime error
            .for_each(|mol| {
                let Molecule::F(fg) = mol else {panic!("sulfides should not have elements, got {:?}", mol)};
                match fg {
                    FunctionalGroup::Alkane(a) => {
                        // alkanes are many characters
                        // iterate over the entire chain
                        let backbone_bonds: Vec<Vec<&Molecule>> = a
                            .backbone()
                            .map(|carbon| a.get_bonded_molecules(carbon.clone()))
                            .collect();

                        let mut cs: Vec<char> = backbone_bonds
                            .iter()
                            // only take the first bond found
                            .filter_map(|bonds| bonds.first())
                            // convert each to chars
                            .map(|m| {
                                let Molecule::F(FunctionalGroup::Ether(e)) = m else {panic!("expected ethers on string alkane, instead got {:?}", m)};
                                let v = e.value();
                                let Value::Number(c) = v else {panic!("expected number from ether, got {:?}", v)};
                                char::from_u32(c as u32).unwrap_or(char::REPLACEMENT_CHARACTER)
                            })
                            .collect();
                        chars.append(&mut cs);
                    },
                    FunctionalGroup::Ether(e) => {
                        // ethers are just a single character
                        let v = e.value();
                        let Value::Number(c) = v else {panic!("expected number from ether, got {:?}", v)};
                        chars.push(char::from_u32(c as u32).unwrap_or(char::REPLACEMENT_CHARACTER));
                    },
                    fg => panic!("expected nothing, ether, or alkane; got {:?}", fg)
                }
            });

        Value::String(chars)
    }
}

impl From<&str> for Sulfide {
    fn from(s: &str) -> Sulfide {
        let mut sulfide = Sulfide::new();
        let sulfur = sulfide.0.head;
        let mol = match s.chars().collect::<Vec<_>>().len() {
            // 0 = nothing
            0 => None,
            // 1 = single ether
            1 => Some(FunctionalGroup::Ether(Ether::from(
                s.chars().next().unwrap_or(char::REPLACEMENT_CHARACTER),
            ))),
            // 2+ = alkane
            _ => {
                let alk = Alkane::new_with(
                    s.chars()
                        .map(|c| AlkaneElement::F(FunctionalGroup::Ether(Ether::from(c))))
                        .collect(),
                );
                Some(FunctionalGroup::Alkane(alk))
            }
        };
        if let Some(mol) = mol {
            let mol = sulfide.add_node(Molecule::F(mol));
            sulfide.add_edge(sulfur, mol);
        }
        sulfide
    }
}
impl From<char> for Sulfide {
    fn from(c: char) -> Sulfide {
        let mut sulfide = Sulfide::new();
        let sulfur = sulfide.0.head;
        let mol = sulfide.add_node(Molecule::F(FunctionalGroup::Ether(Ether::from(c))));
        sulfide.add_edge(sulfur, mol);
        sulfide
    }
}

fg_macros::fg!(Sulfide);
