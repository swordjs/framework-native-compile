use std::fs; 
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "parser/proto.pest"] // relative to src
pub struct TypeScriptParser;

pub fn parse(path: &str) {
    let file = fs::read_to_string(path).unwrap();
    let pairs = TypeScriptParser::parse(Rule::, &file).unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs {
        println!("{:?}", pair);
    }
}
