pub struct Ram {
    data: Box<[u8]>,
    size: u32,
}

impl Ram {
    pub fn new(size: u32) -> Self {
        if !size.is_power_of_two() {
            panic!("RAM size must be power of 2, given {}", size);
        }

        let data = vec![0; size as usize].into_boxed_slice();

        Ram{
            data,
            size,
        }
    }

    pub fn read_byte(&self, addr: u32) -> u8 {
        let addr = self.mask_addr(addr);

        self.data[addr]
    }

    pub fn read_halfword(&self, addr: u32) -> u16 {
        let addr = addr & 0xfffffffe;
        let addr = self.mask_addr(addr);

        (self.data[addr] as u16) | ((self.data[addr + 1] as u16) << 8)
    }

    pub fn write_byte(&mut self, addr:u32, val: u8) {
        let addr = self.mask_addr(addr);
        self.data[addr] = val;
    }

    pub fn write_halfword(&mut self, addr: u32, val: u16) {
        let addr = addr & 0xfffffffe;
        let addr = self.mask_addr(addr);
        self.data[addr] = val as u8;
        self.data[addr + 1] = (val >> 8) as u8;
    }

    fn mask_addr(&self, addr: u32) -> usize {
        let mask = self.size - 1;
        (addr & mask) as usize
    }
}