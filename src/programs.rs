use crate::processor::Instr::*;
use crate::processor::*;
use crate::Operand::{Imm, Reg};

const COLLATZ: i64 = 4;

pub fn load_program(input: &str) -> Vec<Instr> {
    match input {
        "multest" => vec![
            Cp(1, Imm(200)),
            Mul(2, Reg(0), Reg(1)),
            Cp(3, Reg(2)),
            Halt(),
        ],

        "branchtest" => vec![Cp(0, Imm(1000)), Sub(0, Reg(0), Imm(1)), Bigz(1, 0), Halt()],

        "testloads" => vec![
            Cp(0, Imm(1000)),
            Sub(0, Reg(0), Imm(1)),
            Cp(1, Reg(0)),
            Cp(2, Reg(0)),
            Cp(3, Reg(0)),
            Cp(4, Reg(0)),
            Cp(5, Reg(0)),
            Cp(6, Reg(0)),
            Cp(7, Reg(0)),
            Bigz(1, 0),
            Halt(),
        ],

        "collatz" => vec![
            Cp(0, Imm(COLLATZ)),
            Cp(4, Imm(0)),
            Add(4, Reg(4), Imm(1)),
            Sub(1, Reg(0), Imm(1)),
            Biez(18, 1),
            And(1, Reg(0), Imm(1)),
            Biez(10, 1),
            // Odd
            Mul(0, Reg(0), Imm(3)),
            Add(0, Reg(0), Imm(1)),
            J(2),
            // Even
            Cp(2, Imm(0)),
            Add(2, Reg(2), Imm(1)), //STARTDIV
            Mul(3, Reg(2), Imm(2)),
            Sub(3, Reg(0), Reg(3)),
            Biez(16, 3),
            J(11),
            Cp(0, Reg(2)), //ENDDIV
            J(2),
            Sub(4, Reg(4), Imm(1)),
            Halt(),
        ],

        _ => vec![],
    }
}
