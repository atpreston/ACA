use either::Either;

use crate::processor::*;
use crate::registers::*;

pub const STATION_NUM: usize = 4;
pub const SLOT_NUM: usize = 8;
pub struct ReservationSlot {
    pub op : Instr,     // instruction to be performed
    pub qj : u8,        // reservation station that will produce the source operand value
    pub qk : u8,
    pub vj : i64,       // source operand value
    pub vk : i64,
    pub a : i64,        //used to hold information about memory address calculation
    pub busy : bool     // indicates the slot and execution unit are busy
}

pub struct ReservationStation {
    pub slots: [ReservationSlot; SLOT_NUM]
}

pub struct CDB {
    bus: Vec<(i64, u8)>
}

impl ReservationStation {
    pub fn issue(self, inst: Instr, registers: [Register; REGISTERS]) -> bool {
        let mut slot_index: usize;
        let mut slot: &ReservationSlot;
        let mut free = false;
        for (i, possible_slot) in self.slots.iter().enumerate() {
            if possible_slot.busy == false {
                slot_index = i;
                slot = possible_slot;
                free = true;
            }
        }
        if !free {false}
        else {
            slot = &ReservationSlot{op: inst, qj: todo!(), qk: todo!(), vj: todo!(), vk: todo!(), a: todo!(), busy: true};
            
            todo!(); // initialise rest of slot
            return true;
        }
    }
}