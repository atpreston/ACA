use std::fmt;

use either::Either::*;

use crate::processor::{self, *};
use crate::{registers, reservation::*};

const fn get_cycles(inst: Instr) -> usize {
    match inst {
        processor::Instr::Add(_, _, _) => 2,
        processor::Instr::And(_, _, _) => 1,
        processor::Instr::Cp(_, _) => 5,
        processor::Instr::Halt() => 1,
        processor::Instr::J(_) => 2,
        processor::Instr::Ld(_, _) => 1,
        processor::Instr::Mul(_, _, _) => 10,
        processor::Instr::Noop() => 1,
        processor::Instr::Not(_, _) => 1,
        processor::Instr::Or(_, _, _) => 1,
        processor::Instr::St(_, _) => 5,
        processor::Instr::Sub(_, _, _) => 3,
        processor::Instr::Xor(_, _, _) => 1,
        _ => 5, // conditional jumps/branches
    }
}

#[derive(PartialEq, Eq)]
pub struct ExecutionUnit {
    reservation_station: ReservationStation,
    tick: usize,
    current_inst_index: usize,
    executing: bool,
}

impl fmt::Debug for ExecutionUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.reservation_station)
            .field(&self.is_busy())
            .finish()
    }
}

impl ExecutionUnit {
    pub fn is_busy(&self) -> bool {
        self.reservation_station
            .slots
            .iter()
            .all(|x: &ReservationSlot| -> bool { x.busy })
    }
    pub fn new() -> ExecutionUnit {
        return ExecutionUnit {
            reservation_station: ReservationStation {
                slots: std::array::from_fn(|_| ReservationSlot::new()),
            },
            tick: 0,
            current_inst_index: 0,
            executing: false,
        };
    }

    pub fn issue(&mut self, inst: Instr, registers: &[either::Either<i64, u8>; 8]) -> bool {
        self.reservation_station.issue(inst, registers)
    }

    pub fn tick(
        &mut self,
        registers: &mut [either::Either<i64, u8>; 8],
        memory: &mut [i64; 2048],
        prog_counter: &mut u8,
    ) -> bool {
        self.tick += 1;
        let current_slot: &mut ReservationSlot =
            &mut self.reservation_station.slots[self.current_inst_index];
        let current_inst: Instr = current_slot.op.clone();
        match self.executing {
            true => {
                // return executed value after correct number of cycles
                if self.tick >= get_cycles(current_inst.clone()) {
                    let destination = current_inst.clone().get_location();
                    current_inst.execute(
                        current_slot
                            .j
                            .expect_left("vj called for execution but does not exist"),
                        current_slot
                            .j
                            .expect_left("vk called for execution but does not exist"),
                        destination,
                        registers,
                        memory,
                        prog_counter,
                    );
                    self.tick = 0;
                    current_slot.busy = false;
                    return true;
                }
            }
            false => {
                // start executing if operands are available
                if let (Left(_), Left(_)) = (current_slot.j, current_slot.k) {
                    self.executing = true;
                }
            }
        }
        return false;
    }
}
