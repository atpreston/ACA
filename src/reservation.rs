use crate::processor::*;

pub const RES_NUM: usize = 4;
pub struct ReservationStation {
    pub op : Instr,     // instruction to be performed
    pub qj : u8,        // reservation station that will produce the source operand value
    pub qk : u8,
    pub vj : i64,       // source operand value
    pub vk : i64,
    pub a : i64,        //used to hold information about memory address calculation
    pub busy : bool     // indicates the slot and execution unit are busy
}

pub struct ReservationFile {
    slots: [ReservationStation; RES_NUM]
}

pub struct CDB {
    bus: Vec<Instr>
}

impl ReservationFile {
    pub fn issue(inst: Instr) -> bool {
        false
    }
}