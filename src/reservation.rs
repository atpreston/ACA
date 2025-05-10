use crate::processor::*;
use core::fmt;

pub type CDB = Vec<(i64, (u8, u8))>; // (value, (EU, slot))

#[derive(PartialEq, Eq, Clone)]
pub struct ReservationSlot {
    pub op: Instr,      // instruction to be performed
    pub j: RegisterVal, // Either the source operand value, or the reservation station that will produce the source operand value
    pub k: RegisterVal,
    pub busy: bool,  // indicates the slot and execution unit are busy
    pub ready: bool, //indicates the operands are available
}

impl ReservationSlot {
    pub fn new() -> ReservationSlot {
        return ReservationSlot {
            op: Instr::Noop(),
            j: RegisterVal::Val(0),
            k: RegisterVal::Val(0),
            busy: false,
            ready: false,
        };
    }
}

impl fmt::Debug for ReservationSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&format_args!("Op: {:?}", &self.op))
            .field(&format_args!("j: {:?}", &self.j))
            .field(&format_args!("k: {:?}", &self.k))
            .field(&format_args!("busy? {}", &self.busy))
            .field(&format_args!("ready? {}", &self.ready))
            .finish()
    }
}

impl fmt::Display for ReservationSlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "operand: {:?}, j: {:?}, k: {:?}, busy: {}",
            self.op, self.j, self.k, self.busy
        )
    }
}

impl ReservationSlot {
    pub fn tick(&mut self, cdb: &mut CDB) {
        if !self.ready {
            for (val, tag) in cdb {
                match self.j {
                    RegisterVal::Val(_) => (),
                    RegisterVal::Tag(t) => {
                        eprintln!("CHECKING {:?} AGAINST CDB VALUE {:?}", t, *tag);
                        if (*tag) == t {
                            self.j = RegisterVal::Val(*val);
                        }
                    }
                }
                match self.k {
                    RegisterVal::Val(_) => (),
                    RegisterVal::Tag(t) => {
                        eprintln!("CHECKING {:?} AGAINST CDB VALUE {:?}", t, *tag);
                        if *tag == t {
                            self.k = RegisterVal::Val(*val);
                        }
                    }
                }
            }
            if let (RegisterVal::Val(_), RegisterVal::Val(_)) = (self.j, self.k) {
                self.ready = true;
            } else {
                self.ready = false;
            }
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct ReservationStation {
    pub slots: [ReservationSlot; SLOT_NUM],
}

impl fmt::Debug for ReservationStation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&format_args!("{:?}", &self.slots))
            .finish()
    }
}

impl ReservationStation {
    pub fn issue(
        &mut self,
        inst: Instr,
        registers: &mut [RegisterVal; REGISTERS],
        my_index: u8,
    ) -> Option<usize> {
        let mut slot_index: Option<usize> = None;
        for (i, possible_slot) in self.slots.iter_mut().enumerate() {
            if possible_slot.busy == false {
                slot_index = Some(i);
                println!("issuing {:?} on EU {}, slot {}", inst, my_index, i);
                break;
            }
        }
        match slot_index {
            None => (),
            Some(i) => {
                // initialise slot
                let operands: Vec<RegisterVal> = inst.clone().get_operands(&registers);
                eprintln!("OPERANDS: {:?}", operands);
                let destination = inst.clone().get_location();
                // println!(
                //     "#####\nInstr: {:?}, Operands: {:?}, Destination: {:?}\n#####",
                //     inst, operands, destination
                // );
                let (mut j, mut k): (RegisterVal, RegisterVal) =
                    (RegisterVal::Val(0), RegisterVal::Val(0));
                if let [jval] = *operands {
                    {
                        j = jval;
                    };
                }
                if let [jval, kval] = *operands {
                    {
                        j = jval;
                        k = kval;
                    };
                }
                if let Some(dest) = destination {
                    eprintln!("Deferring register {dest} to unit {}", my_index);
                    registers[dest as usize] = RegisterVal::Tag((my_index, i as u8));
                }
                self.slots[i as usize] = ReservationSlot {
                    op: inst,
                    j: j,
                    k: k,
                    busy: true,
                    ready: false,
                };
            }
        }
        return slot_index;
    }

    pub fn tick(&mut self, cdb: &mut CDB) -> () {
        for slot in self.slots.iter_mut() {
            slot.tick(cdb);
        }
    }
}
