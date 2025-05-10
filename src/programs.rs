use crate::processor::Instr::*;
use crate::processor::*;
use crate::Operand::{Imm, Reg};

pub fn load_program(input: &str) -> Vec<Instr> {
    match input {
        "multest" => vec![
            Ld(1, Imm(200)),
            Mul(2, Reg(0), Reg(1)),
            Ld(3, Reg(2)),
            Halt(),
        ],

        "branchtest" => vec![Ld(0, Imm(1000)), Sub(0, Reg(0), Imm(1)), Bigz(1, 0), Halt()],

        "testloads" => vec![
            Ld(0, Imm(1000)),
            Sub(0, Reg(0), Imm(1)),
            Ld(1, Reg(0)),
            Ld(2, Reg(0)),
            Ld(3, Reg(0)),
            Ld(4, Reg(0)),
            Ld(5, Reg(0)),
            Ld(6, Reg(0)),
            Ld(7, Reg(0)),
            Bigz(1, 0),
            Halt(),
        ],

        _ => vec![],
    }
}
