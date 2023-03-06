mod common;
mod eval;
mod grammar;
mod intern;
mod lexer;
mod par;
mod prelude;
mod util;

use logos::Logos;
use prelude::SyntaxError;
use string_interner::{symbol::SymbolUsize, DefaultBackend, StringInterner};
// use par::parser::Parser;
use util::args::parse_args;

use crate::{
    intern::{Backend, Symbol},
    lexer::TokenStream,
    par::parser::parse,
};

fn main() {
    // let code = r"C_3H_8+5O_2 -> 3CO_2+4H_2O".to_string();
    // let mut lexer = lex::Lexer::new(code.to_string());

    // let tokens = lexer.all_tokens();
    // println!("{:?}", tokens);

    // let mut alk = Alkane::new();
    // println!("{:?}", &alk);

    // let mut tree: Tree<Atom> = Tree::new();

    // let three = Ether::from(3);
    // let ten = Ether::from(10);
    // eprintln!("3={} 10={}", three.value(), ten.value());
    // let thirteen = three - ten;

    let (file, source) = parse_args();
    eprintln!("file: {file:?}\n```\n{source}```\n");

    type S = SymbolUsize;
    type B = DefaultBackend<S>;

    let mut interner: StringInterner<B> = StringInterner::new();

    let program = parse(file, &source, &mut interner);
    println!();
    match program {
        Ok(program) => {
            for equation in program.0 {
                println!("{equation:?}")
            }
        }
        Err(error) => eprintln!("{error}"),
    }
}
