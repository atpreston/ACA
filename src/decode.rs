use crate::processor::*;
use crate::execution_unit;

pub fn decode(state: State) -> State {
    let State{mut prog_counter, instr_reg, mut registers, mut memory, mut counter} = state;
    match instr_reg {
        Instr::Add(r, o1, o2) => registers[r as usize] = o1.extract(&registers) + o2.extract(&registers),
        Instr::Sub(r, o1, o2) => registers[r as usize] = o1.extract(&registers) - o2.extract(&registers),
        Instr::Mul(r, o1, o2) => registers[r as usize] = o1.extract(&registers) * o2.extract(&registers),

        Instr::Not(r, o1) => registers[r as usize] = !o1.extract(&registers),
        Instr::And(r, o1, o2) => registers[r as usize] = o1.extract(&registers) & o2.extract(&registers),
        Instr::Or(r, o1, o2) => registers[r as usize] = o1.extract(&registers) | o2.extract(&registers),
        Instr::Xor(r, o1, o2) => registers[r as usize] = o1.extract(&registers) ^ o2.extract(&registers),

        Instr::Cp(r1, o1) => registers[r1 as usize] = o1.extract(&registers),
        Instr::Ld(r1, o1) => registers[r1 as usize] = o1.extract(&registers),
        Instr::Ldr(r1, o1, o2) => registers[r1 as usize] = memory[(o1.extract(&registers) + o2.extract(&registers)) as usize],
        Instr::St(o1,o2) => memory[o1.extract(&registers) as usize] = o2.extract(&registers),
        
        Instr::B(o1) => prog_counter = o1.extract(&registers) as u8,
        Instr::Bilz(r1, o1) => {if registers[r1 as usize] < 0 {prog_counter = o1.extract(&registers) as u8}},
        Instr::Bilt(r1,o1, o2 ) => {if registers[r1 as usize] < o1.extract(&registers) {prog_counter = o2.extract(&registers) as u8}}
        Instr::Bigz(r1, o1) => {if registers[r1 as usize] > 0 {prog_counter = o1.extract(&registers) as u8}},
        Instr::Bigt(r1, o1, o2 ) => {if registers[r1 as usize] > o1.extract(&registers) {prog_counter = o2.extract(&registers) as u8}}
        Instr::J(o1) => prog_counter += o1.extract(&registers) as u8,
        Instr::Jilz(r1, o1) => {if registers[r1 as usize] < 0 {prog_counter += o1.extract(&registers) as u8}},
        Instr::Jilt(r1, o1, o2 ) => {if registers[r1 as usize] < o1.extract(&registers) {prog_counter += o2.extract(&registers) as u8}},
        Instr::Jigz(r1, o1) => {if registers[r1 as usize] > 0 {prog_counter += o1.extract(&registers) as u8}},
        Instr::Jigt(r1, o1, o2 ) => {if registers[r1 as usize] > o1.extract(&registers) {prog_counter += o2.extract(&registers) as u8}},

        Instr::Noop() => (),
        Instr::Halt() => prog_counter = 0,
    }
    counter += 1; // TODO: THIS IS LAZY AND WRONG
    return State{prog_counter, instr_reg: instr_reg.clone(), registers, memory, counter};
}
