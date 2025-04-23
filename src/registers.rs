use either::*;

const REGISTERS: usize = 16;

type RegisterFile = [Either<i64, u8>; REGISTERS];
