use super::interconnect::Interconnect;
use super::instruction;

#[allow(dead_code)] // FIXME - remove once we have a more complete implementation that uses all the registers
#[derive(Default)]
pub struct V810 {
    reg_pc: u32,

    reg_gpr: [u32; 32],

    reg_eipc: u32,
    reg_eipsw: u32,
    reg_fepc: u32,
    reg_fepsw: u32,
    reg_ecr: u32,
    reg_tkcw: u32,
    reg_chcw: u32,
    reg_adtre: u32,

    reg_psw_zero: bool,
    reg_psw_sign: bool,
    reg_psw_overflow: bool,
    reg_psw_carry: bool,
    reg_psw_fp_precision_degredation: bool,
    reg_psw_fp_underflow: bool,
    reg_psw_fp_overflow: bool,
    reg_psw_fp_zero_division: bool,
    reg_psw_fp_invalid_operation: bool,
    reg_psw_fp_reserved_operand: bool,
    reg_psw_interrupt_disable: bool,
    reg_psw_address_trap_enable: bool,
    reg_psw_exception_pending: bool,
    reg_psw_nmi_pending: bool,
    reg_psw_interrupt_mask_level: u8,

}

impl V810 {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn reset(&mut self) {
        self.reg_pc = 0xfffffff0;
        self.reg_eipc = 0xdeadbeef;
        self.reg_eipsw = 0xdeadbeef;
        self.reg_fepc = 0xdeadbeef;
        self.reg_fepsw = 0xdeadbeef;
        self.set_ecr(0x0000, 0xfff0);
        self.set_reg_psw(0x00008000);
        for i in 1..32 {
            self.reg_gpr[i] = 0xdeadbeef;
        }
    }

