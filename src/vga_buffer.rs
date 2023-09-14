use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGary = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    x: usize,                    // x
    y: usize,                    // y
    color_code: ColorCode,       // 字符颜色
    buffer: &'static mut Buffer, // VGA缓冲区
}

impl Writer {
    /**
     * 写入字符串
     */
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // 可打印字符或换行符
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // 不可打印字符
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// 写入一个字节
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(), // 换行
            byte => {
                // 如果超出屏幕宽度，换行
                if self.x >= BUFFER_WIDTH {
                    self.new_line();
                }

                let color_code = self.color_code;

                // 写入字符
                self.buffer.chars[self.y][self.x].write(ScreenChar {
                    character: byte,
                    color_code,
                });

                // 光标后移
                self.x += 1;
            }
        }
    }

    /// 换行
    pub fn new_line(&mut self) {
        self.x = 0;

        if self.y + 1 < BUFFER_HEIGHT {
            self.y += 1;
        } else {
            // 超出屏幕高度，滚动内容
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
        }
    }

    /// 清除行内容
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            character: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    /// 全局实例
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        x: 0,
        y: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}
