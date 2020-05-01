pub struct Vsu {}

impl Vsu {
    pub fn new() -> Self {
        Vsu {}
    }

    pub fn write_byte(&mut self, addr: u32, val: u8) {
        println!("WARNL Writing to VSU not implemented [0x{:08x}] = {:04x}", addr, val);
    }

    pub fn write_halfword(&mut self, addr: u32, val: u16) {
        println!("WARN: Writing to VSU not implemented [0x{:08x}] = {}/0x{:04x}", addr, val, val);
    }
}