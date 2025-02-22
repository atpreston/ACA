mod parser;
mod processor;

use processor::*;

fn execute(state: State) -> State {
    let State{mut prog_counter, instr_reg, mut registers, mut memory, mut counter} = state;
    match instr_reg {
        Instr::Add(r, o1, o2) => registers[r as usize] = registers[o1.extract(&registers) as usize] + registers[o2.extract(&registers) as usize],
        Instr::Sub(r, o1, o2) => registers[r as usize] = registers[o1.extract(&registers) as usize] - registers[o2.extract(&registers) as usize],
        Instr::Mul(r, o1, o2) => registers[r as usize] = registers[o1.extract(&registers) as usize] * registers[o2.extract(&registers) as usize],

        Instr::Not(r, o1) => registers[r as usize] = !o1.extract(&registers),
        Instr::And(r, o1, o2) => registers[r as usize] = o1.extract(&registers) & o2.extract(&registers),
        Instr::Or(r, o1, o2) => registers[r as usize] = o1.extract(&registers) | o2.extract(&registers),
        Instr::Xor(r, o1, o2) => registers[r as usize] = o1.extract(&registers) ^ o2.extract(&registers),

        Instr::Cp(r1, o1) => registers[r1 as usize] = registers[o1.extract(&registers) as usize],
        Instr::Ld(r1, o1) => registers[r1 as usize] = memory[o1.extract(&registers) as usize],
        Instr::Ldr(r1, o1, o2) => registers[r1 as usize] = memory[(o1.extract(&registers) + o2.extract(&registers)) as usize],
        Instr::St(o1,o2) => memory[o1.extract(&registers) as usize] = o2.extract(&registers),
        
        Instr::B(o1) => prog_counter = o1.extract(&registers) as u8,
        Instr::Bilz(r1, o1) => {if registers[r1 as usize] < 0 {prog_counter = o1.extract(&registers) as u8}},
        Instr::Bilt(r1,o1 ,o2 ) => {if registers[r1 as usize] < o1.extract(&registers) {prog_counter = o2.extract(&registers) as u8}}
        Instr::J(o1) => prog_counter += o1.extract(&registers) as u8,
        Instr::Jilz(r1, o1) => {if registers[r1 as usize] < 0 {prog_counter += o1.extract(&registers) as u8}},
        Instr::Jilt(r1,o1 ,o2 ) => {if registers[r1 as usize] < o1.extract(&registers) {prog_counter += o2.extract(&registers) as u8}},
        
        Instr::Noop() => (),
        Instr::Halt() => prog_counter = 0,
    }
    counter += 1; // TODO: THIS IS LAZY AND WRONG
    return State{prog_counter, instr_reg: instr_reg.clone(), registers, memory, counter};
}

fn memory_stage(state: State) -> State {
    state 
    // TODO: This does nothing yet
}

fn instruction_cycle(init_state: State, program : &Vec<Instr>) -> State {

    let State {mut prog_counter, mut registers, mut memory, mut counter, ..} = init_state;
    // Fetch
    let mut instr_reg = program[prog_counter as usize].clone();
    prog_counter += 1;

    // Decode (not needed)
    
    // Execute
    State{prog_counter, instr_reg, registers, memory, counter} = execute(State{prog_counter, instr_reg, registers, memory,counter});
    
    // Memory
    State{memory, ..} = memory_stage(State{prog_counter, instr_reg, registers, memory, counter});
    // Writeback

    return State{prog_counter, instr_reg, registers, memory, counter};
}

fn load_program(path: &str) -> Vec<Instr> {
    return parser::parse(path).expect("Cannot parse program");
}

fn main() {
    let program: Vec<Instr> = load_program("./program.txt");
    println!("{:?}", program);

    let prog_counter : i64 = 0;
    let mut instr_reg : Instr = program[prog_counter as usize].clone();
    let mut state : State = State{prog_counter: 0, instr_reg: instr_reg.clone(), registers: [0; 8], memory: Box::new([0; MEMSIZE]), counter: 0};

    while instr_reg != (Instr::Halt()) {
        state = instruction_cycle(state, &program);
        instr_reg = state.instr_reg;
        println!("{:?}", state);
    }
}
