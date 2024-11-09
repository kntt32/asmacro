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

fn set_rm64_ref(
    ml_gen: &mut MlGen,
    base: Register,
    scale: u8,
    index: Register,
    disp: i32,
) -> Result<(), ()> {
    match scale {
        0 => set_rm64_ref_scale0(ml_gen, base, disp),
        _ => set_rm64_ref_scale(ml_gen, base, scale, index, disp),
    }
}

fn set_rm64_ref_scale0(ml_gen: &mut MlGen, base: Register, disp: i32) -> Result<(), ()> {
    //[base + disp]
    if base.is_64bit() {
        match base {
            Register::Rsp | Register::R12 => {
                if disp == 0 {
                    ml_gen.mod_rm.set_mod(0b00);
                }else if -128 <= disp && disp < 128 {
                    ml_gen.mod_rm.set_mod(0b01);
                    ml_gen.disp = Disp::Disp8(disp as i8);
                }else {
                    ml_gen.mod_rm.set_mod(0b10);
                    ml_gen.disp = Disp::Disp32(disp);
                }
                ml_gen.mod_rm.set_rm(0b100);

                ml_gen.sib.enable();
                ml_gen.sib.set_scale(0);

                ml_gen.sib.set_index(0b100);// none
                ml_gen.rex_prefix.set_x(false);

                let regcode = base.to_regcode()?;
                ml_gen.sib.set_base(regcode & 0b111);
                ml_gen.rex_prefix.set_b(regcode & 0b1000 != 0);

                Ok(())
            }
            Register::Rip => {
                ml_gen.mod_rm.set_mod(0b00);

                ml_gen.disp = Disp::Disp32(disp);

                ml_gen.mod_rm.set_rm(0b101);
                ml_gen.rex_prefix.set_b(false);
                Ok(())
            }
            _ => {
                if disp == 0 && base != Register::Rbp && base != Register::R13 {
                    ml_gen.mod_rm.set_mod(0b00);
                } else if -128 <= disp && disp < 128 {
                    ml_gen.mod_rm.set_mod(0b01);
                    ml_gen.disp = Disp::Disp8(disp as i8);
                } else {
                    ml_gen.mod_rm.set_mod(0b10);
                    ml_gen.disp = Disp::Disp32(disp);
                }

                let regcode = base.to_regcode()?;
                ml_gen.mod_rm.set_rm(regcode & 0b111);
                ml_gen
                    .rex_prefix
                    .set_b(if regcode & 0b1000 != 0 { true } else { false });

                Ok(())
            }
        }
    } else {
        Err(())
    }
}

fn set_rm64_ref_scale(
    ml_gen: &mut MlGen,
    base: Register,
    scale: u8,
    index: Register,
    disp: i32,
) -> Result<(), ()> {
    if !(scale == 1 || scale == 2 || scale == 4 || scale == 8) {
        panic!("invalid input")
    };

    ml_gen.sib.enable();

    // disp
    if disp == 0 {
        ml_gen.mod_rm.set_mod(0b00);
    } else if -128 <= disp && disp < 128 {
        ml_gen.mod_rm.set_mod(0b01);
        ml_gen.disp = Disp::Disp8(disp as i8);
    } else {
        ml_gen.mod_rm.set_mod(0b10);
        ml_gen.disp = Disp::Disp32(disp);
    }

    // base
    if base == Register::Rbp || base == Register::R13 {
        if disp == 0 {
            ml_gen.mod_rm.set_mod(0b01);
            ml_gen.disp = Disp::Disp8(0);
        }
    }
    if !(base.is_64bit() && base != Register::Rip) {
        return Err(());
    };
    let regcode = base.to_regcode()?;
    ml_gen.sib.set_base(regcode & 0b111);
    ml_gen.rex_prefix.set_b(regcode & 0b1000 != 0);

    //scale
    ml_gen.sib.set_scale(match scale {
        1 => 0b00,
        2 => 0b01,
        4 => 0b10,
        8 => 0b11,
        _ => return Err(()),
    });

    // index
    if index == Register::Rsp {
        return Err(());
    }
    if !(index.is_64bit() && index != Register::Rip) {
        return Err(());
    };
    let regcode = index.to_regcode()?;
    ml_gen.sib.set_index(regcode & 0b111);
    ml_gen.rex_prefix.set_x(regcode & 0b1000 != 0);

    Ok(())
}
