mod eval;
mod lex;
mod tok;
mod util;

use eval::alkane::AlkaneElement;
use eval::functional_groups::ether::Ether;
use eval::value::Valuable;
use petgraph::dot::{self, Config};
use petgraph::stable_graph::StableUnGraph;

use crate::eval::alkane::Alkane;
use crate::eval::atom_like::AtomLike;
use crate::eval::functional_groups::FunctionalGroup;
use crate::eval::value::Weighable;
use crate::eval::{element::Element, molecule::Molecule};

fn main() {
    // let code = r"C_3H_8+5O_2 -> 3CO_2+4H_2O".to_string();
    // let mut lexer = lex::Lexer::new(code.to_string());

    // let tokens = lexer.all_tokens();
    // println!("{:?}", tokens);

    // let mut alk = Alkane::new();
    // println!("{:?}", &alk);

    // let mut tree: Tree<Atom> = Tree::new();

    let dot_config = [Config::EdgeNoLabel];

    // let mut alk = Alkane::new_n_alkane(4);
    // let ane = Alkane::new_n_alkane(2);
    // alk.move_down();
    // alk.move_down();
    // alk.add_alkane(ane);
    // let eth = FunctionalGroup::new_ether(Molecule::E(Element::N));
    // let bor = FunctionalGroup::new_borinic_acid(Molecule::Fg(eth.clone()));
    // alk.move_down();
    // alk.add_functional_group(bor.clone());

    // dbg!(&eth.value());
    // dbg!(&bor.value());

    let h = FunctionalGroup::new_ether(Molecule::E(Element::Hf));
    let e = FunctionalGroup::new_ether(Molecule::E(Element::Md));
    let l = FunctionalGroup::new_ether(Molecule::E(Element::Hs));
    let o = FunctionalGroup::new_ether(Molecule::E(Element::Rg));

    let mut hello = Alkane::new_n_alkane(5);
    hello.fill(vec![
        AlkaneElement::F(h.clone()),
        AlkaneElement::F(e.clone()),
        AlkaneElement::F(l.clone()),
        AlkaneElement::F(l.clone()),
        AlkaneElement::F(o.clone()),
    ]);

    let sulfide = FunctionalGroup::new_sulfide(Molecule::Fg(FunctionalGroup::Alkane(hello)));
    dbg!(sulfide.value());

    println!(
        "{:?}",
        dot::Dot::with_config(&sulfide.flatten().atoms(), &dot_config)
    );
}
