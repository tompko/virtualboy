use clap::{App, Arg};

pub struct CmdLineCfg {
    pub rom_path: String
}

pub fn parse_args() -> CmdLineCfg {
    let app = App::new("Virtual Boy")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("A virtual boy emulator")
        .arg(Arg::with_name("ROM")
            .help("The path to the ROM to load")
            .required(true)
            .index(1)
        );
    let matches = app.get_matches();

    let rom_path = matches.value_of("ROM").unwrap();

    CmdLineCfg{
        rom_path: rom_path.into(),
    }
}