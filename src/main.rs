mod eval;
mod lex;
mod tok;
mod util;

use petgraph::dot::{self, Config};

use crate::eval::atom_like::AtomLike;
use crate::eval::functional_groups::alkane::{Alkane, AlkaneElement};
use crate::eval::functional_groups::amine::Amine;

use crate::eval::functional_groups::ether::Ether;
use crate::eval::functional_groups::sulfide::Sulfide;
use crate::eval::functional_groups::{FgElement, FunctionalGroup};
use crate::eval::traits::Valuable;

fn main() {
    // let code = r"C_3H_8+5O_2 -> 3CO_2+4H_2O".to_string();
    // let mut lexer = lex::Lexer::new(code.to_string());

    // let tokens = lexer.all_tokens();
    // println!("{:?}", tokens);

    // let mut alk = Alkane::new();
    // println!("{:?}", &alk);

    // let mut tree: Tree<Atom> = Tree::new();

    let three = Ether::from(3);
    let ten = Ether::from(10);
    eprintln!("3={} 10={}", three.value(), ten.value());
    let thirteen = three - ten;

    let dot_config = [Config::EdgeNoLabel];
    println!(
        "{:?}",
        dot::Dot::with_config(&thirteen.flatten().atoms(), &dot_config)
    );
}
