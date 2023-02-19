mod eval;
mod lex;
mod tok;
mod util;

use eval::value::Valuable;
use petgraph::dot::{self, Config};

use crate::eval::atom_like::AtomLike;
use crate::eval::functional_groups::alkane::{Alkane, AlkaneElement};
use crate::eval::functional_groups::amine::Amine;
use crate::eval::functional_groups::borinic::BorinicAcid;
use crate::eval::functional_groups::ether::Ether;
use crate::eval::functional_groups::sulfide::Sulfide;
use crate::eval::functional_groups::{FgElement, FunctionalGroup};

fn main() {
    // let code = r"C_3H_8+5O_2 -> 3CO_2+4H_2O".to_string();
    // let mut lexer = lex::Lexer::new(code.to_string());

    // let tokens = lexer.all_tokens();
    // println!("{:?}", tokens);

    // let mut alk = Alkane::new();
    // println!("{:?}", &alk);

    // let mut tree: Tree<Atom> = Tree::new();

    let dot_config = [Config::EdgeNoLabel];

    let mut pairs: Vec<AlkaneElement> = (1..=10)
        .zip('a'..='z')
        .map(|(n, c)| {
            let eth = Ether::from(n);
            let sulf = Sulfide::from(c);
            AlkaneElement::F(FunctionalGroup::Amine(Amine::new_with(
                FgElement::F(FunctionalGroup::Ether(eth)),
                FgElement::F(FunctionalGroup::Sulfide(sulf)),
            )))
        })
        .collect();
    pairs.insert(
        4,
        AlkaneElement::F(FunctionalGroup::Sulfide(Sulfide::from(
            "INTERRUPTING SULFIDE",
        ))),
    );
    let alk = Alkane::new_with(pairs);

    eprintln!("{}", &alk.value());
    println!(
        "{:?}",
        dot::Dot::with_config(&alk.flatten().atoms(), &dot_config)
    );
}
