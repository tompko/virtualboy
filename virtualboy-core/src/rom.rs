pub struct Rom {
    data: Box<[u8]>,
}

use std::borrow::Cow;
use std::io::{self, Read, Error, ErrorKind};
use std::fs::File;
use std::path::Path;

use encoding::Encoding;
use encoding::all::WINDOWS_31J;
use encoding::types::DecoderTrap;

pub const MIN_ROM_SIZE: usize = 1024;
pub const MAX_ROM_SIZE: usize = 16 * 1024 * 1024;

impl Rom {
    pub fn load<P: AsRef<Path>>(file_name: P) -> io::Result<Rom> {
        let mut file = File::open(file_name)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        Rom::from_bytes(&contents)
    }

    pub fn from_bytes(bytes: &[u8]) -> io::Result<Rom> {
        let data = bytes.to_vec().into_boxed_slice();

        let size = data.len();
        if size < MIN_ROM_SIZE {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid ROM size, below minimum"));
        }
        if  size > MAX_ROM_SIZE {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid ROM size, above maximum"));
        } 
        if !size.is_power_of_two() {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid ROM size, not power of two"));
        }

        Ok(Rom {
            data,
        })
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn name(&self) -> Result<String, Cow<'static, str>> {
        let mut shift_jis_title = Vec::new();

        for offset in 0xFFFFFDE0..0xFFFFFDF4 {
            shift_jis_title.push(self.read_byte(offset));
        }
        WINDOWS_31J.decode(&shift_jis_title, DecoderTrap::Strict)
    }

    pub fn maker_code(&self) -> String {
        let mut mc = String::new();
        for offset in 0xFFFFFDF9..0xFFFFFDFB {
            mc.push(self.read_byte(offset) as char);
        }
        mc
    }
    
    pub fn game_code(&self) -> String {
        let mut gc = String::new();
        for offset in 0xFFFFFDFB..0xFFFFFDFF {
            gc.push(self.read_byte(offset) as char);
        }
        gc
    }

    pub fn game_version(&self) -> String {
        let game_ver_byte = self.read_byte(0xfffffdff);
        format!("1.{}", game_ver_byte)
    }

    pub fn read_byte(&self, addr: u32) -> u8 {
        let addr = self.mask_addr(addr);
        self.data[addr as usize]
    }

    pub fn read_halfword(&self, addr: u32) -> u16 {
        let addr = self.mask_addr(addr & 0xfffffffe);
        self.data[addr] as u16 | ((self.data[addr+1] as u16) << 8)
    }

    fn mask_addr(&self, addr: u32) -> usize {
        let mask = self.size() - 1;
        addr as usize & mask
    }
}