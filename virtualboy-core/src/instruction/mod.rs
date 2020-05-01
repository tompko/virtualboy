use std::fmt::{Display, Formatter, Result};

mod bits;
mod opcode;

pub use bits::*;
pub use opcode::*;

#[derive(Debug)]
pub enum Instruction {
    Illegal,
    FormatI(Opcode, u16, u16),
    FormatII(Opcode, u16, u16),
    FormatIII(Opcode, i16),
    FormatIV(Opcode, u32),
    FormatV(Opcode, u16, u16, u16),
    FormatVI(Opcode, u16, u16, i16),
    FormatVII(Opcode, u16, u16, u16),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Instruction::Illegal => write!(f, "ILLEGAL OPCODE"),
            Instruction::FormatI(Opcode::Jmp, reg1, _) => write!(f, "{} [r{}]", Opcode::Jmp, reg1),
            Instruction::FormatI(opcode, reg1, reg2) => write!(f, "{} r{} r{}", opcode, reg1, reg2),
            Instruction::FormatII(opcode, reg2, imm5) => write!(f, "{} {} r{}", opcode, imm5, reg2),
            Instruction::FormatIII(opcode, disp9) => write!(f, "{} {}", opcode, disp9),
            Instruction::FormatIV(opcode, disp26) => write!(f, "{} {}", opcode, disp26),
            Instruction::FormatV(opcode, reg1, reg2, imm16) => write!(f, "{} 0x{:04x} r{} r{}", opcode, imm16, reg1, reg2),
            Instruction::FormatVI(opcode, reg1, reg2, disp16) => write!(f, "{} {}[gpr{}] r{}", opcode, disp16, reg1, reg2),
            _ => panic!("Display not implemented for {:?}", self),
        }
    }
}

pub fn from_halfwords(a: u16, b: u16) -> Instruction {
    if (a >> 13) == OPCODE_BITS_BCOND_PREFIX {
        let opbits = a >> 9;
        match opbits {
            OPCODE_BITS_BCOND_BV => format_iii(Opcode::Bv, a, b),
            OPCODE_BITS_BCOND_BC => format_iii(Opcode::Bc, a, b),
            OPCODE_BITS_BCOND_BZ => format_iii(Opcode::Bz, a, b),
            OPCODE_BITS_BCOND_BNH => format_iii(Opcode::Bnh, a, b),
            OPCODE_BITS_BCOND_BN => format_iii(Opcode::Bn, a, b),
            OPCODE_BITS_BCOND_BR => format_iii(Opcode::Br, a, b),
            OPCODE_BITS_BCOND_BLT => format_iii(Opcode::Blt, a, b),
            OPCODE_BITS_BCOND_BLE => format_iii(Opcode::Ble, a, b),
            OPCODE_BITS_BCOND_BNV => format_iii(Opcode::Bnv, a, b),
            OPCODE_BITS_BCOND_BNC => format_iii(Opcode::Bnc, a, b),
            OPCODE_BITS_BCOND_BNZ => format_iii(Opcode::Bnz, a, b),
            OPCODE_BITS_BCOND_BH => format_iii(Opcode::Bh, a, b),
            OPCODE_BITS_BCOND_BP => format_iii(Opcode::Bp, a, b),
            OPCODE_BITS_BCOND_NOP => format_iii(Opcode::Nop, a, b),
            OPCODE_BITS_BCOND_BGE => format_iii(Opcode::Bge, a, b),
            OPCODE_BITS_BCOND_BGT => format_iii(Opcode::Bgt, a, b),
            _ => unreachable!(),
        }
    } else {
        let opbits = a >> 10;

        match opbits {
            OPCODE_BITS_MOV_R => format_i(Opcode::Mov, a, b),
            OPCODE_BITS_ADD_R => format_i(Opcode::Add, a, b),
            OPCODE_BITS_SUB => format_i(Opcode::Sub, a, b),
            OPCODE_BITS_CMP_R => format_i(Opcode::Cmp, a, b),
            OPCODE_BITS_SHL_R => format_i(Opcode::Shl, a, b),
            OPCODE_BITS_SHR_R => format_i(Opcode::Shr, a, b),
            OPCODE_BITS_JMP => format_i(Opcode::Jmp, a, b),
            OPCODE_BITS_SAR_R => format_i(Opcode::Sar, a, b),
            OPCODE_BITS_MUL => format_i(Opcode::Mul, a, b),
            OPCODE_BITS_DIV => format_i(Opcode::Div, a, b),
            OPCODE_BITS_MULU => format_i(Opcode::Mulu, a, b),
            OPCODE_BITS_DIVU => format_i(Opcode::Divu, a, b),
            OPCODE_BITS_OR => format_i(Opcode::Or, a, b),
            OPCODE_BITS_AND => format_i(Opcode::And, a, b),
            OPCODE_BITS_XOR => format_i(Opcode::Xor, a, b),
            OPCODE_BITS_NOT => format_i(Opcode::Not, a, b),
            OPCODE_BITS_MOV_I => format_ii(Opcode::Mov, a, b),
            OPCODE_BITS_ADD_I => format_ii(Opcode::Add, a, b),
            OPCODE_BITS_SETF => format_ii(Opcode::Setf, a, b),
            OPCODE_BITS_CMP_I => format_ii(Opcode::Cmp, a, b),
            OPCODE_BITS_SHL_I => format_ii(Opcode::Shl, a, b),
            OPCODE_BITS_SHR_I => format_ii(Opcode::Shr, a, b),
            OPCODE_BITS_CLI => format_ii(Opcode::Cli, a, b),
            OPCODE_BITS_SAR_I => format_ii(Opcode::Sar, a, b),
            OPCODE_BITS_TRAP => format_ii(Opcode::Trap, a, b),
            OPCODE_BITS_RETI => format_ii(Opcode::Reti, a, b),
            OPCODE_BITS_HALT => format_ii(Opcode::Halt, a, b),
            OPCODE_BITS_LDSR => format_ii(Opcode::Ldsr, a, b),
            OPCODE_BITS_STSR => format_ii(Opcode::Stsr, a, b),
            OPCODE_BITS_SEI => format_ii(Opcode::Sei, a, b),
            OPCODE_BITS_BS => unimplemented!(),
            OPCODE_BITS_MOVEA => format_v(Opcode::MovEa, a, b),
            OPCODE_BITS_ADDI => format_v(Opcode::AddI, a, b),
            OPCODE_BITS_JR => format_iv(Opcode::Jr, a, b),
            OPCODE_BITS_JAL => format_iv(Opcode::Jal, a, b),
            OPCODE_BITS_ORI => format_v(Opcode::OrI, a, b),
            OPCODE_BITS_ANDI => format_v(Opcode::AndI, a, b),
            OPCODE_BITS_XORI => format_v(Opcode::XorI, a, b),
            OPCODE_BITS_MOVHI => format_v(Opcode::MovHi, a, b),
            OPCODE_BITS_LD_B => format_vi(Opcode::LdB, a, b),
            OPCODE_BITS_LD_H => format_vi(Opcode::LdH, a, b),
            OPCODE_BITS_LD_W => format_vi(Opcode::LdW, a, b),
            OPCODE_BITS_ST_B => format_vi(Opcode::StB, a, b),
            OPCODE_BITS_ST_H => format_vi(Opcode::StH, a, b),
            OPCODE_BITS_ST_W => format_vi(Opcode::StW, a, b),
            OPCODE_BITS_IN_B => format_vi(Opcode::InB, a, b),
            OPCODE_BITS_IN_H => format_vi(Opcode::InH, a, b),
            OPCODE_BITS_CAXI => format_vi(Opcode::Caxi, a, b),
            OPCODE_BITS_IN_W => format_vi(Opcode::InW, a, b),
            OPCODE_BITS_OUT_B => format_vi(Opcode::OutB, a, b),
            OPCODE_BITS_OUT_H => format_vi(Opcode::OutH, a, b),
            OPCODE_BITS_FP => unimplemented!(),
            OPCODE_BITS_OUT_W => format_vi(Opcode::OutW, a, b),
            _ => Instruction::Illegal,
        }
    }
}

