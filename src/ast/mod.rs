extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "ast/proto.pest"] // relative to src
pub struct TypeScriptParser;

fn main(){
    let pairs = TypeScriptParser::parse(Rule::COMMENT, "a1 b2");
}