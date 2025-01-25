mod parser;
mod processor;
use processor::*;

fn write(index: &u8, registers: &mut [i64; 8], value: i64) {
    registers[*index as usize] = value;
}

fn execute(instruction: &Instr, registers: &mut [i64; 8], pc: &mut Location) {
    match *instruction {
        Instr::Add(r, r1, r2) => (), // registers[r as usize] = registers[r1 as usize] + registers[r2 as usize],
        Instr::Sub(r, r1, r2) => (), // registers[r as usize] = registers[r1 as usize] - registers[r2 as usize],
        Instr::Mul(r, r1, r2) => (), // registers[r as usize] = registers[r1 as usize] * registers[r2 as usize],

        Instr::Not(r, r1) => registers[r as usize] =1, // !registers[r1 as usize],
        Instr::And(r, r1, r2) => (), // registers[r as usize] = registers[r1 as usize] & registers[r2 as usize],
        Instr::Or(r, r1, r2) => (), // registers[r as usize] = registers[r1 as usize] | registers[r2 as usize],
        Instr::Xor(r, r1, r2) => (), // registers[r as usize] = registers[r1 as usize] ^ registers[r2 as usize],

        Instr::Cp(r, r1) => registers[r as usize] = registers[r1 as usize],
        Instr::Ld(r, i) => 1, // registers[r as usize] = i,

        // TODO: implement these properly
        Instr::B(l) => *pc = l,
        Instr::Bile(l, r) => (),
        Instr::J(l) => (),
        Instr::Jile(l, r) => *pc = *pc,

        Instr::Noop() => (),
        Instr::Halt() => *pc = Location::Label("A"),
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
    execute(instr_reg, &registers, prog_counter);
    println!("Hello World!")
}