fn format_i(opcode: Opcode, a: u16, _b: u16) -> Instruction {
    let reg1 = a & 0x1f;
    let reg2 = (a >> 5) & 0x1f;

    Instruction::FormatI(opcode, reg1, reg2)
}

fn format_ii(opcode: Opcode, a: u16, _b: u16) -> Instruction {
    let imm5 = a & 0x1f;
    let reg2 = (a >> 5) & 0x1f;

    Instruction::FormatII(opcode, reg2, imm5)
}

fn format_iii(opcode: Opcode, a: u16, _b: u16) -> Instruction {
    let disp9 = ((((a as i16) << 7) >> 7) as u16) & 0xfffe;

    Instruction::FormatIII(opcode, disp9 as _)
}

fn format_iv(opcode: Opcode, a: u16, b: u16) -> Instruction {
    let disp26 = (((a as u32) >> 10) << 16) | (b as u32);
    let disp26 = (((disp26 << 6) as i32) >> 6) as u32;

    Instruction::FormatIV(opcode, disp26)
}

fn format_v(opcode: Opcode, a: u16, b: u16) -> Instruction {
    let reg1 = a & 0x1f;
    let reg2 = (a >> 5) & 0x1f;
    let imm16 = b;

    Instruction::FormatV(opcode, reg1, reg2, imm16)
}

fn format_vi(opcode: Opcode, a: u16, b: u16) -> Instruction {
    let reg1 = a & 0x1f;
    let reg2 = (a >> 5) & 0x1f;
    let disp16 = b as i16;

    Instruction::FormatVI(opcode, reg1, reg2, disp16)
}

fn format_vii(opcode: Opcode, a: u16, b: u16) -> Instruction {
    let reg1 = a & 0x1f;
    let reg2 = (a >> 5) & 0x1f;
    let subop = b >> 10;

    Instruction::FormatVII(opcode, subop, reg1, reg2)
}
