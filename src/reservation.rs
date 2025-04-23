use either::Either;

use crate::processor::*;

pub const STATION_NUM: usize = 4;
pub const SLOT_NUM: usize = 8;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ReservationSlot {
    pub op: Instr, // instruction to be performed
    pub qj: u8,    // reservation station that will produce the source operand value
    pub qk: u8,
    pub vj: i64, // source operand value
    pub vk: i64,
    pub a: i64,     //used to hold information about memory address calculation
    pub busy: bool, // indicates the slot and execution unit are busy
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ReservationStation {
    pub slots: [ReservationSlot; SLOT_NUM],
}

pub type CDB = Vec<(i64, u8)>;

impl ReservationStation {
    pub fn issue(mut self, inst: Instr, registers: [Register; REGISTERS]) -> bool {
        let mut slot_index: Option<usize> = None;
        for (i, possible_slot) in self.slots.iter().enumerate() {
            if possible_slot.busy == false {
                slot_index = Some(i);
            }
        }
        match slot_index {
            None => false,
            Some(i) => {
                // initialise slot
                let operands: Vec<Either<i64, u8>> = inst.get_operands(&registers);
                let (qj, qk): (u8, u8);
                let (vj, vk): (i64, i64);
                if operands.len() == 1 {
                    let op = operands[0];
                    match op {
                        either::Left(imm) => {
                            qj = 0;
                            qk = 0;
                            vj = imm;
                            vk = 0
                        }
                        either::Right(addr) => {
                            qj = addr;
                            qk = 0;
                            vj = 0;
                            vk = 0
                        }
                    }
                } else {
                    match operands[0] {
                        Either::Left(imm) => {
                            qj = 0;
                            vj = imm
                        }
                        Either::Right(addr) => {
                            qj = addr;
                            vj = 0
                        }
                    }
                    match operands[1] {
                        Either::Left(imm) => {
                            qk = 0;
                            vk = imm
                        }
                        Either::Right(addr) => {
                            qk = addr;
                            vk = 0
                        }
                    }
                }
                self.slots[i] = ReservationSlot {
                    op: inst,
                    qj: qj,
                    qk: qk,
                    vj: vj,
                    vk: vk,
                    a: 0,
                    busy: true,
                };
                return true;
            }
        }
    }
}
