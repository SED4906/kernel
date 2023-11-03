use x86::io::{outb, inb};

use crate::error::KernelError;

const COM1:u16 = 0x3f8;

pub unsafe fn serial_init() -> Result<(), KernelError> {
    outb(COM1+1, 0);
    outb(COM1+3, 0x80);
    outb(COM1, 0x03);
    outb(COM1+1, 0);
    outb(COM1+3, 0x03);
    outb(COM1+2, 0xC7);
    outb(COM1+4, 0x03);
    outb(COM1+4, 0x1E);
    outb(COM1, 0xAE);
    if inb(COM1) != 0xAE {return Err(KernelError::SerialInit)}
    outb(COM1+4, 0x03);
    Ok(())
}

pub unsafe fn serial_send(byte: u8) {
    while inb(COM1+5) & 0x20 == 0 {}
    outb(COM1, byte);
}