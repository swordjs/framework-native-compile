use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "proto.pest"] // relative to src
pub struct TypeScriptParser;

pub fn parse(file: &str) {
    println!("hello");
    let pairs = TypeScriptParser::parse(Rule::proto, &file).unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs {
        println!("{:?}", pair);
    }
}