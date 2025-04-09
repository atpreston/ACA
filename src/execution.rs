use crate::reservation::*;
use crate::processor::{self, *};

const fn get_cycles(inst: Instr) -> usize {
    match inst {
        processor::Instr::Add(_, _, _) => 2,
        processor::Instr::And(_, _, _) => 1,
        processor::Instr::B(_) => 2,
        processor::Instr::Cp(_, _) => 5,
        processor::Instr::Halt() => 1,
        processor::Instr::J(_) => 2,
        processor::Instr::Ld(_, _) => 1,
        processor::Instr::Ldr(_, _, _) => 4,
        processor::Instr::Mul(_, _, _) => 10,
        processor::Instr::Noop() => 1,
        processor::Instr::Not(_, _) => 1,
        processor::Instr::Or(_, _, _) => 1,
        processor::Instr::St(_, _) => 5,
        processor::Instr::Sub(_, _, _) => 3,
        processor::Instr::Xor(_, _, _) => 1,
        _ => 5 // conditional jumps/branches
    }
}

pub struct ExecutionUnit {
    reservation_station: ReservationStation,
    tick: usize,
    current_inst_index: usize,
}

impl ExecutionUnit {
    pub fn tick(&mut self, state: &mut State) -> bool {
        let current_slot: &mut ReservationSlot = &mut self.reservation_station.slots[self.current_inst_index];
        let current_inst: Instr = current_slot.op;
        if self.tick >= get_cycles(current_inst) {
            *state = current_inst.execute(&state);
            self.tick = 0;
            current_slot.busy = false;
        }
        return false;
    }
}