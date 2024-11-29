use super::*;

mod operation_list::operation_list;

struct Operator {
    mnemonic: &'static str,
    opecode: SVec<3, u8>,
    operands: SVec<2, OperandType>,
}

enum OperandType {
    Imm64,
    Reg64,
    Rm64,
}
