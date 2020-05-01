use super::ram::Ram;
use super::rom::Rom;
use super::vip::Vip;
use super::vsu::Vsu;

#[allow(dead_code)]
pub struct Interconnect {
    vip: Vip,
    vsu: Vsu,
    sys_wram: Ram,
    rom: Rom,

    reg_lcr: u8,
    reg_alr: u8,
    reg_ltd: u8,
    reg_lrd: u8,
    reg_gpil: u8,
    reg_gpih: u8,
    reg_tcrl: u8,
    reg_tcrh: u8,
    reg_tcr: u8,
    reg_wcr: u8,
    reg_gpicr: u8,
}

const VIP_START: u32 = 0x00000000;
const VIP_END: u32 = 0x00ffffff;
const VSU_START: u32 = 0x01000000;
const VSU_END: u32 = 0x01ffffff;
const HARDWARE_LINK_CTRL: u32 = 0x02000000;
const HARDWARE_AUX_LINK: u32 = 0x02000004;
const HARDWARE_LINK_SEND: u32 = 0x02000008;
const HARDWARE_LINK_RECV: u32 = 0x0200000C;
const HARDWARE_GAME_PAD_LOW: u32 = 0x02000010;
const HARDWARE_GAME_PAD_HIGH: u32 = 0x02000014;
const HARDWARE_TIMER_RELOAD_HIGH: u32 = 0x02000018;
const HARDWARE_TIMER_RELOAD_LOW: u32 = 0x0200001C;
const HARDWARE_TIMER_CTRL: u32 = 0x02000020;
const HARDWARE_WAIT_CTRL: u32 = 0x02000024;
const HARDWARE_GAME_PAD_CTRL: u32 = 0x02000028;

const UNUSED_START:u32 = 0x0200002C;
const UNUSED_END:u32 = 0x03ffffff;

const CART_EXPANSION_START: u32 = 0x04000000;
const CART_EXPANSION_END: u32 = 0x04ffffff;
const SWRAM_START: u32 = 0x05000000;
const SWRAM_END: u32 = 0x05ffffff;

const ROM_START: u32 = 0x07000000;
const ROM_END: u32 = 0x07ffffff;

impl Interconnect {
    pub fn new(rom: Rom) -> Self {
        Interconnect {
            vip: Vip::new(),
            vsu: Vsu::new(),
            sys_wram: Ram::new(64*1024*1024),
            rom,

            reg_lcr: 0,
            reg_alr: 0,
            reg_ltd: 0,
            reg_lrd: 0,
            reg_gpil: 0,
            reg_gpih: 0,
            reg_tcrl: 0,
            reg_tcrh: 0,
            reg_tcr: 0,
            reg_wcr: 0,
            reg_gpicr: 0,
        }
    }

    pub fn cycles(&mut self, _cycles: usize) -> Option<u16> {
        None
    }

    pub fn read_byte(&self, addr: u32) -> u8 {
        let addr = addr & 0x07ffffff;
        match addr {
            VIP_START..=VIP_END => self.vip.read_byte(addr - VIP_START),
            VSU_START..=VSU_END => unimplemented!(),   // VSU
            0x02000000..=0x02ffffff => unimplemented!(),   // Hardware Control Registers
            0x03000000..=0x03ffffff => unimplemented!(),   // Not Used
            CART_EXPANSION_START..=CART_EXPANSION_END => unimplemented!(),   // Cartridge Expansion
            SWRAM_START..=SWRAM_END => self.sys_wram.read_byte(addr - SWRAM_START),   // System WRAM
            0x06000000..=0x06ffffff => unimplemented!(),   // Cartridge RAM
            ROM_START..=ROM_END => self.rom.read_byte(addr - ROM_START),   // Cartridge ROM
            _ => unreachable!(),
        }
    }

