use pest::Parser;
use pest_derive::Parser;

use std::fs;

use crate::processor::{self, *};


#[derive(Parser)]
#[grammar = "ISA.pest"]
struct MyParser;

pub fn parse_pest(path: &str) {
    let unparsed_file = fs::read_to_string(path).expect("cannot read file");
    let file = MyParser::parse(Rule::program, &unparsed_file)
                        .expect("unsuccessful parse")
                        .next().unwrap();
    let tokens = file.tokens();
    for token in tokens {
        println!("{:?}", token);
    }
}