    pub fn step(&mut self, interconnect: &mut Interconnect) -> usize {
        let first_halfword = interconnect.read_halfword(self.reg_pc);
        let mut next_pc = self.reg_pc.wrapping_add(2);
        let cycles = 1; // FIXME - should be based on instruction run

        if (first_halfword >> 13) == instruction::OPCODE_BITS_BCOND_PREFIX {
            let opbits = first_halfword >> 9;

            let take_branch = match opbits {
                instruction::OPCODE_BITS_BCOND_BV => unimplemented!(),
                instruction::OPCODE_BITS_BCOND_BC => unimplemented!(),
                instruction::OPCODE_BITS_BCOND_BZ => self.reg_psw_zero,
                instruction::OPCODE_BITS_BCOND_BNH => unimplemented!(),
                instruction::OPCODE_BITS_BCOND_BN => unimplemented!(),
                instruction::OPCODE_BITS_BCOND_BR => unimplemented!(),
                instruction::OPCODE_BITS_BCOND_BLT => self.reg_psw_sign ^ self.reg_psw_overflow,
                instruction::OPCODE_BITS_BCOND_BLE => unimplemented!(),
                instruction::OPCODE_BITS_BCOND_BNV => unimplemented!(),
                instruction::OPCODE_BITS_BCOND_BNC => unimplemented!(),
                instruction::OPCODE_BITS_BCOND_BNZ => !self.reg_psw_zero,
                instruction::OPCODE_BITS_BCOND_BH => unimplemented!(),
                instruction::OPCODE_BITS_BCOND_BP => unimplemented!(),
                instruction::OPCODE_BITS_BCOND_NOP => unimplemented!(),
                instruction::OPCODE_BITS_BCOND_BGE => unimplemented!(),
                instruction::OPCODE_BITS_BCOND_BGT => unimplemented!(),
                _ => unreachable!(),
            };

            if take_branch {
                let disp = ((((first_halfword as i16) << 7) >> 7) as u32) & 0xfffffffe;
                next_pc = self.reg_pc.wrapping_add(disp);
            }
        } else {
            macro_rules! format_i {
                ($f:expr) => ({
                    let reg1 = first_halfword & 0x1f;
                    let reg2 = (first_halfword >> 5) & 0x1f;
                    $f(reg1, reg2);
                });
            }
            macro_rules! format_ii {
                ($f:expr) => ({
                    let imm5 = first_halfword & 0x1f;
                    let reg2 = (first_halfword >> 5) & 0x1f;
                    $f(imm5, reg2);
                })
            }
            macro_rules! format_iv {
                ($f:expr) => ({
                    let second_halfword = interconnect.read_halfword(next_pc);
                    next_pc = next_pc.wrapping_add(2);

                    let disp26 = ((first_halfword as u32) << 16) | (second_halfword as u32);
                    let disp26 = (((disp26 << 6) as i32) >> 6) as u32;
                    let disp26 = disp26 & 0xfffffffe;
                    $f(disp26);
                })
            }
            macro_rules! format_v {
                ($f:expr) => ({
                    let second_halfword = interconnect.read_halfword(next_pc);
                    next_pc = next_pc.wrapping_add(2);

                    let reg1 = (first_halfword & 0x1f);
                    let reg2 = ((first_halfword >> 5) & 0x1f);
                    let imm16 = second_halfword;
                    $f(imm16, reg1, reg2);
                })
            }
            macro_rules! format_vi {
                ($f:expr) => ({
                    let second_halfword = interconnect.read_halfword(next_pc);
                    next_pc = next_pc.wrapping_add(2);

                    let reg1 = (first_halfword & 0x1f);
                    let reg2 = ((first_halfword >> 5) & 0x1f);
                    let disp16 = second_halfword as i16;
                    $f(reg1, reg2, disp16);
                })
            }

            let opbits = first_halfword >> 10;

            match opbits {
                instruction::OPCODE_BITS_MOV_R => format_i!(|reg1, reg2| {
                    let val = self.reg_gpr(reg1);
                    self.set_reg_gpr(reg2, val);
                }),
                instruction::OPCODE_BITS_ADD_R => format_i!(|reg1, reg2| {
                    let lhs = self.reg_gpr(reg1);
                    let rhs = self.reg_gpr(reg2);
                    self.add(lhs, rhs, reg2);
                }),
                // instruction::OPCODE_BITS_SUB=> unimplemented!(),
                instruction::OPCODE_BITS_CMP_R => format_i!(|reg1, reg2| {
                    let lhs = self.reg_gpr(reg2);
                    let rhs = self.reg_gpr(reg1);
                    self.sub(lhs, rhs);
                }),
                // instruction::OPCODE_BITS_SHL_R=> unimplemented!(),
                // instruction::OPCODE_BITS_SHR_R=> unimplemented!(),
                instruction::OPCODE_BITS_JMP => format_i!(|reg1, _| {
                    let dest = self.reg_gpr(reg1);
                    next_pc = dest;
                }),
                // instruction::OPCODE_BITS_SAR_R=> unimplemented!(),
                // instruction::OPCODE_BITS_MUL=> unimplemented!(),
                // instruction::OPCODE_BITS_DIV=> unimplemented!(),
                // instruction::OPCODE_BITS_MULU=> unimplemented!(),
                // instruction::OPCODE_BITS_DIVU=> unimplemented!(),
                instruction::OPCODE_BITS_OR => format_i!(|reg1, reg2| {
                    let lhs = self.reg_gpr(reg1);
                    let rhs = self.reg_gpr(reg2);
                    let res = lhs | rhs;
                    self.reg_psw_overflow = false;
                    self.set_zero_sign_flags(res);
                    self.set_reg_gpr(reg2, res);
                }),
                instruction::OPCODE_BITS_AND => format_i!(|reg1, reg2| {
                    let lhs = self.reg_gpr(reg1);
                    let rhs = self.reg_gpr(reg2);
                    let res = lhs & rhs;
                    self.reg_psw_overflow = false;
                    self.set_zero_sign_flags(res);
                    self.set_reg_gpr(reg2, res);
                }),
                // instruction::OPCODE_BITS_XOR=> unimplemented!(),
                // instruction::OPCODE_BITS_NOT=> unimplemented!(),
                instruction::OPCODE_BITS_MOV_I => format_ii!(|imm5, reg2| {
                    let val = sign_extend_imm5(imm5);
                    self.set_reg_gpr(reg2, val);
                }),
                instruction::OPCODE_BITS_ADD_I => format_ii!(|imm5, reg2| {
                    let lhs = self.reg_gpr(reg2);
                    let rhs = sign_extend_imm5(imm5);
                    self.add(lhs, rhs, reg2);
                }),
                // instruction::OPCODE_BITS_SETF=> unimplemented!(),
                instruction::OPCODE_BITS_CMP_I => format_ii!(|imm5, reg2| {
                    let lhs = self.reg_gpr(reg2);
                    let rhs = sign_extend_imm5(imm5);
                    self.sub(lhs, rhs);
                }),
                instruction::OPCODE_BITS_SHL_I => format_ii!(|imm5, reg2| {
                    let val = self.reg_gpr(reg2);
                    let res = self.shl(val, imm5 as _);
                    self.set_reg_gpr(reg2, res);
                }),
                instruction::OPCODE_BITS_SHR_I => format_ii!(|imm5, reg2| {
                    let val = self.reg_gpr(reg2);
                    let res = self.shr(val, imm5 as _);
                    self.set_reg_gpr(reg2, res);
                }),
                instruction::OPCODE_BITS_CLI => format_ii!(|_, _| {
                    self.reg_psw_interrupt_disable = false;
                }),
                // instruction::OPCODE_BITS_SAR_I=> unimplemented!(),
                // instruction::OPCODE_BITS_TRAP=> unimplemented!(),
                // instruction::OPCODE_BITS_RETI=> unimplemented!(),
                // instruction::OPCODE_BITS_HALT=> unimplemented!(),
                instruction::OPCODE_BITS_LDSR => format_ii!(|imm5, reg2| {
                    let val = self.reg_gpr(reg2);
                    match imm5 {
                        instruction::OPCODE_SYSREG_EIPC => unimplemented!(),
                        instruction::OPCODE_SYSREG_EIPSW => unimplemented!(),
                        instruction::OPCODE_SYSREG_FEPC => unimplemented!(),
                        instruction::OPCODE_SYSREG_FEPSW => unimplemented!(),
                        instruction::OPCODE_SYSREG_ECR => println!("WARN: Attempted to write to ECR {}", val),
                        instruction::OPCODE_SYSREG_PSW => self.set_reg_psw(val),
                        instruction::OPCODE_SYSREG_PIR => println!("WARN: Attempted to write to PIR {}", val),
                        instruction::OPCODE_SYSREG_TKCW => println!("WARN: Attempted to write to TKCW {}", val),
                        instruction::OPCODE_SYSREG_CHCW => self.set_reg_chcw(val),
                        instruction::OPCODE_SYSREG_ADTRE => unimplemented!(),
                        _ => unreachable!(),
                    }
                }),
                instruction::OPCODE_BITS_STSR => format_ii!(|imm5, reg2| {
                    let val = match imm5 {
                        instruction::OPCODE_SYSREG_EIPC => unimplemented!(),
                        instruction::OPCODE_SYSREG_EIPSW => unimplemented!(),
                        instruction::OPCODE_SYSREG_FEPC => unimplemented!(),
                        instruction::OPCODE_SYSREG_FEPSW => unimplemented!(),
                        instruction::OPCODE_SYSREG_ECR => unimplemented!(),
                        instruction::OPCODE_SYSREG_PSW => self.reg_psw(),
                        instruction::OPCODE_SYSREG_PIR => unimplemented!(),
                        instruction::OPCODE_SYSREG_TKCW => unimplemented!(),
                        instruction::OPCODE_SYSREG_CHCW => unimplemented!(),
                        instruction::OPCODE_SYSREG_ADTRE => unimplemented!(),
                        _ => unreachable!(),
                    };
                    self.set_reg_gpr(reg2, val);
                }),
                instruction::OPCODE_BITS_SEI => format_ii!(|_, _| {
                    self.reg_psw_interrupt_disable = true;
                }),
                // instruction::OPCODE_BITS_BS=> unimplemented!(),
                instruction::OPCODE_BITS_MOVEA => format_v!(|imm16, reg1, reg2| {
                    let lhs = self.reg_gpr(reg1);
                    let rhs = imm16 as i16 as u32;
                    self.set_reg_gpr(reg2, lhs.wrapping_add(rhs));
                }),
                instruction::OPCODE_BITS_ADDI => format_v!(|imm16, reg1, reg2| {
                    let lhs = self.reg_gpr(reg1);
                    let rhs = (imm16 as i16) as u32;
                    self.add(lhs, rhs, reg2);
                }),
                instruction::OPCODE_BITS_JR => format_iv!(|disp26| {
                    next_pc = self.reg_pc.wrapping_add(disp26);
                }),
                instruction::OPCODE_BITS_JAL => format_iv!(|disp26| {
                    self.set_reg_gpr(31, next_pc);
                    next_pc = self.reg_pc.wrapping_add(disp26);
                }),
                instruction::OPCODE_BITS_ORI => format_v!(|imm16, reg1, reg2| {
                    let lhs = self.reg_gpr(reg1);
                    let rhs = imm16 as u32;
                    let res = lhs | rhs;
                    self.reg_psw_overflow = false;
                    self.set_zero_sign_flags(res);
                    self.set_reg_gpr(reg2, res);
                }),
                instruction::OPCODE_BITS_ANDI => format_v!(|imm16, reg1, reg2| {
                    let lhs = self.reg_gpr(reg1);
                    let rhs = imm16 as u32;
                    let res = lhs & rhs;
                    self.reg_psw_overflow = false;
                    self.reg_psw_sign = false;
                    self.reg_psw_zero = res == 0;
                    self.set_reg_gpr(reg2, res);
                }),
                // instruction::OPCODE_BITS_XORI=> unimplemented!(),
                instruction::OPCODE_BITS_MOVHI => format_v!(|imm16, reg1, reg2| {
                    let lhs = self.reg_gpr(reg1);
                    let rhs = (imm16 as u32) << 16;
                    self.set_reg_gpr(reg2, lhs.wrapping_add(rhs));
                }),
                instruction::OPCODE_BITS_LD_B => format_vi!(|reg1, reg2, disp16| {
                    let addr = self.reg_gpr(reg1).wrapping_add(disp16 as u32);
                    let addr = addr & 0xfffffffc;

                    let val = interconnect.read_byte(addr);
                    self.set_reg_gpr(reg2, val as _);
                }),
                instruction::OPCODE_BITS_LD_H => format_vi!(|reg1, reg2, disp16| {
                    let addr = self.reg_gpr(reg1).wrapping_add(disp16 as u32);
                    let addr = addr & 0xfffffffc;

                    let val = interconnect.read_halfword(addr);
                    self.set_reg_gpr(reg2, val as _);
                }),
                instruction::OPCODE_BITS_LD_W => format_vi!(|reg1, reg2, disp16| {
                    let addr = self.reg_gpr(reg1).wrapping_add(disp16 as u32);
                    let addr = addr & 0xfffffffc;

                    let val = interconnect.read_word(addr);
                    self.set_reg_gpr(reg2, val);
                }),
                instruction::OPCODE_BITS_ST_B => format_vi!(|reg1, reg2, disp16| {
                    let addr = self.reg_gpr(reg1).wrapping_add(disp16 as u32);
                    let addr = addr & 0xfffffffe;
                    let val = self.reg_gpr(reg2) as u8;

                    interconnect.write_byte(addr, val);
                }),
                instruction::OPCODE_BITS_ST_H => format_vi!(|reg1, reg2, disp16| {
                    let addr = self.reg_gpr(reg1).wrapping_add(disp16 as u32);
                    let addr = addr & 0xfffffffe;
                    let val = self.reg_gpr(reg2) as u16;

                    interconnect.write_halfword(addr, val);
                }),
                instruction::OPCODE_BITS_ST_W => format_vi!(|reg1, reg2, disp16| {
                    let addr = self.reg_gpr(reg1).wrapping_add(disp16 as u32);
                    let addr = addr & 0xfffffffc;
                    let val = self.reg_gpr(reg2);

                    interconnect.write_word(addr, val);
                }),
                // instruction::OPCODE_BITS_IN_B=> unimplemented!(),
                // instruction::OPCODE_BITS_IN_H=> unimplemented!(),
                // instruction::OPCODE_BITS_CAXI=> unimplemented!(),
                // instruction::OPCODE_BITS_IN_W=> unimplemented!(),
                // instruction::OPCODE_BITS_OUT_B=> unimplemented!(),
                // instruction::OPCODE_BITS_OUT_H=> unimplemented!(),
                // instruction::OPCODE_BITS_FP=> unimplemented!(),
                // instruction::OPCODE_BITS_OUT_W=> unimplemented!(),
                _ => {
                    let second_halfword = interconnect.read_halfword(next_pc);
                    let instr = instruction::from_halfwords(first_halfword, second_halfword);
                    panic!("Unimplemented instruction {}", instr);
                }
            }
        }

        self.reg_pc = next_pc;

        cycles
    }

