use util::svec::SVec;
pub use instruction_database::INSTRUCTION_LIST;

mod instruction_database;

pub struct Instruction {
    encoding: EncodingRule,
    expression: Expression,
}

impl Instruction {
    pub fn is_match(&self, expr: &str) -> bool {
        self.expression.is_match(expr)
    }
}

pub struct EncodingRule {
    opecode: SVec<3, u8>,
    rex: Option<RexRule>,
    modrm: Option<ModRmRule>,
    imm: Option<ImmRule>,
    addreg: Option<AddRegRule>,
}

pub enum RexRule {
    Rex,
    RexW,
}

pub enum ModRmRule {
    R,
    Dight(u8),
}

pub enum ImmRule {
    Ib,
    Iw,
    Id,
    Io,
}

pub enum AddRegRule {
    Rb,
    Rw,
    Rd,
    Ro,
}

pub struct Expression {
    mnemonic: &'static str,
    operands: [Option<OperandType>; 2],
}

impl Expression {
    pub fn is_match(&self, expr: &str) -> bool {
        todo!()
    }
}

#[derive(Clone, Copy)]
pub enum OperandType {
    Rel32,
    R8,
    R16,
    R32,
    R64,
    Imm8,
    Imm16,
    Imm32,
    Imm64,
    Rm8,
    Rm16,
    Rm32,
    Rm64,
}

