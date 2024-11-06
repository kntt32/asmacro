use super::*;

pub fn set_rm64(ml_gen: &mut MlGen, rm64: Rm) -> Result<(), ()> {
    match rm64 {
        Rm::Reg(register) => set_rm64_reg(ml_gen, register),
        Rm::Ref {
            base,
            scale,
            index,
            disp,
        } => set_rm64_ref(ml_gen, base, scale, index, disp),
    }
}

fn set_rm64_reg(ml_gen: &mut MlGen, register: Register) -> Result<(), ()> {
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

fn set_rm64_ref(ml_gen: &mut MlGen, base: Register, scale: u8, index: Register, disp: i32) -> Result<(), ()> {
    match scale {
        0 => set_rm64_ref_scale0(ml_gen, base, index, disp),
        1 => {
            todo!()
        },
        2 => {
            todo!()
        },
        4 => {
            todo!()
        },
        _ => Err(()),
    }
}

fn set_rm64_ref_scale0(ml_gen: &mut MlGen, base: Register, index: Register, disp: i32) -> Result<(), ()> {
    //[base + disp]
    if base.is_64bit() {
        match base {
            Register::Rsp => {
                todo!()
            },
            Register::Rbp => {
                if disp < 256 {
                    ml_gen.mod_rm.set_mod(0b01);
                    ml_gen.disp = Disp::Disp8(disp as i8);
                }else {
                    ml_gen.mod_rm.set_mod(0b10);
                    ml_gen.disp = Disp::Disp32(disp);
                }
                ml_gen.mod_rm.set_mod(0b101);
                ml_gen.rex_prefix.set_b(false);
                Ok(())
            },
            Register::R12 => {
                todo!()
            },
            Register::R13 => {
                if disp < 256 {
                    ml_gen.mod_rm.set_mod(0b01);
                    ml_gen.disp = Disp::Disp8(disp as i8);
                }else {
                    ml_gen.mod_rm.set_mod(0b10);
                    ml_gen.disp = Disp::Disp32(disp);
                }
                ml_gen.mod_rm.set_rm(0b101);
                ml_gen.rex_prefix.set_b(true);
                Ok(())
            },
            Register::Rip => {
                ml_gen.mod_rm.set_mod(0b00);

                ml_gen.disp = Disp::Disp32(disp);

                ml_gen.mod_rm.set_rm(0b101);
                ml_gen.rex_prefix.set_b(false);
                Ok(())
            },
            _ => {
                if disp == 0 {
                    ml_gen.mod_rm.set_mod(0b00);
                }else if disp < 256 {
                    ml_gen.mod_rm.set_mod(0b01);
                    ml_gen.disp = Disp::Disp8(disp as i8);
                }else {
                    ml_gen.mod_rm.set_mod(0b10);
                    ml_gen.disp = Disp::Disp32(disp);
                }
                let regcode = base.to_regcode()?;
                ml_gen.mod_rm.set_rm(regcode & 0b111);
                ml_gen.rex_prefix.set_b(if regcode & 0b1000 != 0 { true } else { false });
                Ok(())
            }
        }
    }else {
        Err(())
    }
}
