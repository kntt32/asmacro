use crate::registers::Register;
use super::*;
use util::svec::SVec;

type Opecode = SVec<3, u8>;

impl MlGen {
    pub fn raw_encode(
        opecode: Opecode,
        rex: RexMode,
        mod_rm: ModRmMode,
        imm: ImmMode,
        add_reg: AddRegMode,
        rel: Rel,
    ) -> Result<MlGen, ()> {
        let mut ml_gen = MlGen::new();

        ml_gen.set_opecode(opecode)?;
        ml_gen.set_rex_prefix(rex);
        ml_gen.set_mod_rm(mod_rm)?;
        ml_gen.set_imm(imm);
        ml_gen.add_regcode_to_opecode(add_reg)?;
        ml_gen.set_rel(rel);

        Ok(ml_gen)
    }

    fn set_opecode(&mut self, opecode: Opecode) -> Result<(), ()> {
        if opecode.len() == 0 {
            return Err(());
        };
        self.opecode.set_opecode(opecode);
        Ok(())
    }

    fn set_rex_prefix(&mut self, rex: RexMode) {
        match rex {
            RexMode::None => (),
            RexMode::Rex => self.rex_prefix.enable(),
            RexMode::RexW => {
                self.rex_prefix.enable();
                self.rex_prefix.set_w(true);
            }
        }
    }

    fn set_mod_rm(&mut self, mod_rm: ModRmMode) -> Result<(), ()> {
        match mod_rm {
            ModRmMode::None => (),
            ModRmMode::R(register, rm) => {
                self.mod_rm.enable();
                self.set_mod_rm_register(register)?;
                self.set_mod_rm_rm(rm)?;
            }
            ModRmMode::Dight(op_ext, rm) => {
                self.mod_rm.enable();
                self.mod_rm.set_reg(op_ext);
                self.set_mod_rm_rm(rm)?;
            }
        }
        Ok(())
    }

    fn set_mod_rm_register(&mut self, register: Register) -> Result<(), ()> {
        if register.is_64bit() {
            if register != Register::Rip {
                let regcode = register.to_regcode64()?;

                self.mod_rm.set_reg(regcode & 0b111);

                if regcode & 0b1000 != 0 {
                    if !self.rex_prefix.is_enabled() {
                        self.rex_prefix.enable();
                    }
                    self.rex_prefix.set_r(true);
                }

                Ok(())
            } else {
                Err(())
            }
        } else {
            todo!("todo");
        }
    }

    fn set_mod_rm_rm(&mut self, rm: Rm) -> Result<(), ()> {
        match rm {
            Rm::Reg(register) => {
                if register.is_64bit() {
                    let regcode = register.to_regcode64()?;

                    self.mod_rm.set_mod(0b11);
                    self.mod_rm.set_rm(regcode & 0b111);

                    if regcode & 0b1000 != 0 {
                        if !self.rex_prefix.is_enabled() {
                            self.rex_prefix.enable();
                        }
                        self.rex_prefix.set_b(true);
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
                    self.set_mod_rm_rm_scale0(base, disp)
                } else {
                    self.set_mod_rm_rm_scalenone0(scale, index, base, disp)
                }
            }
        }
    }

    fn set_mod_rm_rm_scale0(&mut self, base: Register, disp: i32) -> Result<(), ()> {
        if base.is_64bit() {
            match base {
                Register::Rip => {
                    self.mod_rm.set_mod(0b00);
                    self.mod_rm.set_rm(0b101);
                    self.disp = Disp::Disp32(disp);
                    Ok(())
                }
                Register::Rsp | Register::R12 => {
                    todo!("todo");
                }
                _ => {
                    if disp == 0 && base != Register::Rbp && base != Register::R12 {
                        self.mod_rm.set_mod(0b00);
                    } else if i8::MIN as i32 <= disp && disp <= i8::MAX as i32 {
                        self.mod_rm.set_mod(0b01);
                        self.disp = Disp::Disp8(disp as i8);
                    } else {
                        self.mod_rm.set_mod(0b10);
                        self.disp = Disp::Disp32(disp);
                    }
                    let regcode = base.to_regcode64()?;
                    self.mod_rm.set_rm(regcode & 0b111);
                    if regcode & 0b1000 != 0 {
                        if !self.rex_prefix.is_enabled() {
                            self.rex_prefix.enable();
                        }
                        self.rex_prefix.set_b(true);
                    }
                    Ok(())
                }
            }
        } else {
            todo!("todo")
        }
    }

    fn set_mod_rm_rm_scalenone0(
        &mut self,
        scale: u8,
        index: Register,
        base: Register,
        disp: i32,
    ) -> Result<(), ()> {
        if index.is_64bit() && base.is_64bit() {
            self.mod_rm.set_rm(0b100);
            self.sib.enable();

            if disp == 0 && base != Register::Rbp {
                self.disp = Disp::None;
                self.mod_rm.set_mod(0b00);
            } else if i8::MIN as i32 <= disp && disp <= i8::MAX as i32 {
                self.disp = Disp::Disp8(disp as i8);
                self.mod_rm.set_mod(0b01);
            } else {
                self.disp = Disp::Disp32(disp);
                self.mod_rm.set_mod(0b10);
            }

            let regcode = base.to_regcode64()?;
            self.sib.set_base(regcode & 0b111);
            if regcode & 0b1000 != 0 {
                if !self.rex_prefix.is_enabled() {
                    self.rex_prefix.enable();
                }
                self.rex_prefix.set_b(true);
            }

            if scale == 0 {
                self.sib.set_scale(0);
                self.sib.set_index(0b100);
            } else {
                let scale_field = match scale {
                    1 => 0b00,
                    2 => 0b01,
                    4 => 0b10,
                    8 => 0b11,
                    _ => return Err(()),
                };
                self.sib.set_scale(scale_field);
                let regcode = index.to_regcode64()?;
                self.sib.set_index(regcode & 0b111);
                if regcode & 0b1000 != 0 {
                    if !self.rex_prefix.is_enabled() {
                        self.rex_prefix.enable();
                    }
                    self.rex_prefix.set_x(true);
                }
            }
        } else {
            todo!("todo")
        }

        Ok(())
    }

    fn set_imm(&mut self, imm: ImmMode) {
        match imm {
            ImmMode::None => (),
            ImmMode::Ib(imm8) => self.imm = Imm::Imm8(imm8),
            ImmMode::Iw(imm16) => self.imm = Imm::Imm16(imm16),
            ImmMode::Id(imm32) => self.imm = Imm::Imm32(imm32),
            ImmMode::Io(imm64) => self.imm = Imm::Imm64(imm64),
        }
    }

    fn add_regcode_to_opecode(&mut self, add_reg: AddRegMode) -> Result<(), ()> {
        if add_reg != AddRegMode::None {
            let regcode = match add_reg {
                AddRegMode::Rb(register) => register.to_regcode8()?,
                AddRegMode::Ro(register) => register.to_regcode64()?,
                _ => todo!(),
            };
            self.opecode.add_register(regcode & 0b111);
            if regcode & 0b1000 != 0 {
                if !self.rex_prefix.is_enabled() {
                    self.rex_prefix.enable();
                }
                self.rex_prefix.set_r(true);
            }
        }
        Ok(())
    }

    fn set_rel(&mut self, rel: Rel) {
        match rel {
            Rel::Cd(disp) => self.disp = Disp::Disp32(disp),
            Rel::None => (),
        }
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

pub enum Rel {
    None,
    Cd(i32),
}
