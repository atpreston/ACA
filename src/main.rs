mod decode;
mod execution;
// mod parser;
mod processor;
mod registers;
mod reservation;
mod user;

// use std::env;

// use decode::*;
use execution::ExecutionUnit;
use processor::*;

use rand::prelude::*;
use user::UserState;

fn memory_stage(state: State) -> State {
    state
    // TODO: This does nothing yet
}

fn instruction_cycle(state: &mut State, program: &Vec<Instr>, user_state: &mut UserState) -> () {
    //let init_state: State = state.clone();
    // Fetch
    state.instr_reg = program[state.prog_counter as usize].clone();

    // Decode/dispatch
    // state = decode(state);
    for execution_unit in state.execution_units.iter_mut() {
        let successful_issue = execution_unit.issue(state.instr_reg.clone(), &state.registers);
        if successful_issue {
            state.prog_counter += 1;
            break;
        }
    }

    // Execute (done in execution units)
    for execution_unit in state.execution_units.iter_mut() {
        execution_unit.tick(
            &mut state.registers,
            &mut state.memory,
            &mut state.prog_counter,
        );
    }

    // Memory
    // state = memory_stage(state); DOES NOTHING YET

    // Writeback
    //println!("{:?}, {:?}", init_state.clone(), *state);

    user_state.wait(); // implements stepping through/running the program
    return;
}

// fn load_program(path: &str) -> Vec<Instr> {
//     return parser::parse(path).expect("Cannot parse program");
// }

fn main() {
    let mut user_state = UserState { stepping: true };

    // let args: Vec<String> = env::args().collect();
    // let path = args.get(2).map(String::as_str).unwrap_or("./program.txt");
    // let program: Vec<Instr> = load_program(path);
    let program = vec![
        Instr::Ld(0, Operand::Imm(2)),
        Instr::Ld(1, Operand::Imm(5)),
        Instr::Mul(0, Operand::Reg(0), Operand::Reg(1)),
        Instr::Halt(),
    ];
    println!("{:?}\n\n", program);

    let prog_counter: i64 = 0;
    let mut instr_reg: Instr = program[prog_counter as usize].clone();
    let mut state: State = State {
        prog_counter: 0,
        instr_reg: instr_reg.clone(),
        registers: [either::Either::Left(0); REGISTERS],
        memory: Box::new([0; MEMSIZE]),
        counter: 0,
        cdb: vec![],
        execution_units: Box::new(std::array::from_fn(|_| ExecutionUnit::new())),
    };

    let mut rng = rand::rng();
    for i in 1..state.memory.len() {
        state.memory[i] = rng.random::<i64>();
    }
    while instr_reg != (Instr::Halt()) {
        instruction_cycle(&mut state, &program, &mut user_state);
        instr_reg = state.instr_reg.clone();
        println!("{:?}", instr_reg);
    }
    println!("{:?}", state);
}
