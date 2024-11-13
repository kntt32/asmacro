use super::*;
use crate::registers::Register;

impl MlGen {
    pub fn encode() -> Self {
        todo!("todo");
    }
}

pub struct Operator {
    opecode: Opecode,
    operand: Operand,
}

enum Operand {
    None,
    Reg64(Register),
}
