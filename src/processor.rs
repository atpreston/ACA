use core::fmt;
use either::*;

pub type Register = Either<i64,u8>;
pub type Immediate = i64;
// pub type Label = String;

pub const MEMSIZE: usize = 2048;
pub const REGISTERS: usize = 8;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operand {Reg(u8), Imm(Immediate)}
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
    pub counter: u64
}
impl fmt::Debug for State { // to not print out all of memory on debug
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
    Halt()
}

impl Instr {
    pub fn execute(self, state: &State, vi: i64, vj: i64, dest: u8) -> State {
        let State{mut prog_counter, instr_reg, mut registers, mut memory, counter} = state.clone();
        match self {
            Instr::Add(..) => registers[dest as usize] = vi + vj,
            Instr::Sub(..) => registers[dest as usize] = vi - vj,
            Instr::Mul(..) => registers[dest as usize] = vi * vj,
    
            Instr::Not(..) => registers[dest as usize] = !vi,
            Instr::And(..) => registers[dest as usize] = vi & vj,
            Instr::Or(..) => registers[dest as usize] = vi | vj,
            Instr::Xor(..) => registers[dest as usize] = vi ^ vj,
    
            Instr::Cp(..) => registers[dest as usize] = vi,
            Instr::Ld(..) => registers[dest as usize] = vi,
            Instr::Ldr(..) => registers[dest as usize] = memory[(vi + vj) as usize],
            Instr::St(..) => memory[dest as usize] = vi,
            
            Instr::B(..) => prog_counter = vi as u8,
            Instr::Bilz(..) => {if registers[dest as usize] < 0 {prog_counter = vi as u8}},
            Instr::Bilt(..) => {if registers[dest as usize] < vi {prog_counter = vj as u8}}
            Instr::Bigz(..) => {if registers[dest as usize] > 0 {prog_counter = vi as u8}},
            Instr::Bigt(..) => {if registers[dest as usize] > vi {prog_counter = vj as u8}}
            Instr::J(..) => prog_counter += vi as u8,
            Instr::Jilz(..) => {if registers[dest as usize] < 0 {prog_counter += vi as u8}},
            Instr::Jilt(..) => {if registers[dest as usize] < vi {prog_counter += vj as u8}},
            Instr::Jigz(..) => {if registers[dest as usize] > 0 {prog_counter += vi as u8}},
            Instr::Jigt(..) => {if registers[dest as usize] > vi {prog_counter += vj as u8}},
    
            Instr::Noop() => (),
            Instr::Halt() => prog_counter = 0,
        }
        return State{prog_counter, instr_reg: instr_reg.clone(), registers, memory, counter};
    }
}

pub type Program = Vec<Instr>;