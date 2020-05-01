use wisegui::{Palette, Color};

struct VBPalette;

impl Palette for VBPalette {
    fn color(&self, color: Color) -> u32 {
        match color {
            Color::Darkest => 0x000000,
            Color::Dark => 0x555555,
            Color::Light => 0xaaaaaa,
            Color::Lightest => 0xffffff,
        }
    }
}

pub mod debug;
pub mod main;