    pub fn request_interrupt(&mut self, _interrupt_code: u16) {
        unimplemented!();
    }

    pub fn reg_pc(&self) -> u32 {
        self.reg_pc
    }

    pub fn reg_eipc(&self) -> u32 {
        self.reg_eipc
    }

    pub fn reg_eipsw(&self) -> u32 {
        self.reg_eipsw
    }

    pub fn reg_gpr(&self, index: u16) -> u32 {
        self.reg_gpr[index as usize]
    }

    fn reg_psw(&self) -> u32 {
        let mut val = 0;
        val |= if self.reg_psw_zero { 1 << 0 } else { 0 };
        val |= if self.reg_psw_sign { 1 << 1 } else { 0 };
        val |= if self.reg_psw_overflow { 1 << 2 } else { 0 };
        val |= if self.reg_psw_carry { 1 << 3 } else { 0 };
        val |= if self.reg_psw_fp_precision_degredation { 1 << 4 } else { 0 };
        val |= if self.reg_psw_fp_underflow { 1 << 5 } else { 0 };
        val |= if self.reg_psw_fp_overflow { 1 << 6 } else { 0 };
        val |= if self.reg_psw_fp_zero_division { 1 << 7 } else { 0 };
        val |= if self.reg_psw_fp_invalid_operation { 1 << 8 } else { 0 };
        val |= if self.reg_psw_fp_reserved_operand { 1 << 9 } else { 0 };
        val |= if self.reg_psw_interrupt_disable { 1 << 12 } else { 0 };
        val |= if self.reg_psw_address_trap_enable { 1 << 13 } else { 0 };
        val |= if self.reg_psw_exception_pending { 1 << 14 } else { 0 };
        val |= if self.reg_psw_nmi_pending { 1 << 15 } else { 0 };
        val |= (self.reg_psw_interrupt_mask_level as u32 & 0x0f) << 16;
        val
    }

