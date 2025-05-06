use core::fmt;
use either::*;

use crate::execution::ExecutionUnit;
use crate::reservation::*;

pub type RegisterVal = Either<i64, u8>;
pub type RegisterIndex = u8;
pub type Immediate = i64;
pub type MemoryIndex = u8;
// pub type Label = String;

pub const MEMSIZE: usize = 2048;
pub const REGISTERS: usize = 8;
pub const EXECUTIONUNITS: usize = 1;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operand {
    Reg(u8),
    Imm(Immediate),
}
impl Operand {
    /// returns the value stored by an operand (immediate, or register's value)
    pub fn extract(&self, registers: &[RegisterVal; REGISTERS]) -> Either<i64, u8> {
        match self {
            Operand::Imm(val) => return Left(*val),
            Operand::Reg(ind) => {
                let reg_val = registers[*ind as usize];
                return reg_val;
            }
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct State {
    pub prog_counter: u8,
    pub instr_reg: Instr,
    pub registers: [RegisterVal; REGISTERS],
    pub memory: Box<[i64; MEMSIZE]>,
    pub counter: u64,

    pub execution_units: Box<[ExecutionUnit; EXECUTIONUNITS]>,
    pub cdb: CDB,
}
impl fmt::Debug for State {
    // to not print out all of memory on debug
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.counter)
            .field(&self.instr_reg)
            .field(&self.prog_counter)
            .field(&self.registers)
            .field(&self.execution_units)
            .finish()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Instr {
    // instructions are of the form output input (input)
    Add(RegisterIndex, Operand, Operand),
    Sub(RegisterIndex, Operand, Operand),
    Mul(RegisterIndex, Operand, Operand),

    Not(RegisterIndex, Operand),
    And(RegisterIndex, Operand, Operand),
    Or(RegisterIndex, Operand, Operand),
    Xor(RegisterIndex, Operand, Operand),

    Cp(RegisterIndex, Operand),

    Ld(MemoryIndex, Operand),
    St(Operand, Operand),

    Bigz(Immediate, RegisterIndex),
    Bilz(Immediate, RegisterIndex),
    Biez(Immediate, RegisterIndex),
    J(Immediate),

    Noop(),
    Halt(),
}

fn resolve_dest(dest: Option<u8>) -> Result<RegisterIndex, String> {
    match dest {
        Some(u) => Ok(u),
        _ => Err(
            "Attempt to resolve a destination from a instruction that does not write to one"
                .to_string(),
        ),
    }
}

impl Instr {
    pub fn execute(
        self,
        vj: i64,
        vk: i64,
        dest: Option<u8>,
        registers: &mut [RegisterVal; REGISTERS],
        memory: &mut [i64; 2048],
        prog_counter: &mut u8,
    ) -> () {
        match self {
            Instr::Add(..) => registers[resolve_dest(dest).unwrap() as usize] = Left(vj + vk),
            Instr::Sub(..) => registers[resolve_dest(dest).unwrap() as usize] = Left(vj - vk),
            Instr::Mul(..) => registers[resolve_dest(dest).unwrap() as usize] = Left(vj * vk),

            Instr::Not(..) => registers[resolve_dest(dest).unwrap() as usize] = Left(!vj),
            Instr::And(..) => registers[resolve_dest(dest).unwrap() as usize] = Left(vj & vk),
            Instr::Or(..) => registers[resolve_dest(dest).unwrap() as usize] = Left(vj | vk),
            Instr::Xor(..) => registers[resolve_dest(dest).unwrap() as usize] = Left(vj ^ vk),

            Instr::Cp(..) => registers[resolve_dest(dest).unwrap() as usize] = Left(vj),
            Instr::Ld(..) => registers[resolve_dest(dest).unwrap() as usize] = Left(vj),
            Instr::St(..) => memory[resolve_dest(dest).unwrap() as usize] = vj,

            Instr::Bilz(..) => {
                if vj < 0 {
                    *prog_counter = vk as u8
                }
            }
            Instr::Bigz(..) => {
                if vk > 0 {
                    *prog_counter = vj as u8
                }
            }
            Instr::Biez(..) => {}
            Instr::J(..) => *prog_counter += vj as u8,
            Instr::Noop() => (),
            Instr::Halt() => *prog_counter = 0,
        }
        return;
    }

    pub fn get_operands(self, registers: &[RegisterVal; REGISTERS]) -> Vec<Either<i64, u8>> {
        match self {
            Instr::Add(_, operand, operand1) => {
                vec![operand.extract(registers), operand1.extract(registers)]
            }
            Instr::Sub(_, operand, operand1) => {
                vec![operand.extract(registers), operand1.extract(registers)]
            }
            Instr::Mul(_, operand, operand1) => {
                vec![operand.extract(registers), operand1.extract(registers)]
            }
            Instr::Not(_, operand) => vec![operand.extract(registers)],
            Instr::And(_, operand, operand1) => {
                vec![operand.extract(registers), operand1.extract(registers)]
            }
            Instr::Or(_, operand, operand1) => {
                vec![operand.extract(registers), operand1.extract(registers)]
            }
            Instr::Xor(_, operand, operand1) => {
                vec![operand.extract(registers), operand1.extract(registers)]
            }
            Instr::Cp(_, operand) => vec![operand.extract(registers)],
            Instr::Ld(_, operand) => vec![operand.extract(registers)],
            Instr::St(_, operand) => vec![operand.extract(registers)],
            Instr::J(_) => vec![],
            Instr::Bilz(_, operand) => vec![Right(operand)],
            Instr::Bigz(_, operand) => vec![Right(operand)],
            Instr::Biez(_, operand) => vec![Right(operand)],
            Instr::Noop() => vec![],
            Instr::Halt() => vec![],
        }
    }

    pub fn get_location(self) -> Option<u8> {
        match self {
            Instr::Add(loc, ..) => Some(loc),
            Instr::Sub(loc, ..) => Some(loc),
            Instr::Mul(loc, ..) => Some(loc),
            Instr::Not(loc, ..) => Some(loc),
            Instr::And(loc, ..) => Some(loc),
            Instr::Or(loc, ..) => Some(loc),
            Instr::Xor(loc, ..) => Some(loc),
            Instr::Cp(loc, ..) => Some(loc),
            Instr::Ld(loc, ..) => Some(loc),
            _ => None,
        }
    }
}

pub type Program = Vec<Instr>;
