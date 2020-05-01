#![allow(clippy::unreadable_literal)]

extern crate clap;
#[macro_use] extern crate nom;
extern crate minifb;
extern crate wfd;
extern crate wisegui;
extern crate virtualboy_core;

mod argparse;
mod command;
mod emulator;
mod windows;

use emulator::Emulator;
use virtualboy_core::rom::Rom;

fn humanize(size: usize) -> String {
    if size < 1024 {
        format!("{}B", size)
    } else if size < 1024*1024 {
        format!("{}KiB", size / 1024)
    } else {
        format!("{}MiB", size / 1024 / 1024)
    }
}

fn main() {
    let cmd_line_cfg = argparse::parse_args();

    println!("Loading log file {}", cmd_line_cfg.rom_path);

    let rom = Rom::load(&cmd_line_cfg.rom_path).unwrap();

    println!("ROM size: {}", humanize(rom.size()));
    println!("Header info:");
    println!(" name: \"{}\"", rom.name().unwrap());
    println!(" maker code: \"{}\"", rom.maker_code());
    println!(" game code: \"{}\"", rom.game_code());
    println!(" game version: \"{}\"", rom.game_version());

    let mut emulator = Emulator::new(rom);
    emulator.run();
}
