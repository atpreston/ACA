// THIS FILE IS NOT USED BY THE FINAL SIMULATOR

use crate::processor::{Immediate, Instr, Operand, Program, RegisterIndex};
use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

use std::{collections::HashMap, *};
#[derive(Parser)]
#[grammar = "ISA.pest"]
struct MyParser;

fn string_to_reg(string: &str) -> Operand {
    let reg: u8 = string
        .trim()
        .parse::<u8>()
        .unwrap_or_else(|_| panic!("Register value {string} cannot be converted to u8"));
    return Operand::Reg(reg);
}

fn string_to_imm(string: &str) -> Operand {
    let imm = string
        .parse::<Immediate>()
        .unwrap_or_else(|_| panic!("Immediate value {string} cannot be converted to i64"));
    return Operand::Imm(imm);
}

fn build_from_pairs<'a>(pairs: pest::iterators::Pair<'a, Rule>) -> Result<Vec<Operand>, &'a str> {
    let mut return_vec = Vec::<Operand>::new();
    for pair in pairs.clone().into_inner() {
        match pair.as_rule() {
            Rule::register => return_vec.push(string_to_reg(&pair.as_str()[1..])),
            Rule::immediate => return_vec.push(string_to_imm(&pair.as_str()[1..])),
            _ => {
                let val = build_from_pairs(pairs.into_inner().next().unwrap());
                return val.to_owned();
            }
        }
    }
    Ok(return_vec)
}

fn location_from_pair(pair: pest::iterators::Pair<'_, Rule>) -> Result<Operand, &'static str> {
    match pair.as_rule() {
        Rule::register => {
            return Ok(Operand::Reg(
                pair.as_str()[1..].parse::<RegisterIndex>().unwrap(),
            ))
        }
        Rule::immediate => Ok(Operand::Imm(
            pair.as_str()[1..].parse::<Immediate>().unwrap(),
        )),
        Rule::label => Err("LABELS NOT IMPLEMENTED IN METHOD location_from_pair"),
        _ => Err("Cannot convert to location"),
    }
}

fn reg_from_op(op: Operand) -> Result<RegisterIndex, &'static str> {
    match op {
        Operand::Reg(reg) => Ok(reg),
        _ => Err("Cannot convert immediate to register"),
    }
}

fn destination_from_operands(ops: &Vec<Operand>) -> Result<RegisterIndex, &'static str> {
    if ops.len() > 0 {
        return reg_from_op(ops[0]);
    } else {
        Err("No operands given")
    }
}

fn get_label(pair: pest::iterators::Pair<'_, Rule>) -> Option<HashMap<&str, u8>> {
    match pair.as_rule() {
        Rule::label => Some([(pair.as_str(), 1 as u8)].iter().cloned().collect()),
        _ => return get_labels(pair.into_inner()),
    }
}

fn get_labels(pairs: Pairs<'_, Rule>) -> Option<HashMap<&str, u8>> {
    let mut ret_map: HashMap<&str, u8> = HashMap::new();
    for pair in pairs {
        match get_label(pair) {
            Some(new_map) => {
                ret_map.extend(new_map);
            }
            None => (),
        }
    }
    return Some(ret_map);
}

pub fn parse(path: &str) -> Result<Program, &str> {
    let mut program: Program = vec![];
    let file = fs::read_to_string(path).expect("Cannot read file");
    // println!("{}", file);
    let pairs: pest::iterators::Pairs<'_, Rule> =
        MyParser::parse(Rule::program, &file).expect("unsuccessful parse");
    println!("PAIRS: {:?}", pairs);
    let label_map = get_labels(pairs.clone());
    for pair in pairs.into_iter() {
        match parse_pair(pair) {
            Ok(mut prog) => {
                println!("Program appended: {:?}", prog);
                program.append(&mut prog)
            }
            Err(err) => {
                println!("ERROR in parse: {}", err);
                return Err("Unsuccessful parse of pair");
            }
        }
    }
    return Ok(program);
}

fn parse_pairs(pairs: pest::iterators::Pairs<'_, Rule>) -> Result<Program, &'static str> {
    let mut program: Program = vec![];
    for pair in pairs {
        match parse_pair(pair) {
            Ok(prg) => {
                program.extend(prg);
            }
            Err(err) => {
                println!("ERROR in parse: {}", err);
                return Err("Unsuccessful parse of pair");
            }
        }
    }
    return Ok(program);
}

fn parse_pair(pair: pest::iterators::Pair<'_, Rule>) -> Result<Program, &'static str> {
    let operands = build_from_pairs(pair.clone()).expect("Cannot recover operands");
    let destination = destination_from_operands(&operands);
    let unwrap_dest: u8 =
        destination.unwrap_or_else(|_| panic!("Destination {destination:#?} cannot be recovered"));
    match pair.as_rule() {
        Rule::addinst => Ok(vec![Instr::Add(unwrap_dest, operands[1], operands[2])]),
        Rule::subinst => Ok(vec![Instr::Sub(unwrap_dest, operands[1], operands[2])]),
        Rule::mulinst => Ok(vec![Instr::Mul(unwrap_dest, operands[1], operands[2])]),

        Rule::notinst => Ok(vec![Instr::Not(unwrap_dest, operands[1])]),
        Rule::andinst => Ok(vec![Instr::And(unwrap_dest, operands[1], operands[2])]),
        Rule::orinst => Ok(vec![Instr::Or(unwrap_dest, operands[1], operands[2])]),
        Rule::xorinst => Ok(vec![Instr::Xor(unwrap_dest, operands[1], operands[2])]),

        Rule::cpinst => Ok(vec![Instr::Cp(unwrap_dest, operands[1])]),
        Rule::ldinst => Ok(vec![Instr::Ld(unwrap_dest, operands[1])]),
        Rule::jinst => Ok(vec![Instr::J(operands[1])]),
        Rule::bilzinst => Ok(vec![Instr::Bilz(unwrap_dest, operands[1])]),

        Rule::noopinst => Ok(vec![Instr::Noop()]),
        Rule::haltinst => Ok(vec![Instr::Halt()]),

        Rule::label => {
            println!("{} is a label", pair.as_str());
            Err("LOCATION NOT IMPLEMENTED")
        }
        _ => return parse_pairs(pair.into_inner()),
    }
}
