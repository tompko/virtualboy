use std::time::SystemTime;
use std::thread::{self, JoinHandle};
use std::io::{stdin, stdout, Write};
use std::sync::mpsc::{channel, Receiver};

use super::windows::debug::DebugWindow;
use super::windows::main::MainWindow;

use virtualboy_core::rom::Rom;
use virtualboy_core::virtualboy::VirtualBoy;
use virtualboy_core::instruction::{self, Instruction};

use super::command::Command;

const CPU_CYCLE_TIME_NS: i64 = 1000000000 / 20000000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Running,
    Debugging,
}

pub struct Emulator {
    vb: VirtualBoy,

    mode: Mode,

    stdin_receiver: Receiver<String>,
    _stdin_thread: JoinHandle<()>,

    cursor: u32,
    quit: bool,

    main_window: MainWindow,
    debug_window: Option<DebugWindow>,

    last_command: Option<Command>,
}

impl Emulator {
    pub fn new(rom: Rom) -> Self {
        let (stdin_sender, stdin_receiver) = channel();
        let stdin_thread = thread::spawn(move || {
            loop {
                stdin_sender.send(read_stdin()).unwrap();
            }
        });

        let mut e = Emulator {
            vb: VirtualBoy::new(rom),

            mode: Mode::Debugging,

            stdin_receiver,
            _stdin_thread: stdin_thread,

            cursor: 0,
            quit: false,

            main_window: MainWindow::new(),
            debug_window: Some(DebugWindow::new()),

            last_command: None,
        };
        e.start_debugger();

        e
    }

    pub fn run(&mut self) {
        let mut last_loop_time = SystemTime::now();
        let mut nanos_to_cover = 0;

        while !self.quit && self.main_window.is_open() {
            let now = SystemTime::now();
            let elapsed = now.duration_since(last_loop_time).unwrap().as_nanos() as i64;
            nanos_to_cover += elapsed;

            match self.mode {
                Mode::Running => {
                    while nanos_to_cover > 0 {
                        self.cursor = self.vb.cpu.reg_pc();
                        let instr = self.disassemble_instruction();
                        println!("{:08x} {}", self.cursor, instr);
                        let cycles = self.vb.step() as i64;
                        nanos_to_cover -= cycles * CPU_CYCLE_TIME_NS;
                    }
                }
                Mode::Debugging => {
                    self.run_debugger_commands();

                    self.update_windows();
                }
            }

            self.update_windows();
            last_loop_time = now;
        }
    }

