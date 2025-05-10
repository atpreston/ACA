mod execution;
// mod parser;
mod processor;
mod programs;
mod reservation;
mod user;

// use std::env;

// use decode::*;
use execution::ExecutionUnit;
use processor::*;

use programs::load_program;
use rand::prelude::*;
use user::UserMode;

fn instruction_cycle(state: &mut State, program: &Vec<Instr>, halted: bool) -> () {
    eprintln!("\n");
    // execute
    for (index, execution_unit) in state.execution_units.iter_mut().enumerate() {
        execution_unit.tick(&mut state.prog_counter, &mut state.cdb, index as u8);
    }

    // writeback
    for (eu_index, execution_unit) in state.execution_units.iter_mut().enumerate() {
        execution_unit.writeback(&mut state.cdb);
        if let Some(exec_location) = execution_unit.writeback_result.clone() {
            match exec_location {
                (ExecLocation::Reg(val, index), slot_index) => {
                    if state.registers[index as usize]
                        == RegisterVal::Tag((eu_index as u8, slot_index as u8))
                    {
                        // if the register hasn't been written to by another instruction since this one issued
                        state.registers[index as usize] = RegisterVal::Val(val)
                    }
                }
                (ExecLocation::Mem(val, index), _) => state.memory[index as usize] = val,
            }
            execution_unit.writeback_result = None;
        }
    }

    for execution_unit in state.execution_units.iter_mut() {
        execution_unit.reservation_station.tick(&mut state.cdb); // updates slots to take values from cdb
    }

    // Fetch
    if !halted {
        if let Some(instr) = program.get(state.prog_counter as usize) {
            state.instr_reg = instr.clone();
        }
    }

    // decode/issue
    if !halted {
        match state.branch_index {
            Some((exec_index, slot_index)) => {
                // if a branch instruction was executing
                if !state.execution_units[exec_index].is_slot_busy(slot_index) {
                    // check if it still is
                    state.branch_index = None; // if it isn't then update the index
                }
            }
            None => {
                // rewriting to issue to the execution unit with the lowest number of available slots
                let mut best_eu_index: Option<usize> = None;
                let mut mostfreeslots = 0;
                for (eu_index, execution_unit) in state.execution_units.iter().enumerate() {
                    let free_slots = execution_unit
                        .reservation_station
                        .slots
                        .iter()
                        .filter(|x: &&reservation::ReservationSlot| -> bool { !x.busy })
                        .count();
                    if free_slots > mostfreeslots {
                        mostfreeslots = free_slots;
                        best_eu_index = Some(eu_index);
                    }
                }

                match best_eu_index {
                    Some(index) => {
                        let execution_unit = &mut state.execution_units[index];
                        let exec_index =
                            best_eu_index.expect("EU chosen without index given") as u8;
                        match execution_unit.issue(
                            state.instr_reg.clone(),
                            &mut state.registers,
                            exec_index,
                            &mut state.cdb,
                        ) {
                            // try to issue to the unit
                            Some(slot_index) => {
                                // if we did issue to a slot...
                                match state.instr_reg {
                                    // update branch index if branch instruction sent
                                    Instr::Biez(..) => {
                                        state.branch_index =
                                            Some((exec_index as usize, slot_index));
                                    }
                                    Instr::Bigz(..) => {
                                        state.branch_index =
                                            Some((exec_index as usize, slot_index));
                                    }
                                    Instr::Bilz(..) => {
                                        state.branch_index =
                                            Some((exec_index as usize, slot_index));
                                    }
                                    Instr::J(..) => {
                                        state.branch_index =
                                            Some((exec_index as usize, slot_index));
                                    }
                                    _ => (),
                                }
                                state.prog_counter = state.prog_counter.saturating_add(1);
                                // saturating add because Halt could have executed - Halt just sets PC to u8::MAX
                            }
                            None => (), // if we didn't issue the slot must have been busy, so let the for loop try the next one
                        };
                    }
                    None => eprintln!("NO EU WITH SLOTS AVAILABLE"),
                }
            }
        }
    }
    eprintln!("{:?}", state);

    return;
}

// fn load_program(path: &str) -> Vec<Instr> {
//     return parser::parse(path).expect("Cannot parse program");
// }

fn wait(user_mode: &mut UserMode) {
    match user_mode.wait() {
        false => (),
        true => {
            // eprintln!("Keyboard Interrupt");
            std::process::exit(0)
        }
    };
}

fn main() {
    let mut user_state = UserMode::new();

    // let args: Vec<String> = env::args().collect();
    // let path = args.get(2).map(String::as_str).unwrap_or("./program.txt");
    // let program: Vec<Instr> = load_program(path);
    let program: Vec<Instr> = load_program(PROGNAME);
    println!("{:?}\n\n", program);

    let prog_counter: i64 = 0;
    let instr_reg: Instr = program[prog_counter as usize].clone();
    let mut state: State = State {
        prog_counter: 0,
        instr_reg: instr_reg,
        registers: [RegisterVal::Val(0); REGISTERS],
        memory: Box::new([0; MEMSIZE]),
        counter: 0,
        cdb: vec![],
        execution_units: Box::new(std::array::from_fn(|_| ExecutionUnit::new())),
        branch_index: None,
    };

    let mut rng = rand::rng();
    for i in 1..state.memory.len() {
        state.memory[i] = rng.random::<i64>();
    }
    while state.prog_counter < u8::MAX {
        instruction_cycle(&mut state, &program, false);
        state.counter += 1;
        // println!("{:?}\n", state.instr_reg);

        // implements user being able to step through/run the program
        wait(&mut user_state);
    }

    eprintln!("HALTED");

    // once halted, wait until all execution units have finished
    while !state
        .execution_units
        .iter()
        .all(|x: &ExecutionUnit| -> bool { x.all_free() })
    {
        instruction_cycle(&mut state, &program, true);
        // println!("{:?}\n", state.instr_reg);
        wait(&mut user_state);
    }
    println!("{:?}", state);
    println!("PROGRAM COMPLETE in {} cycles", state.counter);
}
