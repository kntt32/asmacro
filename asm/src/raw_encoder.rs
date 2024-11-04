use crate::ml_generator::{Imm, MlGen};
use crate::registers::Register;
use util::svec::SVec;

pub fn encode(opecode: SVec<3, u8>, operand: Operand) -> Result<SVec<19, u8>, ()> {
    let mut ml_gen = MlGen::new();

    // enable rex_prefix
    ml_gen.rex_prefix.enable();
    ml_gen.rex_prefix.set_w(true);

    // opecode
    ml_gen.opecode.set_opecode(opecode);

    // operand
    match operand {
        Operand::None => {
            ml_gen.rex_prefix.disable();
        }
        Operand::Reg64(register64) => {
            add_reg64_to_opecode(&mut ml_gen, register64)?;
        }
        Operand::Reg64Rm64(register64, rm64) => {
            ml_gen.mod_rm.enable();
            set_reg64_to_reg(&mut ml_gen, register64)?;
            set_rm64(&mut ml_gen, rm64)?;
        }
        Operand::Reg64Imm64(register64, imm64) => {
            add_reg64_to_opecode(&mut ml_gen, register64)?;
            set_imm64(&mut ml_gen, imm64);
        }
        _ => {
            todo!();
        }
    }

    Ok(ml_gen.encode())
}

fn set_reg64_to_reg(ml_gen: &mut MlGen, register64: Register) -> Result<(), ()> {
    if register64.is_64bit() && register64 != Register::Rip {
        let regcode = register64.to_regcode()?;

        ml_gen.mod_rm.set_reg(regcode & 0b111);
        ml_gen
            .rex_prefix
            .set_r(if regcode & 0b1000 != 0 { true } else { false });

        Ok(())
    } else {
        Err(())
    }
}

fn set_rm64(ml_gen: &mut MlGen, rm64: Rm) -> Result<(), ()> {
    match rm64 {
        Rm::Reg(register) => {
            if register.is_64bit() && register != Register::Rip {
                let regcode = register.to_regcode()?;

                ml_gen.mod_rm.set_mod(0b11);

                ml_gen.mod_rm.set_rm(regcode & 0b111);
                ml_gen
                    .rex_prefix
                    .set_b(if regcode & 0b1000 != 0 { true } else { false });

                Ok(())
            } else {
                Err(())
            }
        }
        Rm::Ref {
            base,
            scale,
            index,
            disp,
        } => {
            match scale {
                0 => {
                    todo!()
                    /*
                    //[base + disp]
                    if disp == 0 {
                        ml_gen.mod_rm.set_mod(0b00);
                    }else if disp < 256 {
                        ml_gen.mod_rm.set_mod(0b01);
                        ml_gen.disp = Disp::Disp8(disp);
                    }else {
                        ml_gen.mod_rm.set_mod(0b10);
                        ml_gen.disp = Disp::Disp32(disp);
                    }

                    if base.is_64bit() && base != Register::Rip {

                    }else {
                        Err(())
                    }*/
                }
                1 => {
                    todo!()
                }
                2 => {
                    todo!()
                }
                4 => {
                    todo!()
                }
                _ => Err(()),
            }
        }
    }
}

fn add_reg64_to_opecode(ml_gen: &mut MlGen, register64: Register) -> Result<(), ()> {
    if register64.is_64bit() && register64 != Register::Rip {
        let regcode = register64.to_regcode()?;

        ml_gen.opecode.add_register(regcode & 0b111);
        if regcode & 0b1000 != 0 {
            ml_gen.rex_prefix.set_b(true);
        } else {
            ml_gen.rex_prefix.set_b(false);
        }
        Ok(())
    } else {
        Err(())
    }
}

fn set_imm64(ml_gen: &mut MlGen, imm64: u64) {
    ml_gen.imm = Imm::Imm64(imm64);
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
    Ref {
        base: Register,
        scale: u8,
        index: Register,
        disp: u32,
    },
}