    fn run_debugger_commands(&mut self) {
        while let Ok(command_string) = self.stdin_receiver.try_recv() {
            let command = match (command_string.parse(), self.last_command.clone()) {
                (Ok(Command::Repeat), Some(c)) => Ok(c),
                (Ok(Command::Repeat), None) => Err("No last command".into()),
                (Ok(c), _) => Ok(c),
                (Err(e), _) => Err(e),
            };

            match command {
                Ok(Command::ShowRegs) => {
                    // println!("pc: 0x{:08x}", self.virtual_boy.cpu.reg_pc());
                    // println!("gpr:");
                    // for i in 0..32 {
                    //     println!(" r{}: 0x{:08x}", i, self.virtual_boy.cpu.reg_gpr(i));
                    // }
                    // println!("psw: 0x{:08x}", self.virtual_boy.cpu.reg_psw());
                    // println!("eipc: 0x{:08x}", self.virtual_boy.cpu.reg_eipc());
                    // println!("eipsw: 0x{:08x}", self.virtual_boy.cpu.reg_eipsw());
                    // println!("ecr: 0x{:08x}", self.virtual_boy.cpu.reg_ecr());
                }
                Ok(Command::ShowCpuCache) => {
                    // println!("CPU Instruction Cached enable: {}", self.virtual_boy.cpu.cache.is_enabled());
                    // let (hits, misses) = self.virtual_boy.cpu.cache.stats();
                    // let percent_hit = (hits as f64 / (hits + misses) as f64) * 100.0;
                    // println!("Cache Hits: {}, Cache Misses: {} ({:.1}% hit rate)", hits, misses, percent_hit);
                    // for i in 0..128 {
                    //     println!("Entry {:3}: {}", i, self.virtual_boy.cpu.cache.entry(i));
                    // }
                },
                Ok(Command::Step(count)) => {
                    for _ in 0..count {
                        self.vb.step();
                        self.cursor = self.vb.cpu.reg_pc();
                        let instr = self.disassemble_instruction();
                        println!("{:08x} {}", self.cursor, instr);
                    }
                }
                Ok(Command::Continue) => {
                    self.mode = Mode::Running;
                    // self.time_source_start_time_ns = self.time_source.time_ns() - (self.emulated_cycles * CPU_CYCLE_TIME_NS);
                }
                Ok(Command::Goto(_addr)) => {
                    // self.cursor = addr;
                }
                Ok(Command::ShowMem(_addr)) => {
                    // if let Some(addr) = addr {
                    //     self.cursor = addr;
                    // }

                    // self.print_labels_at_cursor();

                    // const NUM_ROWS: u32 = 16;
                    // const NUM_COLS: u32 = 16;
                    // for _ in 0..NUM_ROWS {
                    //     print!("0x{:08x}  ", self.cursor);
                    //     for x in 0..NUM_COLS {
                    //         let byte = self.virtual_boy.interconnect.read_byte(self.cursor);
                    //         self.cursor = self.cursor.wrapping_add(1);
                    //         print!("{:02x}", byte);
                    //         if x < NUM_COLS - 1 {
                    //             print!(" ");
                    //         }
                    //     }
                    //     println!();
                    // }
                }
                Ok(Command::Disassemble(_count)) => {
                    // for _ in 0..count {
                    //     self.cursor = self.disassemble_instruction();
                    // }
                }
                Ok(Command::Label) => {
                    // for (name, addr) in self.labels.iter() {
                    //     println!(".{}: 0x{:08x}", name, addr);
                    // }
                }
                Ok(Command::AddLabel(ref _name, _addr)) => {
                    // self.labels.insert(name.clone(), addr);
                }
                Ok(Command::RemoveLabel(ref _name)) => {
                    // if let None = self.labels.remove(name) {
                    //     println!("Label .{} does not exist", name);
                    // }
                }
                Ok(Command::Breakpoint) => {
                    // for addr in self.breakpoints.iter() {
                    //     println!("* 0x{:08x}", addr);
                    // }
                }
                Ok(Command::AddBreakpoint(_addr)) => {
                    // self.breakpoints.insert(addr);
                }
                Ok(Command::RemoveBreakpoint(_addr)) => {
                    // if !self.breakpoints.remove(&addr) {
                    //     println!("Breakpoint at 0x{:08x} does not exist", addr);
                    // }
                }
                Ok(Command::Watchpoint) => {
                    // for addr in self.virtual_boy.cpu.watchpoints.iter() {
                    //     println!("* 0x{:08x}", addr);
                    // }
                }
                Ok(Command::AddWatchpoint(_addr)) => {
                    // self.virtual_boy.cpu.watchpoints.insert(addr);
                }
                Ok(Command::RemoveWatchpoint(_addr)) => {
                    // if !self.virtual_boy.cpu.watchpoints.remove(&addr) {
                    //     println!("Watchpoint at 0x{:08x} does not exist", addr);
                    // }
                }
                Ok(Command::Exit) => {
                    self.quit = true;
                }
                Ok(Command::Repeat) => unreachable!(),
                Err(ref e) => println!("{}", e),
            }

            if let Ok(c) = command {
                self.last_command = Some(c);
            }

            if self.mode == Mode::Debugging {
                self.print_cursor();
            }
        }
    }

    fn start_debugger(&mut self) {
        self.mode = Mode::Debugging;

        self.cursor = self.vb.cpu.reg_pc();
        let instr = self.disassemble_instruction();

        println!("{}", instr);
        self.print_cursor();
    }

    fn disassemble_instruction(&self) -> Instruction {
        let a = self.vb.interconnect.read_halfword(self.cursor);
        let b = self.vb.interconnect.read_halfword(self.cursor.wrapping_add(2));

        instruction::from_halfwords(a, b)
    }

    fn print_cursor(&self) {
        print!("(vb-rs 0x{:08x}) > ", self.cursor);
        stdout().flush().unwrap();
    }

    fn update_windows(&mut self) {
        self.main_window.update();
        if let Some(ref mut dwind) = self.debug_window {
            dwind.update_with_vb(&self.vb);

            if !dwind.is_open() {
                self.debug_window = None;
            }
        }
    }
}

fn read_stdin() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().into()
}