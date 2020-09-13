use crate::{error::Error, token::Token};
use logos::Lexer;
use swc_css_ast::*;

pub type PResult<T> = Result<T, Error>;

pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
}

impl Parser<'_> {
    pub fn parse(&mut self) -> PResult<Stylesheet> {}

    fn parse_rule(&mut self) -> PResult<Rule> {}

    fn parse_at_rule(&mut self) -> PResult<AtRule> {
        expect!(self, "@");
    }

    fn parse_style_rule(&mut self) -> PResult<StyleRule> {}

    fn parse_property(&mut self) -> PResult<Property> {}
}