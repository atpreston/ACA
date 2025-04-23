use core::fmt;
use either::*;

use crate::execution::ExecutionUnit;
use crate::reservation::*;

pub type Register = Either<i64, u8>;
pub type Immediate = i64;
// pub type Label = String;

pub const MEMSIZE: usize = 2048;
pub const REGISTERS: usize = 8;
pub const EXECUTIONUNITS: usize = 4;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operand {
    Reg(u8),
    Imm(Immediate),
}
impl Operand {
    /// returns the value stored by an operand (immediate, or register's value)
    pub fn extract(&self, registers: &[Register; REGISTERS]) -> Either<i64, u8> {
        match self {
            Operand::Imm(val) => return Left(*val),
            Operand::Reg(ind) => {
                let reg_val = registers[*ind as usize];
                return reg_val;
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct State {
    pub prog_counter: u8,
    pub instr_reg: Instr,
    pub registers: [i64; REGISTERS],
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
            .finish()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instr {
    // instructions are of the form output input (input)
    Add(Register, Operand, Operand),
    Sub(Register, Operand, Operand),
    Mul(Register, Operand, Operand),

    Not(Register, Operand),
    And(Register, Operand, Operand),
    Or(Register, Operand, Operand),
    Xor(Register, Operand, Operand),

    Cp(Register, Operand),

    Ld(Register, Operand),
    Ldr(Register, Operand, Operand),
    St(Operand, Operand),

    B(Operand),
    J(Operand),
    Jilz(Register, Operand),
    Bilz(Register, Operand),
    Bilt(Register, Operand, Operand),
    Jilt(Register, Operand, Operand),
    Jigz(Register, Operand),
    Bigz(Register, Operand),
    Bigt(Register, Operand, Operand),
    Jigt(Register, Operand, Operand),

    Noop(),
    Halt(),
}

impl Instr {
    pub fn execute(
        self,
        vj: i64,
        vk: i64,
        dest: u8,
        mut registers: &[Register; REGISTERS],
        memory: Box<[i64; 2048]>,
        mut prog_counter: u8,
    ) -> () {
        match self {
            Instr::Add(..) => registers[dest as usize] = Left(vj + vk),
            Instr::Sub(..) => registers[dest as usize] = Left(vj - vk),
            Instr::Mul(..) => registers[dest as usize] = Left(vj * vk),

            Instr::Not(..) => registers[dest as usize] = Left(!vj),
            Instr::And(..) => registers[dest as usize] = Left(vj & vk),
            Instr::Or(..) => registers[dest as usize] = Left(vj | vk),
            Instr::Xor(..) => registers[dest as usize] = Left(vj ^ vk),

            Instr::Cp(..) => registers[dest as usize] = Left(vj),
            Instr::Ld(..) => registers[dest as usize] = Left(vj),
            Instr::Ldr(..) => registers[dest as usize] = Left(memory[(vj + vk) as usize]),
            Instr::St(..) => memory[dest as usize] = vj,

            Instr::B(..) => prog_counter = vj as u8,
            Instr::Bilz(..) => {
                if registers[dest as usize] < 0 {
                    prog_counter = vj as u8
                }
            }
            Instr::Bilt(..) => {
                if registers[dest as usize] < vj {
                    prog_counter = vk as u8
                }
            }
            Instr::Bigz(..) => {
                if registers[dest as usize] > 0 {
                    prog_counter = vj as u8
                }
            }
            Instr::Bigt(..) => {
                if registers[dest as usize] > vj {
                    prog_counter = vk as u8
                }
            }
            Instr::J(..) => prog_counter += vj as u8,
            Instr::Jilz(..) => {
                if registers[dest as usize] < 0 {
                    prog_counter += vj as u8
                }
            }
            Instr::Jilt(..) => {
                if registers[dest as usize] < vj {
                    prog_counter += vk as u8
                }
            }
            Instr::Jigz(..) => {
                if registers[dest as usize] > 0 {
                    prog_counter += vj as u8
                }
            }
            Instr::Jigt(..) => {
                if registers[dest as usize] > vj {
                    prog_counter += vk as u8
                }
            }

            Instr::Noop() => (),
            Instr::Halt() => prog_counter = 0,
        }
        return;
    }

    pub fn get_operands(self, registers: &[Register; REGISTERS]) -> Vec<Either<i64, u8>> {
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
            Instr::Ldr(_, operand, operand1) => {
                vec![operand.extract(registers), operand1.extract(registers)]
            }
            Instr::St(_, operand) => vec![operand.extract(registers)],
            Instr::B(_) => vec![],
            Instr::J(_) => vec![],
            Instr::Jilz(_, operand) => vec![operand.extract(registers)],
            Instr::Bilz(_, operand) => vec![operand.extract(registers)],
            Instr::Bilt(_, operand, operand1) => {
                vec![operand.extract(registers), operand1.extract(registers)]
            }
            Instr::Jilt(_, operand, operand1) => {
                vec![operand.extract(registers), operand1.extract(registers)]
            }
            Instr::Jigz(_, operand) => vec![operand.extract(registers)],
            Instr::Bigz(_, operand) => vec![operand.extract(registers)],
            Instr::Bigt(_, operand, operand1) => {
                vec![operand.extract(registers), operand1.extract(registers)]
            }
            Instr::Jigt(_, operand, operand1) => {
                vec![operand.extract(registers), operand1.extract(registers)]
            }
            Instr::Noop() => vec![],
            Instr::Halt() => vec![],
        }
    }

    pub fn get_location(self) -> Option<Either<i64, u8>> {
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
            Instr::Ldr(loc, ..) => Some(loc),
            _ => None,
        }
    }
}

pub type Program = Vec<Instr>;
