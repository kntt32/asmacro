use crate::registers::Register;
use crate::ml_generator::MlGen;
use util::svec::SVec;

pub fn encode(opecode: SVec<3, u8>, operand: Operand) -> Result<SVec<19, u8>, ()> {
    let mut ml_gen = MlGen::new();

    // opecode
    ml_gen.opecode.set_opecode(opecode);

    // operand
    todo!("");
    match operand {
        Operand::None => {
            // do nothing
        },
        Operand::Reg64(register) => {
            if !register.is_64bit() {
                return Err(())
            }
        },
        _ => {
            
        }
    }

    Ok(ml_gen.encode())
}

pub enum Operand {
    None,
    Reg64(Register),
    Reg64Rm64(Register, Rm),
    Reg64Imm64(Register, u64),
    Rm64Imm64(Rm, u64),
}

pub enum Rm {
    Reg(Register),
    Ref { base: Register, scale: u8, index: Register },
}
