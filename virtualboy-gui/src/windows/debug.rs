use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Scale, ScaleMode, Window, WindowOptions};
use wisegui::{Context, Painter, Color, FONT_CHAR_WIDTH, FONT_CHAR_HEIGHT};
use super::{VBWindow, VBPalette};

pub struct DebugWindow {
    window: Window,
    context: Context,

    width: usize,
    height: usize,
    buffer: Vec<u32>,

    current_prompt: String,
    prompt_stack: Vec<String>,
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
        
        let width = DebugWindow::mem_width() + DebugWindow::disasm_width();
        let height = DebugWindow::mem_disasm_height() + DebugWindow::prompt_height();

        DebugWindow {
            window: Window::new("Debugger", width as _, height as _, options).unwrap(),
            context: context,
            width: width,
            height: height,
            buffer: vec![0; width * height],
            current_prompt: String::new(),
            prompt_stack: Vec::new(),
        }
    }

    fn mem_width() -> usize {
        let left_pad = FONT_CHAR_WIDTH / 2;
        let text_width = "0xffffffff 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00".len() * FONT_CHAR_WIDTH;
        let right_pad = FONT_CHAR_WIDTH / 2;

        left_pad + text_width + right_pad
    }

    fn disasm_width() -> usize {
        let left_pad = FONT_CHAR_WIDTH / 2;
        let text_width = "0xffffffff this is a really long disasm".len() * FONT_CHAR_WIDTH;
        let right_pad = FONT_CHAR_WIDTH / 2;

        left_pad + text_width + right_pad
    }

    fn mem_disasm_height() -> usize {
        let top_pad = FONT_CHAR_HEIGHT / 2;
        let height = FONT_CHAR_HEIGHT * 16;
        let padding = FONT_CHAR_HEIGHT * 8;
        let bottom_pad = FONT_CHAR_HEIGHT / 2;

        top_pad + height + padding + bottom_pad
    }

    fn prompt_height() -> usize {
        let top_pad = FONT_CHAR_HEIGHT / 2;
        let height = FONT_CHAR_HEIGHT;
        let bottom_pad = FONT_CHAR_HEIGHT / 2;

        top_pad + height + bottom_pad
    }

    fn paint(&mut self) {
        let mut painter = Painter::new(&self.context, &mut self.buffer, self.width, self.height);
        painter.clear(Color::Darkest);

        let mut x = FONT_CHAR_WIDTH / 2;
        let mut y = FONT_CHAR_HEIGHT / 2;

        for _i in 0..16 {
            painter.text(x as _, y as _, Color::Lightest, "0xffffffff 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00");
            y += FONT_CHAR_HEIGHT + FONT_CHAR_HEIGHT / 2;
        }

        x = DebugWindow::mem_width() + FONT_CHAR_WIDTH / 2;
        y = FONT_CHAR_HEIGHT / 2;

        for _i in 0..16 {
            painter.text(x as _, y as _, Color::Lightest, "0xffffffff this is a really long disasm");
            y += FONT_CHAR_HEIGHT + FONT_CHAR_HEIGHT / 2;
        }

        x = FONT_CHAR_WIDTH / 2;
        y = DebugWindow::mem_disasm_height() + FONT_CHAR_HEIGHT / 2;
        let prompt =  format!("vb> {}_", self.current_prompt);
        painter.text(x as _, y as _, Color::Lightest, &prompt);

        x = DebugWindow::mem_width();
        y = DebugWindow::mem_disasm_height();
        painter.horizontal_line(0, self.width as _, y as _, Color::Lightest);
        painter.vertical_line(0, y as _, x as _, Color::Lightest);
    }
}

impl VBWindow for DebugWindow {
    fn update(&mut self) {
        if !self.window.is_open() {
            return;
        }

        let mouse_pos = {
            let p = self.window.get_mouse_pos(MouseMode::Clamp).unwrap_or((0.0, 0.0));
            (p.0 as i32, p.1 as i32)
        };
        let is_left_mouse_down = self.window.get_mouse_down(MouseButton::Left);
        self.context.update(mouse_pos, is_left_mouse_down);

        if let Some(keys) = self.window.get_keys_pressed(KeyRepeat::No) {
            for k in keys {
                match k {
                    Key::A => self.current_prompt += "a",
                    Key::B => self.current_prompt += "b",
                    Key::C => self.current_prompt += "c",
                    Key::D => self.current_prompt += "d",
                    Key::E => self.current_prompt += "e",
                    Key::F => self.current_prompt += "f",
                    Key::G => self.current_prompt += "g",
                    Key::H => self.current_prompt += "h",
                    Key::I => self.current_prompt += "i",
                    Key::J => self.current_prompt += "j",
                    Key::K => self.current_prompt += "k",
                    Key::L => self.current_prompt += "l",
                    Key::M => self.current_prompt += "m",
                    Key::N => self.current_prompt += "n",
                    Key::O => self.current_prompt += "o",
                    Key::P => self.current_prompt += "p",
                    Key::Q => self.current_prompt += "q",
                    Key::R => self.current_prompt += "r",
                    Key::S => self.current_prompt += "s",
                    Key::T => self.current_prompt += "t",
                    Key::U => self.current_prompt += "u",
                    Key::V => self.current_prompt += "v",
                    Key::W => self.current_prompt += "w",
                    Key::X => self.current_prompt += "x",
                    Key::Y => self.current_prompt += "y",
                    Key::Z => self.current_prompt += "z",
                    Key::Key0 => self.current_prompt += "0",
                    Key::Key1 => self.current_prompt += "1",
                    Key::Key2 => self.current_prompt += "2",
                    Key::Key3 => self.current_prompt += "3",
                    Key::Key4 => self.current_prompt += "4",
                    Key::Key5 => self.current_prompt += "5",
                    Key::Key6 => self.current_prompt += "6",
                    Key::Key7 => self.current_prompt += "7",
                    Key::Key8 => self.current_prompt += "8",
                    Key::Key9 => self.current_prompt += "9",
                    Key::Space => self.current_prompt += " ",
                    Key::Backspace => { self.current_prompt.pop(); },
                    Key::Enter => {
                        self.prompt_stack.push(self.current_prompt.clone());
                        self.current_prompt = String::new();
                    }
                    _ => {}
                }
            }
        }

        self.paint();

        self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
    }

    fn is_open(&self) -> bool {
        self.window.is_open()
    }

    fn get_command(&mut self) -> Option<usize> {
        None
    }
}