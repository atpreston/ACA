pub type Register = u8;
pub type Immediate = i64;
// pub type Label = String;

pub const MEMSIZE: usize = 2048;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub prog_counter: Register,
    pub instr_reg: Instr,
    pub registers: [i64; 8],
    pub memory: Box<[i64; MEMSIZE]>,
    pub counter: u64
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

    Noop(),
    Halt()
}

pub type Program = Vec<Instr>;