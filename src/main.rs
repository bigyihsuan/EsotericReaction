mod eval;
mod lex;
mod tok;
mod util;

use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

use clap::{ArgGroup, Parser};

use crate::lex::Lexer;

#[derive(Debug, Parser)]
#[command(name = "EsotericReaction")]
#[command(author = "bigyhsuan")]
#[command(version = "0.0.0")]
#[clap(group(ArgGroup::new("source").required(false).multiple(false).args(&["file", "code"])))]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
    #[arg(short, long, value_name = "CODE")]
    code: Option<String>,
}

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

    // let dot_config = [Config::EdgeNoLabel];
    // println!(
    //     "{:?}",
    //     dot::Dot::with_config(&thirteen.flatten().atoms(), &dot_config)
    // );

    let args = Args::parse();
    let source = match (args.file, args.code) {
        (None, Some(code)) => code,
        (Some(file), None) => fs::read_to_string(file)
            .unwrap_or_else(|err| panic!("could not read from stdin: {}", err)),
        (Some(_), Some(_)) => panic!("only 1 of `--file` or `--code` is allowed"),
        (None, None) => {
            let mut s = String::new();
            io::stdin()
                .read_to_string(&mut s)
                .unwrap_or_else(|err| panic!("could not read from stdin: {}", err));
            s
        }
    };

    let mut lexer = Lexer::new(source);
    let tokens = lexer.all_tokens();

    tokens.iter().for_each(|tok| println!("{}", tok))
}
