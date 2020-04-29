extern crate minifb;
extern crate wisegui;

mod windows;

use windows::VBWindow;
use windows::debug::DebugWindow;

fn main() {
    let mut window = DebugWindow::new();

    while window.is_open() {
        window.update();
    }
    println!("Hello, world!");
}
