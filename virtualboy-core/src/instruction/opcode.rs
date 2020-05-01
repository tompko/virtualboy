use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Opcode {
    Mov,
    Add,
    Sub,
    Cmp,
    Shl,
    Shr,
    Jmp,
    Sar,
    Mul,
    Div,
    Mulu,
    Divu,
    Or,
    And,
    Xor,
    Not,
    Setf,
    Cli,
    Trap,
    Reti,
    Halt,
    Ldsr,
    Stsr,
    Sei,
    MovEa,
    AddI,
    Jr,
    Jal,
    OrI,
    AndI,
    XorI,
    MovHi,
    LdB,
    LdH,
    LdW,
    StB,
    StH,
    StW,
    InB,
    InH,
    Caxi,
    InW,
    OutB,
    OutH,
    OutW,

    // BCond opcode
    Bv,
    Bc,
    Bz,
    Bnh,
    Bn,
    Br,
    Blt,
    Ble,
    Bnv,
    Bnc,
    Bnz,
    Bh,
    Bp,
    Nop,
    Bge,
    Bgt,

    // Bit String opcodes - main opcode 0b011111
    Sch0BSU,
    Sch0BSD,
    Sch1BSU,
    Sch1BSD,
    OrBSU,
    AndBSU,
    XorBSU,
    MovBSU,
    OrNBSU,
    AndNBSU,
    XorNBSU,
    NotBSU,

    // Floating-point opcodes - main opcode 0b111110
    CmpFS,
    CvtWS,
    CvtSW,
    AddFS,
    SubFS,
    MulFS,
    DivFS,
    XB,
    XH,
    Rev,
    TrncSW,
    MpyHw,
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Opcode::Mov => write!(f, "MOV"),
            Opcode::Add => write!(f, "ADD"),
            Opcode::Sub => write!(f, "SUB"),
            Opcode::Cmp => write!(f, "CMP"),
            Opcode::Shl => write!(f, "SHL"),
            Opcode::Shr => write!(f, "SHR"),
            Opcode::Jmp => write!(f, "JMP"),
            Opcode::Sar => write!(f, "SAR"),
            Opcode::Mul => write!(f, "MUL"),
            Opcode::Div => write!(f, "DIV"),
            Opcode::Mulu => write!(f, "MULU"),
            Opcode::Divu => write!(f, "DIVU"),
            Opcode::Or => write!(f, "OR"),
            Opcode::And => write!(f, "AND"),
            Opcode::Xor => write!(f, "XOR"),
            Opcode::Not => write!(f, "NOT"),
            Opcode::Setf => write!(f, "SETF"),
            Opcode::Cli => write!(f, "CLI"),
            Opcode::Trap => write!(f, "TRAP"),
            Opcode::Reti => write!(f, "RETI"),
            Opcode::Halt => write!(f, "HALT"),
            Opcode::Ldsr => write!(f, "LDSR"),
            Opcode::Stsr => write!(f, "STSR"),
            Opcode::Sei => write!(f, "SEI"),
            Opcode::MovEa => write!(f, "MOVEA"),
            Opcode::AddI => write!(f, "ADDI"),
            Opcode::Jr => write!(f, "JR"),
            Opcode::Jal => write!(f, "JAL"),
            Opcode::OrI => write!(f, "ORI"),
            Opcode::AndI => write!(f, "ANDI"),
            Opcode::XorI => write!(f, "XORI"),
            Opcode::MovHi => write!(f, "MOVHI"),
            Opcode::LdB => write!(f, "LD.B"),
            Opcode::LdH => write!(f, "LD.H"),
            Opcode::LdW => write!(f, "LD.W"),
            Opcode::StB => write!(f, "ST.B"),
            Opcode::StH => write!(f, "ST.H"),
            Opcode::StW => write!(f, "ST.W"),
            Opcode::InB => write!(f, "IN.B"),
            Opcode::InH => write!(f, "IN.H"),
            Opcode::Caxi => write!(f, "CAXI"),
            Opcode::InW => write!(f, "IN.W"),
            Opcode::OutB => write!(f, "OUT.B"),
            Opcode::OutH => write!(f, "OUT.H"),
            Opcode::OutW => write!(f, "OUT.W"),
            Opcode::Bv => write!(f, "BV"),
            Opcode::Bc => write!(f, "BC"),
            Opcode::Bz => write!(f, "BZ"),
            Opcode::Bnh => write!(f, "BNH"),
            Opcode::Bn => write!(f, "BN"),
            Opcode::Br => write!(f, "BR"),
            Opcode::Blt => write!(f, "BLT"),
            Opcode::Ble => write!(f, "BLE"),
            Opcode::Bnv => write!(f, "BNV"),
            Opcode::Bnc => write!(f, "BNC"),
            Opcode::Bnz => write!(f, "BNZ"),
            Opcode::Bh => write!(f, "BH"),
            Opcode::Bp => write!(f, "BP"),
            Opcode::Nop => write!(f, "NOP"),
            Opcode::Bge => write!(f, "BGE"),
            Opcode::Bgt => write!(f, "BGT"),
            Opcode::Sch0BSU => write!(f, "SCH0BSU"),
            Opcode::Sch0BSD => write!(f, "SCH0BSD"),
            Opcode::Sch1BSU => write!(f, "SCH1BSU"),
            Opcode::Sch1BSD => write!(f, "SCH1BSD"),
            Opcode::OrBSU => write!(f, "ORBSU"),
            Opcode::AndBSU => write!(f, "ANDBSU"),
            Opcode::XorBSU => write!(f, "XORBSU"),
            Opcode::MovBSU => write!(f, "MOVBSU"),
            Opcode::OrNBSU => write!(f, "ORNBSU"),
            Opcode::AndNBSU => write!(f, "ANDNBSU"),
            Opcode::XorNBSU => write!(f, "XORNBSU"),
            Opcode::NotBSU => write!(f, "NOTBSU"),
            Opcode::CmpFS => write!(f, "CMPF.S"),
            Opcode::CvtWS => write!(f, "CVT.WS"),
            Opcode::CvtSW => write!(f, "CVT.SW"),
            Opcode::AddFS => write!(f, "ADDF.S"),
            Opcode::SubFS => write!(f, "SUBF.S"),
            Opcode::MulFS => write!(f, "MULF.S"),
            Opcode::DivFS => write!(f, "DIVF.S"),
            Opcode::XB => write!(f, "XB"),
            Opcode::XH => write!(f, "XH"),
            Opcode::Rev => write!(f, "REV"),
            Opcode::TrncSW => write!(f, "TRNC.SW"),
            Opcode::MpyHw => write!(f, "MPYHW"),
        }
    }
}