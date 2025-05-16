use std::fmt;

use crate::processor::{self, *};
use crate::reservation::*;

const fn get_cycles(inst: Instr) -> usize {
    match inst {
        processor::Instr::Add(..) => 2,
        processor::Instr::And(..) => 1,
        processor::Instr::Cp(..) => 5,
        processor::Instr::Halt() => 1,
        processor::Instr::J(..) => 2,
        processor::Instr::Ld(..) => 10,
        processor::Instr::Mul(..) => 10,
        processor::Instr::Noop() => 1,
        processor::Instr::Not(..) => 1,
        processor::Instr::Or(..) => 1,
        processor::Instr::St(..) => 10,
        processor::Instr::Sub(..) => 3,
        processor::Instr::Xor(..) => 1,
        _ => 5, // conditional jumps/branches
    }
}

#[derive(PartialEq, Eq)]
pub struct ExecutionUnit {
    pub reservation_station: ReservationStation,
    tick: usize,
    current_slot_index: Option<usize>,
    executing: bool,
    cdb_result: Option<(i64, (u8, u8))>,
    pub writeback_result: Option<(ExecLocation, usize)>, // location for writeback, and index of origin slot
}

impl fmt::Debug for ExecutionUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut progress: f32 = 0 as f32;
        if let Some(slot_index) = self.current_slot_index {
            progress = self.tick.clone() as f32
                / get_cycles(self.reservation_station.slots[slot_index].op.clone()) as f32;
        }
        let percent = (progress * (100 as f32)).round();

        f.debug_tuple("")
            .field(&format_args!(
                "Res Station: {:?}",
                &self.reservation_station
            ))
            .field(&format_args!(
                "Current instruction index: {:?}",
                &self.current_slot_index
            ))
            .field(&format_args!(
                "Writeback result: {:?}",
                &self.writeback_result
            ))
            .field(&format_args!("Executing? {}", &self.executing))
            .field(&format_args!("Tick: {} ({}%)", &self.tick, percent))
            .finish()
    }
}

impl ExecutionUnit {
    pub fn all_free(&self) -> bool {
        self.reservation_station
            .slots
            .iter()
            .all(|x: &ReservationSlot| -> bool { !x.busy })
    }
    pub fn is_slot_busy(&self, index: usize) -> bool {
        self.reservation_station.slots[index].busy
    }
    pub fn new() -> ExecutionUnit {
        return ExecutionUnit {
            reservation_station: ReservationStation {
                slots: std::array::from_fn(|_| ReservationSlot::new()),
            },
            tick: 0,
            current_slot_index: Some(0),
            executing: false,
            cdb_result: None,
            writeback_result: None,
        };
    }

    pub fn issue(
        &mut self,
        inst: Instr,
        registers: &mut [RegisterVal; 8],
        my_index: u8,
        cdb: &mut CDB,
    ) -> Option<usize> {
        match self.reservation_station.issue(inst, registers, my_index) {
            Some(i) => {
                self.tick = 0;
                self.executing = false;
                self.current_slot_index = None;
                cdb.retain(|x| -> bool {
                    if x.1 == (my_index, i as u8) {
                        eprintln!("FLUSHING {:?}", x);
                    }
                    x.1 != (my_index, i as u8)
                });
                return Some(i);
            }
            None => None,
        }
    }

    pub fn tick(&mut self, prog_counter: &mut u8, cdb: &mut CDB, index: u8) -> bool {
        self.reservation_station.tick(cdb);
        // eprintln!("RES STATION: {:?}", self.reservation_station);
        // eprintln!(
        //     "EXECUTION UNIT {:?} HAS SLOT INDEX {:?}",
        //     index, self.current_slot_index
        // );

        match self.current_slot_index {
            Some(slot_index) => {
                self.tick += 1;
                let current_slot: &mut ReservationSlot =
                    &mut self.reservation_station.slots[slot_index];
                let maybe_dest = current_slot.op.clone().get_location();
                let current_inst: Instr = current_slot.op.clone();
                // return executed value after correct number of cycles
                if self.tick >= get_cycles(current_inst.clone()) {
                    let writeback_temp = current_inst.execute(
                        current_slot.j.to_either().expect_left(&format!(
                            "vj called for execution of {:?} but does not exist {:?}",
                            current_slot.op, current_slot.j
                        )),
                        current_slot.k.to_either().expect_left(&format!(
                            "vk called for execution of {:?} but does not exist {:?}",
                            current_slot.op, current_slot.k
                        )),
                        prog_counter,
                    );
                    if let Some(location) = writeback_temp.clone() {
                        self.cdb_result = Some((location.val(), (index, slot_index as u8)));
                    }
                    self.tick = 0;
                    self.current_slot_index = None;
                    current_slot.busy = false;
                    current_slot.op = Instr::Noop();

                    if let Some(destination) = maybe_dest {
                        match writeback_temp {
                            Some(ExecLocation::Reg(val, _)) => {
                                self.writeback_result =
                                    Some((ExecLocation::Reg(val, destination), slot_index))
                            }
                            Some(ExecLocation::Mem(val, _)) => {
                                self.writeback_result =
                                    Some((ExecLocation::Mem(val, destination), slot_index))
                            }
                            None => {}
                        }
                    }
                    return true;
                } else {
                    return false;
                }
            }

            None => {
                // get first instruction that has operands available
                for (slot_index, slot) in self.reservation_station.slots.iter_mut().enumerate() {
                    if slot.ready && slot.busy {
                        self.current_slot_index = Some(slot_index);
                        break;
                    }
                }
                return false;
            }
        }
    }

    pub fn writeback(&mut self, cdb: &mut CDB) {
        if let Some(pair) = self.cdb_result {
            cdb.push(pair);
            eprintln!("PUSHING {:?} TO CDB", pair);
            self.cdb_result = None;
        }
    }
}
