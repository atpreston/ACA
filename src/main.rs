mod parser;
mod processor;
mod decode;
mod execution_unit;

use std::env;

use processor::*;
use decode::*;

use rand::prelude::*;

fn memory_stage(state: State) -> State {
    state 
    // TODO: This does nothing yet
}

fn instruction_cycle(init_state: State, program : &Vec<Instr>) -> State {

    let mut ret_state = init_state.clone();
    // Fetch
    ret_state.instr_reg = program[ret_state.prog_counter as usize].clone();
    ret_state.prog_counter += 1;

    // Decode/dispath
    ret_state = decode(ret_state);

    // Execute (done in execution units)
    
    // Memory
    ret_state = memory_stage(ret_state);
    // Writeback
    println!("{:?}, {:?}", init_state.clone(), ret_state);
    return ret_state;
}

fn load_program(path: &str) -> Vec<Instr> {
    return parser::parse(path).expect("Cannot parse program");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(2).map(String::as_str).unwrap_or("./program.txt");
    let program: Vec<Instr> = load_program(path);
    println!("{:?}", program);

    let prog_counter : i64 = 0;
    let mut instr_reg : Instr = program[prog_counter as usize].clone();
    let mut state : State = State{prog_counter: 0, instr_reg: instr_reg.clone(), registers: [0; REGISTERS], memory: Box::new([0; MEMSIZE]), counter: 0};

    let mut rng = rand::rng();
    for i in 1..state.memory.len() {
        state.memory[i] = rng.random::<i64>();
    }
    while instr_reg != (Instr::Halt()) {
        state = instruction_cycle(state, &program);
        instr_reg = state.instr_reg;
        println!("{:?}", state);
    }
}
