use virtualboy_core::virtualboy::VirtualBoy;

use super::VBPalette;

use minifb::{MouseButton, MouseMode, Scale, ScaleMode, Window, WindowOptions};
use wisegui::{Context, Painter, Color, FONT_CHAR_WIDTH, FONT_CHAR_HEIGHT};

const FONT_HALF_WIDTH: usize = FONT_CHAR_WIDTH / 2;
const FONT_HALF_HEIGHT: usize = FONT_CHAR_HEIGHT / 2;

pub struct DebugWindow {
    window: Window,
    context: Context,

    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl DebugWindow {
    pub fn new() -> Self {
        let context = Context::new(Box::new(VBPalette));
        let options = WindowOptions{
            borderless: false,
            title: true,
            resize: false,
            scale: Scale::X1,
            scale_mode: ScaleMode::Stretch,
            topmost: false,
        };
        
        // FIXME - calculate the actual width/height
        let width = 1024;
        let height = 1024;

        DebugWindow {
            window: Window::new("Debugger", width as _, height as _, options).unwrap(),
            context,
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    fn paint_with_vb(&mut self, vb: &VirtualBoy) {
        let mut painter = Painter::new(&self.context, &mut self.buffer, self.width, self.height);
        painter.clear(Color::Darkest);

        let mut x = FONT_HALF_WIDTH as i32;
        let mut y = FONT_HALF_HEIGHT as i32;
        for i in 0..16 {
            let val = vb.cpu.reg_gpr(i);
            let string = if i < 10 {
                format!("   r{}: 0x{:08x}", i, val)
            } else {
                format!("  r{}: 0x{:08x}", i, val)
            };
            painter.text(x, y, Color::Lightest, &string);
            y += FONT_CHAR_HEIGHT as i32 + FONT_HALF_HEIGHT as i32;
        }
        y += FONT_CHAR_HEIGHT as i32 + FONT_HALF_HEIGHT as i32;
        let string = format!("   pc: 0x{:08x}", vb.cpu.reg_pc());
        painter.text(x, y, Color::Lightest, &string);
        y += FONT_CHAR_HEIGHT as i32 + FONT_HALF_HEIGHT as i32;
        let string = format!(" eipc: 0x{:08x}", vb.cpu.reg_eipc());
        painter.text(x, y, Color::Lightest, &string);
        y += FONT_CHAR_HEIGHT as i32 + FONT_HALF_HEIGHT as i32;
        let string = format!("eipsw: 0x{:08x}", vb.cpu.reg_eipsw());
        painter.text(x, y, Color::Lightest, &string);

        x += (FONT_CHAR_WIDTH * "  r31: 0xffffffff  ".len()) as i32;
        y = FONT_HALF_HEIGHT as i32;
        for i in 16..32 {
            let val = vb.cpu.reg_gpr(i);
            let string = format!("r{}: 0x{:08x}", i, val);
            painter.text(x, y, Color::Lightest, &string);
            y += FONT_CHAR_HEIGHT as i32 + FONT_HALF_HEIGHT as i32;
        }
    }

    pub fn update_with_vb(&mut self, vb: &VirtualBoy) {
        if !self.window.is_open() {
            return;
        }

        let mouse_pos = {
            let p = self.window.get_mouse_pos(MouseMode::Clamp).unwrap_or((0.0, 0.0));
            (p.0 as i32, p.1 as i32)
        };
        let is_left_mouse_down = self.window.get_mouse_down(MouseButton::Left);
        self.context.update(mouse_pos, is_left_mouse_down);

        self.paint_with_vb(vb);

        self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
}