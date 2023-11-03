use core::fmt;
use spin::Mutex;

use crate::serial::serial_send;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if let Some(framebuffer) = unsafe{&crate::gfx::framebuffer::FRAMEBUFFER} {
            for c in s.as_bytes() {
                unsafe {
                    match c {
                        8 => {
                            COL = COL.saturating_sub(1);
                        }
                        9 => {
                            COL += 8;
                            if COL >= framebuffer.width / 8 {
                                COL = 0;
                                ROW += 1;
                                if ROW >= framebuffer.height / 8 {
                                    ROW = 0;
                                }
                            }
                        }
                        13 => {
                            COL = 0;
                        }
                        10 => {
                            COL = 0;
                            ROW += 1;
                            if ROW >= framebuffer.height / 8 {
                                ROW = 0;
                            }
                        }
                        _ => {
                            framebuffer.rect(
                                COL * 8,
                                ROW * 8,
                                COL * 8 + 8,
                                ROW * 8 + 8,
                                0x00000000,
                                0x00000000,
                            );
                            framebuffer.character(COL * 8, ROW * 8, *c, 0xFFFFFFFF);
                            COL += 1;
                            if COL >= framebuffer.width / 8 {
                                COL = 0;
                                ROW += 1;
                                if ROW >= framebuffer.height / 8 {
                                    ROW = 0;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

pub struct Writer {}
static WRITER: Mutex<Writer> = Mutex::new(Writer {});
pub static mut COL: usize = 0;
pub static mut ROW: usize = 0;

pub fn _print(args: fmt::Arguments) {
    // NOTE: Locking needs to happen around `print_fmt`, not `print_str`, as the former
    // will call the latter potentially multiple times per invocation.
    let mut writer = WRITER.lock();
    fmt::Write::write_fmt(&mut *writer, args).ok();
}

#[macro_export]
macro_rules! print {
    ($($t:tt)*) => { $crate::terminal::_print(format_args!($($t)*)) };
}

#[macro_export]
macro_rules! println {
    ()          => { $crate::print!("\n"); };
    // On nightly, `format_args_nl!` could also be used.
    ($($t:tt)*) => { $crate::print!("{}\n", format_args!($($t)*)) };
}

impl fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.as_bytes() {
            unsafe{serial_send(*c);}
        }
        Ok(())
    }
}

pub struct SerialWriter {}
static SERIAL_WRITER: Mutex<SerialWriter> = Mutex::new(SerialWriter {});

pub fn _serial_print(args: fmt::Arguments) {
    // NOTE: Locking needs to happen around `print_fmt`, not `print_str`, as the former
    // will call the latter potentially multiple times per invocation.
    let mut writer = SERIAL_WRITER.lock();
    fmt::Write::write_fmt(&mut *writer, args).ok();
}

#[macro_export]
macro_rules! serial_print {
    ($($t:tt)*) => { $crate::terminal::_serial_print(format_args!($($t)*)) };
}

#[macro_export]
macro_rules! serial_println {
    ()          => { $crate::serial_print!("\n"); };
    // On nightly, `format_args_nl!` could also be used.
    ($($t:tt)*) => { $crate::serial_print!("{}\n", format_args!($($t)*)) };
}