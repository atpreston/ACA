use pest_derive::Parser;
use pest::Parser;
use crate::processor::{Immediate, Instr, Operand, Program, Register};

use std::*;
#[derive(Parser)]
#[grammar = "ISA.pest"]
struct MyParser;

fn string_to_reg(string: &str) -> Operand {
    let reg: Register = string.trim().parse::<Register>().unwrap_or_else(|_| panic!("Register value {string} cannot be converted to u8"));
    return Operand::Reg(reg);
}

fn string_to_imm(string: &str) -> Operand {
    let imm = string.parse::<Immediate>().unwrap_or_else(|_| panic!("Immediate value {string} cannot be converted to i64"));
    return Operand::Imm(imm);
}

fn build_from_pairs<'a>(pairs: pest::iterators::Pair<'a, Rule>) -> Result<Vec<Operand>, &'a str> {
    let mut return_vec = Vec::<Operand>::new();
    for pair in pairs.clone().into_inner() { 
        match pair.as_rule() {
            Rule::register => return_vec.push(string_to_reg(&pair.as_str()[1..])),
            Rule::immediate => return_vec.push(string_to_imm(&pair.as_str()[1..])),
            _ => {let val = build_from_pairs(pairs.clone().into_inner().next().unwrap());
                return val.to_owned()}
        }
    }
    Ok(return_vec)
}

fn location_from_pair(pair: pest::iterators::Pair<'_, Rule>) -> Result<Operand, &'static str> {
    match pair.as_rule() {
        Rule::register => return Ok(Operand::Reg(pair.as_str()[1..].parse::<Register>().unwrap())),
        Rule::immediate => Ok(Operand::Imm(pair.as_str()[1..].parse::<Immediate>().unwrap())),
        Rule::label => Err("LABELS NOT IMPLEMENTED IN METHOD location_from_pair"),
        _ => Err("Cannot convert to location")
    }
}

fn reg_from_op(op: Operand) -> Result<Register, &'static str> {
    match op {
        Operand::Reg(reg) => Ok(reg),
        _ => Err("Cannot convert immediate to register")
    }
}

fn destination_from_operands(ops: &Vec<Operand>) -> Result<Register, &'static str> {
    if ops.len() > 0 {
        return reg_from_op(ops[0]);
    }
    else {Err("No operands given")}
}

pub fn parse(path: &str) -> Result<Program, &str> {
    let mut program: Program = vec![];
    let file = fs::read_to_string(path).expect("Cannot read file");
    println!("{}", file);
    let pairs: pest::iterators::Pairs<'_, Rule> = MyParser::parse(Rule::program, &file).expect("unsuccessful parse");
    for pair in pairs {
        match parse_pair(pair) {
            Ok(prog) => {for inst in prog {program.push(inst)}}
            Err(err) => {println!("ERROR in parse: {}", err); return Err("Unsuccessful parse of pair")},
        }
    }
    return Ok(program)
}

fn parse_pairs(pairs: pest::iterators::Pairs<'_, Rule>) -> Result<Program, &'static str> {
    let mut program: Program = vec![];
    for pair in pairs {
        match parse_pair(pair) {
            Ok(prg) => {for inst in prg {program.push(inst);}},
            Err(err) => {println!("ERROR in parse: {}", err); return Err("Unsuccessful parse of pair")},
        }
    }
    return Ok(program)
}

fn parse_pair(pair: pest::iterators::Pair<'_, Rule>) -> Result<Program, & 'static str>{
    let operands = build_from_pairs(pair.clone()).expect("Cannot recover operands");
    let destination = destination_from_operands(&operands);
    match pair.as_rule() {
        Rule::addinst => Ok(vec![Instr::Add(destination.unwrap(), operands[1], operands[2])]),
        Rule::subinst => Ok(vec![Instr::Sub(destination.unwrap(), operands[1], operands[2])]),
        Rule::mulinst => Ok(vec![Instr::Mul(destination.unwrap(), operands[1], operands[2])]),

        Rule::notinst => Ok(vec![Instr::Not(destination.unwrap(), operands[1])]),
        Rule::andinst => Ok(vec![Instr::And(destination.unwrap(), operands[1], operands[2])]),
        Rule::orinst => Ok(vec![Instr::Or(destination.unwrap(), operands[1], operands[2])]),
        Rule::xorinst => Ok(vec![Instr::Xor(destination.unwrap(), operands[1], operands[2])]),

        Rule::cpinst => Ok(vec![Instr::Cp(destination.unwrap(), operands[1])]),
        Rule::ldinst => Ok(vec![Instr::Ld(destination.unwrap(), operands[1])]),
        Rule::ldrinst => Ok(vec![Instr::Ldr(destination.unwrap(), operands[1], operands[2])]),
        Rule::stinst => Ok(vec![Instr::St(operands[0], operands[1])]),

        Rule::binst => Ok(vec![Instr::B(location_from_pair(pair.into_inner().next().unwrap()).unwrap())]), 
        Rule::jinst => Ok(vec![Instr::J(operands[1])]),
        Rule::bilzinst => Ok(vec![Instr::Bilz(reg_from_op(operands[0]).unwrap(), operands[1])]),
        Rule::jilzinst => Ok(vec![Instr::Jilz(reg_from_op(operands[0]).unwrap(), location_from_pair(pair.into_inner().next().unwrap()).unwrap())]),
        Rule::jiltinst => Ok(vec![Instr::Jilt(reg_from_op(operands[0]).unwrap(), operands[1], operands[2])]),
        Rule::biltinst => Ok(vec![Instr::Bilt(reg_from_op(operands[0]).unwrap(), operands[1], location_from_pair(pair).unwrap())]), // TODO: THIS WILL NOT WORK - LOCATION IS WRONG

        Rule::noopinst => Ok(vec![Instr::Noop()]),
        Rule::haltinst => Ok(vec![Instr::Halt()]),

        Rule::label => Err("LOCATION NOT IMPLEMENTED"),
        _ => {return parse_pairs(pair.into_inner())}
    }
}