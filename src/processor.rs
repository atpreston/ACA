pub type Register = u8;
pub type Immediate = i64;
pub type Label = String;

#[derive(Copy, Clone)]
pub enum Operand {Reg(Register), Imm(Immediate)}
impl Operand {
    /// returns the value stored by an operand (immediate, or register's value)
    fn extract(&self, registers: &[i64]) -> i64 {
        match self {
            Operand::Imm(val) => return *val,
            Operand::Reg(ind) => return registers[*ind as usize]
        }
    }
}

#[derive(Copy, Clone)]
pub enum Location {Label, Register, Immediate}

pub struct State {
    prog_counter: Register,
    instr_reg: Instr,
    registers: [i64; 8]
}

#[derive(Copy, Clone)]
pub enum Instr {
    // instructions are of the form output input (input)
    Add(Register, Operand, Operand),
    Sub(Register, Operand, Operand),
    Mul(Register, Operand, Operand),

    Not(Register, Operand),
    And(Register, Operand, Operand),
    Or(Register, Operand, Operand),
    Xor(Register, Operand, Operand),

    Cp(Register, Register),
    Mv(Register, Immediate),

    Ld(Register, Operand),
    Ldr(Register, Operand, Operand),
    St(Operand, Operand),

    B(Location),
    J(Location),
    Jile(Location, Register),
    Bile(Location, Register),

    Noop(),
    Halt()
}

pub type Program = Vec<Instr>;