    pub fn set_reg_gpr(&mut self, index:u16, val: u32) {
        if index != 0 {
            self.reg_gpr[index as usize] = val;
        }
    }

    fn set_ecr(&mut self, fecc: u16, eicc: u16) {
        self.reg_ecr = ((fecc as u32) << 16) | eicc as u32;
    }

    fn set_reg_chcw(&mut self, val: u32) {
        // FIXME
        println!("WARN: Cache Control Word not implemented");
    }

    #[allow(clippy::identity_op)]
    fn set_reg_psw(&mut self, val: u32) {
        self.reg_psw_zero = ((val >> 0) & 0x01) != 0;
        self.reg_psw_sign = ((val >> 1) & 0x01) != 0;
        self.reg_psw_overflow = ((val >> 2) & 0x01) != 0;
        self.reg_psw_carry = ((val >> 3) & 0x01) != 0;
        self.reg_psw_fp_precision_degredation = ((val >> 4) & 0x01) != 0;
        self.reg_psw_fp_underflow = ((val >> 5) & 0x01) != 0;
        self.reg_psw_fp_overflow = ((val >> 6) & 0x01) != 0;
        self.reg_psw_fp_zero_division = ((val >> 7) & 0x01) != 0;
        self.reg_psw_fp_invalid_operation = ((val >> 8) & 0x01) != 0;
        self.reg_psw_fp_reserved_operand = ((val >> 9) & 0x01) != 0;
        self.reg_psw_interrupt_disable = ((val >> 12) & 0x01) != 0;
        self.reg_psw_address_trap_enable = ((val >> 13) & 0x01) != 0;
        self.reg_psw_exception_pending = ((val >> 14) & 0x01) != 0;
        self.reg_psw_nmi_pending = ((val >> 15) & 0x01) != 0;
        self.reg_psw_interrupt_mask_level = ((val >> 16) & 0x0f) as u8;
    }

