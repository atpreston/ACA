use core::fmt;

pub type Register = u8;
pub type Immediate = i64;
// pub type Label = String;

pub const MEMSIZE: usize = 2048;
pub const REGISTERS: usize = 8;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operand {Reg(Register), Imm(Immediate)}
impl Operand {
    /// returns the value stored by an operand (immediate, or register's value)
    pub fn extract(&self, registers: &[i64]) -> i64 {
        match self {
            Operand::Imm(val) => return *val,
            Operand::Reg(ind) => return registers[*ind as usize]
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct State {
    pub prog_counter: Register,
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

pub type Program = Vec<Instr>;