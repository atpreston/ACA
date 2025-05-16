use core::fmt;
use either::*;

use crate::execution::ExecutionUnit;
use crate::reservation::*;

// VALUES TO CHANGE
pub const PROGNAME: &str = "collatz";
pub const MEMSIZE: usize = 2048;
pub const REGISTERS: usize = 8;
pub const EXECUTIONUNITS: usize = 3;
pub const SLOT_NUM: usize = 1;
// VALUES TO CHANGE

pub type RegisterIndex = u8;
pub type Immediate = i64;
pub type MemoryIndex = u8;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operand {
    Reg(u8),
    Imm(Immediate),
}

impl Operand {
    /// returns the value stored by an operand (immediate, or register's value)
    pub fn extract(&self, registers: &[RegisterVal; REGISTERS]) -> RegisterVal {
        match self {
            Operand::Imm(val) => return RegisterVal::Val(*val),
            Operand::Reg(ind) => {
                let reg_val = registers[*ind as usize];
                return reg_val;
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RegisterVal {
    Val(i64),
    Tag((u8, u8)), // EU, slot
}
impl RegisterVal {
    pub fn to_either(self) -> Either<i64, (u8, u8)> {
        match self {
            RegisterVal::Val(val) => Either::Left(val),
            RegisterVal::Tag(tag) => Either::Right(tag),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ExecLocation {
    Reg(i64, u8),
    Mem(i64, u8),
}
impl ExecLocation {
    pub fn val(&self) -> i64 {
        match self {
            ExecLocation::Reg(val, _) => *val,
            ExecLocation::Mem(val, _) => *val,
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
    pub branch_index: Option<(usize, usize)>,
}
impl fmt::Debug for State {
    // to not print out all of memory on debug
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&format_args!("COUNTER: {}", &self.counter))
            .field(&format_args!(
                "Instruction register: {:?}, Program counter: {}",
                &self.instr_reg, &self.prog_counter
            ))
            .field(&format_args!("Registers: {:?}", &self.registers))
            .field(&format_args!("EUs: {:#?}", &self.execution_units))
            .field(&format_args!("CDB: {:?}", &self.cdb))
            .field(&format_args!("Branch index: {:?}", &self.branch_index))
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

// fn resolve_dest(dest: Option<u8>) -> Result<RegisterIndex, String> {
//     match dest {
//         Some(u) => Ok(u),
//         _ => Err(
//             "Attempt to resolve a destination from a instruction that does not write to one"
//                 .to_string(),
//         ),
//     }
// }

impl Instr {
    pub fn execute(self, vj: i64, vk: i64, prog_counter: &mut u8) -> Option<ExecLocation> {
        match self {
            Instr::Add(..) => Some(ExecLocation::Reg(vj.wrapping_add(vk), 0)),
            Instr::Sub(..) => Some(ExecLocation::Reg(vj.wrapping_sub(vk), 0)),
            Instr::Mul(..) => Some(ExecLocation::Reg(vj.wrapping_mul(vk), 0)),

            Instr::Not(..) => Some(ExecLocation::Reg(!vj, 0)),
            Instr::And(..) => Some(ExecLocation::Reg(vj & vk, 0)),
            Instr::Or(..) => Some(ExecLocation::Reg(vj | vk, 0)),
            Instr::Xor(..) => Some(ExecLocation::Reg(vj ^ vk, 0)),

            Instr::Cp(..) => Some(ExecLocation::Reg(vj, 0)),
            Instr::Ld(..) => Some(ExecLocation::Reg(vj, 0)),
            Instr::St(..) => Some(ExecLocation::Mem(vj, 0)),

            Instr::Bilz(..) => {
                if vk < 0 {
                    *prog_counter = vj as u8;
                }
                None
            }
            Instr::Bigz(..) => {
                if vk > 0 {
                    *prog_counter = vj as u8;
                }
                None
            }
            Instr::Biez(..) => {
                if vk == 0 {
                    *prog_counter = vj as u8;
                }
                None
            }
            Instr::J(..) => {
                eprintln!("JUMPING PC TO {}", vj);
                *prog_counter = vj as u8;
                None
            }
            Instr::Noop() => None,
            Instr::Halt() => {
                *prog_counter = u8::MAX;
                None
            }
        }
    }

    pub fn get_operands(self, registers: &[RegisterVal; REGISTERS]) -> Vec<RegisterVal> {
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
            Instr::J(imm) => vec![RegisterVal::Val(imm)],
            Instr::Bilz(imm, index) => vec![RegisterVal::Val(imm), registers[index as usize]],
            Instr::Bigz(imm, index) => vec![RegisterVal::Val(imm), registers[index as usize]],
            Instr::Biez(imm, index) => vec![RegisterVal::Val(imm), registers[index as usize]],
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
