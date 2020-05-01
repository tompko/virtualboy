pub struct Vip {

}

impl Vip {
    pub fn new() -> Self {
        Vip {}
    }

    pub fn read_byte(&self, addr: u32) -> u8 {
        println!("WARN: Reading from VIP not implemented [0x{:08x}]", addr);
        0    
    }

    pub fn read_halfword(&self, addr: u32) -> u16 {
        println!("WARN: Reading from VIP not implemented [0x{:08x}]", addr);
        0
    }

    pub fn write_halfword(&mut self, addr: u32, val: u16) {
        println!("WARN: Writing to VIP not implemented [0x{:08x}] = {}/0x{:04x}", addr, val, val);
    }
}