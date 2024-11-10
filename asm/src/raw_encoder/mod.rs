use super::{ml_generator::*, registers::Register};
use util::svec::SVec;

type Opecode = SVec<3, u8>;

pub fn raw_encode(
    opecode: Opecode,
    rex: RexMode,
    mod_rm: ModRmMode,
    imm: ImmMode,
    add_reg: AddRegMode,
) -> Result<Code, ()> {
    let mut ml_gen = MlGen::new();

    set_opecode(&mut ml_gen, opecode)?;
    set_rex_prefix(&mut ml_gen, rex);
    set_mod_rm(&mut ml_gen, mod_rm)?;
    set_imm(&mut ml_gen, imm);

    // add_reg
    if add_reg != AddRegMode::None {
        let regcode = match add_reg {
            AddRegMode::Rb(register) => register.to_regcode8()?,
            AddRegMode::Ro(register) => register.to_regcode64()?,
            _ => todo!(),
        };
        ml_gen.opecode.add_register(regcode & 0b111);
        if regcode & 0b1000 != 0 {
            if !ml_gen.rex_prefix.is_enabled() {
                ml_gen.rex_prefix.enable();
            }
            ml_gen.rex_prefix.set_r(true);
        }
    }

    Ok(ml_gen.encode())
}

fn set_opecode(ml_gen: &mut MlGen, opecode: Opecode) -> Result<(), ()> {
    if opecode.len() == 0 {
        return Err(());
    };
    ml_gen.opecode.set_opecode(opecode);
    Ok(())
}

fn set_rex_prefix(ml_gen: &mut MlGen, rex: RexMode) {
    match rex {
        RexMode::None => (),
        RexMode::Rex => ml_gen.rex_prefix.enable(),
        RexMode::RexW => {
            ml_gen.rex_prefix.enable();
            ml_gen.rex_prefix.set_w(true);
        }
    }
}

fn set_mod_rm(ml_gen: &mut MlGen, mod_rm: ModRmMode) -> Result<(), ()> {
    match mod_rm {
        ModRmMode::None => (),
        ModRmMode::R(register, rm) => {
            ml_gen.mod_rm.enable();
            set_mod_rm_register(ml_gen, register)?;
            set_mod_rm_rm(ml_gen, rm)?;
        }
        ModRmMode::Dight(op_ext, rm) => {
            ml_gen.mod_rm.enable();
            ml_gen.mod_rm.set_reg(op_ext);
            set_mod_rm_rm(ml_gen, rm)?;
        }
    }
    Ok(())
}

fn set_mod_rm_register(ml_gen: &mut MlGen, register: Register) -> Result<(), ()> {
    if register.is_64bit() {
        if register != Register::Rip {
            let regcode = register.to_regcode64()?;

            ml_gen.mod_rm.set_reg(regcode & 0b111);

            if regcode & 0b1000 != 0 {
                if !ml_gen.rex_prefix.is_enabled() {
                    ml_gen.rex_prefix.enable();
                }
                ml_gen.rex_prefix.set_r(true);
            }

            Ok(())
        } else {
            Err(())
        }
    } else {
        todo!("todo");
    }
}

fn set_mod_rm_rm(ml_gen: &mut MlGen, rm: Rm) -> Result<(), ()> {
    match rm {
        Rm::Reg(register) => {
            if register.is_64bit() {
                let regcode = register.to_regcode64()?;

                ml_gen.mod_rm.set_mod(0b11);
                ml_gen.mod_rm.set_rm(regcode & 0b111);

                if regcode & 0b1000 != 0 {
                    if !ml_gen.rex_prefix.is_enabled() {
                        ml_gen.rex_prefix.enable();
                    }
                    ml_gen.rex_prefix.set_b(true);
                }

                Ok(())
            } else {
                todo!("todo")
            }
        }
        Rm::Ref {
            scale,
            index,
            base,
            disp,
        } => {
            if scale == 0 {
                set_mod_rm_rm_scale0(ml_gen, base, disp)
            } else {
                set_mod_rm_rm_scalenone0(ml_gen, scale, index, base, disp)
            }
        }
    }
}

fn set_mod_rm_rm_scale0(ml_gen: &mut MlGen, base: Register, disp: i32) -> Result<(), ()> {
    if base.is_64bit() {
        match base {
            Register::Rip => {
                ml_gen.mod_rm.set_mod(0b00);
                ml_gen.mod_rm.set_rm(0b101);
                ml_gen.disp = Disp::Disp32(disp);
                Ok(())
            }
            Register::Rsp | Register::R12 => {
                todo!("todo");
            }
            _ => {
                if disp == 0 && base != Register::Rbp && base != Register::R12 {
                    ml_gen.mod_rm.set_mod(0b00);
                } else if i8::MIN as i32 <= disp && disp <= i8::MAX as i32 {
                    ml_gen.mod_rm.set_mod(0b01);
                    ml_gen.disp = Disp::Disp8(disp as i8);
                } else {
                    ml_gen.mod_rm.set_mod(0b10);
                    ml_gen.disp = Disp::Disp32(disp);
                }
                let regcode = base.to_regcode64()?;
                ml_gen.mod_rm.set_rm(regcode & 0b111);
                if regcode & 0b1000 != 0 {
                    if !ml_gen.rex_prefix.is_enabled() {
                        ml_gen.rex_prefix.enable();
                    }
                    ml_gen.rex_prefix.set_b(true);
                }
                Ok(())
            }
        }
    } else {
        todo!("todo")
    }
}

fn set_mod_rm_rm_scalenone0(
    ml_gen: &mut MlGen,
    scale: u8,
    index: Register,
    base: Register,
    disp: i32,
) -> Result<(), ()> {
    if index.is_64bit() && base.is_64bit() {
        todo!("todo")
    }else {
        todo!("todo")
    }

    Ok(())
}

fn set_imm(ml_gen: &mut MlGen, imm: ImmMode) {
    match imm {
        ImmMode::None => (),
        ImmMode::Ib(imm8) => ml_gen.imm = Imm::Imm8(imm8),
        ImmMode::Iw(imm16) => ml_gen.imm = Imm::Imm16(imm16),
        ImmMode::Id(imm32) => ml_gen.imm = Imm::Imm32(imm32),
        ImmMode::Io(imm64) => ml_gen.imm = Imm::Imm64(imm64),
    }
}

pub enum RexMode {
    None,
    Rex,
    RexW,
}

pub enum ModRmMode {
    None,
    R(Register, Rm),
    Dight(u8, Rm),
}

pub enum ImmMode {
    None,
    Ib(i8),
    Iw(i16),
    Id(i32),
    Io(i64),
}

#[derive(PartialEq)]
pub enum AddRegMode {
    None,
    Rb(Register),
    Rw(Register),
    Rd(Register),
    Ro(Register),
}

pub enum Rm {
    Reg(Register),
    Ref {
        scale: u8, // 0, 1, 2, 4, 8
        index: Register,
        base: Register,
        disp: i32,
    },
}