    pub fn read_halfword(&self, addr: u32) -> u16 {
        let addr = addr & 0x07ffffff;
        let addr = addr & 0xfffffffe;
        match addr {
            VIP_START..=VIP_END => self.vip.read_halfword(addr - VIP_START),
            VSU_START..=VSU_END => unimplemented!(),   // VSU
            0x02000000..=0x02ffffff => unimplemented!(),   // Hardware Control Registers
            0x03000000..=0x03ffffff => unimplemented!(),   // Not Used
            CART_EXPANSION_START..=CART_EXPANSION_END => unimplemented!(),   // Cartridge Expansion
            SWRAM_START..=SWRAM_END => self.sys_wram.read_halfword(addr - SWRAM_START),   // System WRAM
            0x06000000..=0x06ffffff => unimplemented!(),   // Cartridge RAM
            ROM_START..=ROM_END => self.rom.read_halfword(addr - ROM_START),   // Cartridge ROM
            _ => unreachable!(),
        }
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        (self.read_halfword(addr) as u32) | ((self.read_halfword(addr + 2) as u32) << 16)
    }

    pub fn write_byte(&mut self, addr:u32, val: u8) {
        match addr {
            VIP_START..=VIP_END => unimplemented!(),
            VSU_START..=VSU_END => self.vsu.write_byte(addr - VSU_START, val),
            HARDWARE_LINK_CTRL => unimplemented!(),
            HARDWARE_AUX_LINK => unimplemented!(),
            HARDWARE_LINK_SEND => unimplemented!(),
            HARDWARE_LINK_RECV => unimplemented!(),
            HARDWARE_GAME_PAD_LOW => unimplemented!(),
            HARDWARE_GAME_PAD_HIGH => unimplemented!(),
            HARDWARE_TIMER_RELOAD_HIGH => {
                println!("WARN: Write to timer reload high register not fully supported");
                self.reg_tcrh = val;
            }
            HARDWARE_TIMER_RELOAD_LOW => {
                println!("WARN: Write to timer reload high register not fully supported");
                self.reg_tcrl = val;
            }
            HARDWARE_TIMER_CTRL => {
                println!("WARN: Write to timer control register not fully supported");
                self.reg_tcrl = val
            },
            HARDWARE_WAIT_CTRL => unimplemented!(),
            HARDWARE_GAME_PAD_CTRL => {
                println!("WARN: Write to game pad control register not fully supported");
                self.reg_gpicr = val
            }
            UNUSED_START..=UNUSED_END => {
                println!("WARN: Write to unused portion of memory");
            },
            CART_EXPANSION_START..=CART_EXPANSION_END => {
                println!("WARN: Writing to cartridge expansion unimplemented {:08x}={}", addr, val);
            }
            SWRAM_START..=SWRAM_END => self.sys_wram.write_byte(addr - SWRAM_START, val),   // System WRAM
            0x06000000..=0x06ffffff => unimplemented!(),   // Cartridge RAM
            ROM_START..=ROM_END => unimplemented!(),   // Cartridge ROM
            _ => unreachable!(),
        }
    }

    pub fn write_halfword(&mut self, addr:u32, val: u16) {
        match addr {
            VIP_START..=VIP_END => self.vip.write_halfword(addr - VIP_START, val),
            VSU_START..=VSU_END => self.vsu.write_halfword(addr - VSU_START, val),   // VSU
            0x02000000..=0x02ffffff => unimplemented!(),   // Hardware Control Registers
            0x03000000..=0x03ffffff => unimplemented!(),   // Not Used
            CART_EXPANSION_START..=CART_EXPANSION_END => unimplemented!(),   // Cartridge Expansion
            SWRAM_START..=SWRAM_END => self.sys_wram.write_halfword(addr - SWRAM_START, val),   // System WRAM
            0x06000000..=0x06ffffff => unimplemented!(),   // Cartridge RAM
            ROM_START..=ROM_END => unimplemented!(),   // Cartridge ROM
            _ => unreachable!(),
        }
    }

    pub fn write_word(&mut self, addr: u32, val: u32) {
        self.write_halfword(addr, val as _);
        self.write_halfword(addr + 2, (val >> 16) as _);
    }
}