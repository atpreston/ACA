use pest::Parser;
use pest_derive::Parser;

use crate::processor::{self, *};


#[derive(Parser)]
#[grammar = "./pareser/ISA.pest"]
struct MyParser;

pub fn parse_pest(path: &str) -> Result<Program, &str> {
    let unparsed_file = fs::read_to_string(path).expect("cannot read file");
    let file = MyParser::parse(Rule::file, &unparsed_file)
                        .expect("unsuccessful parse")
                        .next().unwrap();
    let tokens = file.tokens();
    println!("{:?", token);
}



pub fn parse(string: &str) -> Result<Program, &str> {
    use processor::Instr::*;
    let mut program : Program = Program::new();
    let lines = string.lines();
    for line in lines {
        match &line[..3] {
            "Add" => {
                let ops = line[..4];

            }
            "Sub" => {

            }
            "Mul" => {

            }
            "Not" => {
            
            }
            "And" => {

            }
            "Or " => {

            }
            "Xor" => {

            }
            "Cp" => {

            }
            "Mv " => {

            }
            "Ld " => {

            }
            "Ldr" => {

            }
            "St " => {

            }
            "B" => { // WILL THIS WORK?

            }
            "J" => {

            }
            "Jil" => {

            }
            "Bil" => {

            }
            "Noo" => {

            }
            "Hal" => {

            }
            _ => return Err("Operand not recognised")
        }
    }
    return Ok(program);
}