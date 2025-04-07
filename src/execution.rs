use crate::reservation::*;
use crate::processor::{self, *};

const fn get_cycles(inst: Instr) -> usize {
    match inst {
        processor::Instr::Add(_, _, _) => 1,
        processor::Instr::And(_, _, _) => 1,
        processor::Instr::B(_) => 2,
        processor::Instr::Cp(_, _) => 5,
        processor::Instr::Halt() => 1,
        processor::Instr::J(_) => 2,
        processor::Instr::Ld(_, _) => 5,
        processor::Instr::Ldr(_, _, _) => 7,
        processor::Instr::Mul(_, _, _) => 10,
        processor::Instr::Noop() => 1,
        processor::Instr::Not(_, _) => 1,
        processor::Instr::Or(_, _, _) => 1,
        processor::Instr::St(_, _) => 5,
        processor::Instr::Sub(_, _, _) => 3,
        processor::Instr::Xor(_, _, _) => 1,
        _ => 5
    }
}

pub struct ExecutionUnit {
    reservation_stations: [ReservationStation; RES_NUM],
    tick: usize,
    current_inst_index: usize,
}

fn rename(r1: Register, o1: Operand, o2: Operand) {

}

impl ExecutionUnit {
    pub fn send(&mut self, instruction: Instr, registers:&[i64]) -> bool {
        let mut free_slot: Option<usize> = None;
        for (i, slot) in self.reservation_stations.iter().enumerate() {
            if !slot.busy {
                free_slot = Some(i)
            } 
        }
        match free_slot {
            None => return false,
            Some(i) => {
                self.reservation_stations[i].op = instruction;
                match instruction {
                    // set the locations / values of the operands of the instructions, and the locations of the destination
                    processor::Instr::Add(register, operand, operand1) => rename(register, operand, operand1),
                    Instr::Sub(register, operand, operand1) => rename(register, operand, operand1),
                    Instr::Mul(register, operand, operand1) => rename(register, operand, operand1),
                    Instr::Not(operand, operand1) => todo!(),
                    Instr::And(_, operand, operand1) => todo!(),
                    Instr::Or(_, operand, operand1) => todo!(),
                    Instr::Xor(_, operand, operand1) => todo!(),
                    Instr::Cp(_, operand) => todo!(),
                    Instr::Ld(_, operand) => todo!(),
                    Instr::Ldr(_, operand, operand1) => todo!(),
                    Instr::St(operand, operand1) => todo!(),
                    Instr::B(operand) => todo!(),
                    Instr::J(operand) => todo!(),
                    Instr::Jilz(_, operand) => todo!(),
                    Instr::Bilz(_, operand) => todo!(),
                    Instr::Bilt(_, operand, operand1) => todo!(),
                    Instr::Jilt(_, operand, operand1) => todo!(),
                    Instr::Jigz(_, operand) => todo!(),
                    Instr::Bigz(_, operand) => todo!(),
                    Instr::Bigt(_, operand, operand1) => todo!(),
                    Instr::Jigt(_, operand, operand1) => todo!(),
                    Instr::Noop() => todo!(),
                    Instr::Halt() => todo!(),
                }
                return true;
            }
        }
    }

    pub fn tick(&mut self, state: &mut State) -> bool {
        let current_slot: &mut ReservationStation = &mut self.reservation_stations[self.current_inst_index];
        let current_inst: Instr = current_slot.op;
        if self.tick >= get_cycles(current_inst) {
            *state = current_inst.execute(&state);
            self.tick = 0;
            current_slot.busy = false;
        }
        return false;
    }
}