type Register = u8;
type Immediate = i64;
type Label = String;
enum Operand {Reg(Register), Imm(Immediate)}
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
enum Location {Label, Register, Immediate}

struct State {
    prog_counter: Register,
    instr_reg: Instr,
    registers: [i64; 8]
}

#[derive(Copy, Clone)]
enum Instr {
    // instructions are of the form output input (input)
    Add(Register, Register, Register),
    Addi(Register, Register, Immediate),
    Sub(Register, Register, Register),
    Subi(Register, Register, Immediate),
    Mul(Register, Register, Register),
    Muli(Register, Register, Immediate),

    Not(Register, Register),
    And(Register, Register, Register),
    Andi(Register, Register, Immediate),
    Or(Register, Register, Register),
    Ori(Register, Register, Immediate),
    Xor(Register, Register, Register),
    Xori(Register, Register, Immediate),

    Cp(Register, Register),
    Ld(Register, Immediate),
    Ldo(Register, Register, Immediate),

    B(Location),
    J(Location),
    Jile(Location),
    Bile(Location),

    Noop(),
    Halt()
}

fn write(index: &u8, registers: &mut [i64; 8], value: i64) {
    registers[*index as usize] = value;
}

fn execute(instruction: &Instr, registers: &mut [i64; 8], pc: &mut Location) {
    match *instruction {
        Instr::Add(r, r1, r2) => registers[r as usize] = registers[r1 as usize] + registers[r2 as usize],
        Instr::Addi(r, r1, i1) => registers[r as usize] = registers[r1 as usize] + i1,
        Instr::Sub(r, r1, r2) => registers[r as usize] = registers[r1 as usize] - registers[r2 as usize],
        Instr::Subi(r, r1, i1) => registers[r as usize] = registers[r1 as usize] - i1,
        Instr::Mul(r, r1, r2) => registers[r as usize] = registers[r1 as usize] * registers[r2 as usize],
        Instr::Muli(r, r1, i1) => registers[r as usize] = registers[r1 as usize] * i1,

        Instr::Not(r, r1) => registers[r as usize] = !registers[r1 as usize],
        Instr::And(r, r1, r2) => registers[r as usize] = registers[r1 as usize] & registers[r2 as usize],
        Instr::Andi(r, r1, i1) => registers[r as usize] = registers[r1 as usize] & i1,
        Instr::Or(r, r1, r2) => registers[r as usize] = registers[r1 as usize] | registers[r2 as usize],
        Instr::Ori(r, r1, i1) => registers[r as usize] = registers[r1 as usize] | i1,
        Instr::Xor(r, r1, r2) => registers[r as usize] = registers[r1 as usize] ^ registers[r2 as usize],
        Instr::Xori(r, r1, i1) => registers[r as usize] = registers[r1 as usize] ^ i1,

        Instr::Cp(r, r1) => registers[r as usize] = registers[r1 as usize],
        Instr::Ld(r, i) => registers[r as usize] = i,
        Instr::Ldo(r, r1, i1) => {let index: i64 = registers[r1 as usize] + i1; registers[r as usize] = registers[index as usize]},

        // TODO: implement these properly
        Instr::B(l) => *pc = l
        Instr::Ble(l) => w
        Instr::J(l) => *pc = *pc + l,
        Instr::Jle(l) => *pc = *pc
        }
}

fn instruction_cycle(init_state: State, program : &[Instr]) {
    let State {mut prog_counter, mut instr_reg, mut registers} = init_state;

    // Fetch
    instr_reg = program[prog_counter as usize];

    // Decode (not needed)

    // Execute
    execute(&instr_reg, &mut registers, &mut prog_counter);
    
}

fn load_program() -> Vec<Instr> {
    return Vec::<Instr>::new(); //TODO: load in program from file
}

fn main() {
    let mut registers: [i64; 8] = [0; 8];

    let mut prog_counter : Register = 0;
    let mut instr_reg : Instr;
    let mut state : State = State {prog_counter, instr_reg, registers};

    let program: Vec<Instr> = load_program();

    instruction_cycle(state, program);
    execute(instr_reg, &registers, pc);
    println!("Hello World!")
}
