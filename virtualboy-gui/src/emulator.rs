use super::windows::VBWindow;
use super::windows::debug::DebugWindow;
use super::windows::main::{MainWindow, MENU_FILE_OPEN, MENU_FILE_EXIT, MENU_VIEW_DEBUG};

use virtualboy_core::rom::Rom;

pub struct Emulator {
    rom: Rom,

    main_window: MainWindow,
    debug_window: Option<DebugWindow>,
}

impl Emulator {
    pub fn new(rom: Rom) -> Self {
        Emulator {
            rom: rom,

            main_window: MainWindow::new(),
            debug_window: None,
        }
    }

    pub fn run(&mut self) {
        let mut quit = false;

        while !quit && self.main_window.is_open() {
            self.main_window.update();

            if let Some(cmd) = self.main_window.get_command() {
                match cmd {
                    MENU_FILE_OPEN => {
                        // TODO - set dialog params
                        // see https://docs.rs/wfd/0.1.3/wfd/struct.DialogParams.html
                        let result = wfd::open_dialog(Default::default());

                        match result {
                            Ok(res) => println!("Open path: {:?}", res),
                            Err(wfd::DialogError::UserCancelled) => println!("User cancelled dialog"),
                            Err(e) => panic!(e),
                        }

                        // TODO - load the rom, and reset the VB
                    }
                    MENU_FILE_EXIT => {
                        quit = true;
                    }
                    MENU_VIEW_DEBUG => {
                        if self.debug_window.is_none() {
                            self.debug_window = Some(DebugWindow::new());
                        }
                    }

                    _ => {}
                }
            }

            if let Some(ref mut dwind) = self.debug_window {
                dwind.update();
                if !dwind.is_open() {
                    self.debug_window = None;
                }
            }
        }
    }
}
