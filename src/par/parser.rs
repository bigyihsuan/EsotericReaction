use string_interner::StringInterner;

use crate::{
    grammar,
    intern::{Backend, Symbol},
    lexer,
    prelude::Result,
};

use super::parse_tree::Program;

pub fn parse<S: Symbol, B: Backend<S>>(
    filename: Option<std::path::PathBuf>,
    input: &str,
    interner: &mut StringInterner<B>,
) -> Result<Program<S>> {
    let stream = lexer::TokenStream::new(filename, input)?;

    println!("{:?}", &stream.tokens());

    let ast = grammar::module_parser::program(&stream, interner)?;
    Ok(ast)
}
