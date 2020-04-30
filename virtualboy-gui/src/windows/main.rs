use minifb::{Menu, Scale, ScaleMode, Window, WindowOptions};
use super::VBWindow;

const VB_WIDTH: usize = 384;
const VB_HEIGHT: usize = 224;

pub const MENU_FILE_OPEN: usize = 1;
pub const MENU_FILE_EXIT: usize = 2;
pub const MENU_VIEW_DEBUG: usize = 3;


pub struct MainWindow {
    window: Window,

    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl MainWindow {
    pub fn new() -> Self {
        let options = WindowOptions{
            borderless: false,
            title: true,
            resize: false,
            scale: Scale::X2,
            scale_mode: ScaleMode::Stretch,
            topmost: false,
        };
        let mut window = Window::new("Virtual Boy Emulator", VB_WIDTH as _, VB_HEIGHT as _, options).unwrap(); 
        
        let mut file_menu = Menu::new("File").unwrap();
        file_menu.add_item("Open", MENU_FILE_OPEN).build();
        file_menu.add_item("Exit", MENU_FILE_EXIT).build();

        let mut view_menu = Menu::new("View").unwrap();
        view_menu.add_item("Debug", MENU_VIEW_DEBUG).build();
        
        window.add_menu(&file_menu);
        window.add_menu(&view_menu);
        MainWindow {
            window: window,
            width: VB_WIDTH,
            height: VB_HEIGHT,
            buffer: vec![0; VB_WIDTH * VB_HEIGHT],
        }
    }
}

impl VBWindow for MainWindow {
    fn update(&mut self) {
        if !self.window.is_open() {
            return;
        }

        self.window.update();
    }

    fn is_open(&self) -> bool {
        self.window.is_open()
    }

    fn get_command(&mut self) -> Option<usize> {
        self.window.is_menu_pressed()
    }
}