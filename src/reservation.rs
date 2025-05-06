use core::fmt;

use either::Either::{self, *};

use crate::processor::*;

pub const SLOT_NUM: usize = 1;

#[derive(PartialEq, Eq, Clone)]
pub struct ReservationSlot {
    pub op: Instr,          // instruction to be performed
    pub j: Either<i64, u8>, // Either the source operand value, or the reservation station that will produce the source operand value
    pub k: Either<i64, u8>,
    pub busy: bool, // indicates the slot and execution unit are busy
}

impl ReservationSlot {
    pub fn new() -> ReservationSlot {
        return ReservationSlot {
            op: Instr::Noop(),
            j: Left(0),
            k: Left(0),
            busy: false,
        };
    }
}

impl fmt::Debug for ReservationSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.j)
            .field(&self.k)
            .field(&self.busy)
            .finish()
    }
}

impl fmt::Display for ReservationSlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "operand: {:?}, j: {}, k: {}, busy: {}",
            self.op, self.j, self.k, self.busy
        )
    }
}
#[derive(PartialEq, Eq)]
pub struct ReservationStation {
    pub slots: [ReservationSlot; SLOT_NUM],
}

impl fmt::Debug for ReservationStation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("").field(&self.slots).finish()
    }
}

pub type CDB = Vec<(i64, u8)>;

impl ReservationStation {
    pub fn issue(&mut self, inst: Instr, registers: &[RegisterVal; REGISTERS]) -> bool {
        let mut slot_index: Option<usize> = None;
        // if self
        //     .slots
        //     .iter()
        //     .any(|x: &ReservationSlot| -> bool { x.busy })
        // {
        //     println!("ONE SLOT BUSY");
        // }
        for (i, possible_slot) in self.slots.iter_mut().enumerate() {
            if possible_slot.busy == false {
                slot_index = Some(i);
                println!("issuing on slot {}", i);
                break;
            }
        }
        match slot_index {
            None => false,
            Some(i) => {
                // initialise slot
                let operands: Vec<Either<i64, u8>> = inst.clone().get_operands(&registers);
                let (mut j, mut k): (Either<i64, u8>, Either<i64, u8>) = (Left(0), Left(0));
                if let [jval] = *operands {
                    {
                        j = jval
                    };
                }
                if let [_, kval] = *operands {
                    {
                        k = kval
                    };
                }
                self.slots[i] = ReservationSlot {
                    op: inst,
                    j: j,
                    k: k,
                    busy: true,
                };
                return true;
            }
        }
    }
}