    fn add(&mut self, lhs: u32, rhs: u32, reg2: u16) {
        let (res, carry) = lhs.overflowing_add(rhs);
        self.set_reg_gpr(reg2, res);
        self.set_zero_sign_flags(res);
        self.reg_psw_overflow = ((!(lhs ^ rhs) & (rhs ^ res)) & 0x80000000) != 0;
        self.reg_psw_carry = carry;
    }

    fn sub(&mut self, lhs: u32, rhs: u32) -> u32 {
        let (res, carry) = lhs.overflowing_sub(rhs);

        self.set_zero_sign_flags(res);
        self.reg_psw_overflow = (((lhs ^ rhs) & !(rhs ^ res)) & 0x80000000) != 0;
        self.reg_psw_carry = carry;

        res
    }

    fn shl(&mut self, val: u32, shift: u32) -> u32 {
        let shift = shift & 0x0000001f;
        if shift == 0 {
            self.reg_psw_carry = false;
            self.reg_psw_overflow = false;
            self.set_zero_sign_flags(val);
            val
        } else {
            let res = val << (shift - 1);
            self.reg_psw_carry = (res & 0x80000000) != 0;
            let res = res << 1;
            self.reg_psw_overflow = false;
            self.set_zero_sign_flags(res);
            res
        }
    }

    fn shr(&mut self, val: u32, shift: u32) -> u32 {
        let shift = shift & 0x0000001f;
        if shift == 0 {
            self.reg_psw_carry = false;
            self.reg_psw_overflow = false;
            self.set_zero_sign_flags(val);
            val
        } else {
            let res = val >> (shift - 1);
            self.reg_psw_carry = (res & 0x00000001) != 0;
            let res = res >> 1;
            self.reg_psw_overflow = false;
            self.set_zero_sign_flags(res);
            res
        }
    }

    fn set_zero_sign_flags(&mut self, val: u32) {
        self.reg_psw_zero = val == 0;
        self.reg_psw_sign = (val & 0x80000000) != 0;
    }
}

fn sign_extend_imm5(imm5: u16) -> u32 {
    (((imm5 as i32) << 27) >> 27) as _
}