use crate::intern::{Backend, StringInterner, Symbol};
use crate::lexer::{Token, TokenStream};
use crate::par::parse_tree::*;

peg::parser! {
    pub grammar module_parser<'source>() for TokenStream<'source> {
        pub rule program<S:Symbol, B: Backend<S>>(interner: &mut StringInterner<B>) -> Program<S>
            = exprs:equation(interner)+
            { Program(exprs) }

        pub rule equation<S: Symbol, B: Backend<S>>(interner: &mut StringInterner<B>) -> Equation<S>
            = lhs:reagent(interner) [Token::Arrow] rhs:reagent(interner) [Token::Period]
            { Equation{lhs, rhs} }

        pub rule reagent<S: Symbol, B: Backend<S>>(interner: &mut StringInterner<B>) -> Reagent<S>
            = terms:(term(interner) ** [Token::Plus])
            { Reagent(terms) }

        pub rule term<S: Symbol, B: Backend<S>>(interner: &mut StringInterner<B>) -> Term<S>
            = coeff:number()? mol:molecule(interner)+
            { Term{coeff, mol} }

        pub rule molecule<S: Symbol, B: Backend<S>>(interner: &mut StringInterner<B>) -> Molecule<S>
            = mol:molecule_contents(interner) sub:subscript()?
            { Molecule{mol, sub} }

        pub rule molecule_contents<S: Symbol, B: Backend<S>>(interner: &mut StringInterner<B>) -> MoleculeContents<S>
            = molecule_literal(interner) / molecule_nested(interner) / molecule_element(interner)

        pub rule molecule_nested<S: Symbol, B: Backend<S>>(interner: &mut StringInterner<B>) -> MoleculeContents<S>
            = [Token::LParen] mol:molecule(interner) [Token::RParen]
            { MoleculeContents::Nested(Box::new(mol)) }

        pub rule molecule_element<S: Symbol, B: Backend<S>>(interner: &mut StringInterner<B>) -> MoleculeContents<S>
            = ele:element(interner)
            { MoleculeContents::Element(ele) }

        pub rule molecule_literal<S: Symbol, B: Backend<S>>(interner: &mut StringInterner<B>) -> MoleculeContents<S>
            = lit:literal(interner)
            { MoleculeContents::Literal(lit) }

        pub rule literal<S: Symbol, B: Backend<S>>(interner: &mut StringInterner<B>) -> Literal<S>
            = literal_number(interner)
            / literal_boolean(interner)
            / literal_string(interner)
            / literal_pair(interner)
            / literal_list(interner)
            / literal_map(interner)

        pub rule literal_number<S: Symbol, B: Backend<S>>(interner: &mut StringInterner<B>) -> Literal<S>
            = hydrogen:ele(interner, "H") [Token::Caret] n:number()
            { Literal::Number{hydrogen, n} }

        pub rule literal_boolean<S: Symbol, B: Backend<S>>(interner: &mut StringInterner<B>) -> Literal<S>
            = hydrogen:ele(interner, "H") ([Token::Element(b) if b == "Tr" || b == "Fa"])
            { let b = interner.get_or_intern(b); Literal::Boolean{hydrogen, b: Element(b)} }

        pub rule literal_string<S: Symbol, B: Backend<S>>(interner: &mut StringInterner<B>) -> Literal<S>
            = hydrogen:ele(interner, "H") sulfur:ele(interner, "S") [Token::String(s)]
            { Literal::String{hydrogen, sulfur, s: s.clone()} }

        pub rule literal_pair<S: Symbol, B: Backend<S>>(interner: &mut StringInterner<B>) -> Literal<S>
            = hydrogen:ele(interner, "H") [Token::LAngle] l:literal(interner) [Token::Comma] r:literal(interner) [Token::RAngle]
            { Literal::Pair{hydrogen, p: Box::new((l,r))} }

        pub rule literal_list<S: Symbol, B: Backend<S>>(interner: &mut StringInterner<B>) -> Literal<S>
            = hydrogen:ele(interner, "H") [Token::LBracket] l:(literal(interner) ** [Token::Comma]) [Token::RBracket]
            { Literal::List{hydrogen, l} }

        pub rule literal_map<S: Symbol, B: Backend<S>>(interner: &mut StringInterner<B>) -> Literal<S>
            = hydrogen:ele(interner, "H") [Token::LBrace] m:(literal_map_element(interner) ** [Token::Comma]) [Token::RBrace]
            { Literal::Map{hydrogen, m} }

        pub rule literal_map_element<S: Symbol, B: Backend<S>>(interner: &mut StringInterner<B>) -> (Literal<S>, Literal<S>)
            = l:literal(interner) [Token::Colon] r:literal(interner)
            {(l,r)}

        pub rule element<S: Symbol, B: Backend<S>>(interner: &mut StringInterner<B>) -> Element<S>
            = [Token::Element(e)]
            { let e = interner.get_or_intern(e); Element(e) }

        pub rule ele<S: Symbol, B: Backend<S>>(interner: &mut StringInterner<B>, ele: &str) -> Element<S>
            = [Token::Element(e) if e == ele]
            { let h = interner.get_or_intern(ele); Element(h) }

        pub rule number() -> Number
            = [Token::Number(n)]
            { Number(*n) }

        pub rule subscript() -> Number
            = [Token::Underscore] n:number()
            { n }
    }
}
