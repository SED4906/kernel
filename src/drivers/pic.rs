use x86::io::{outb, inb};

const PIC1: u16 = 0x20;
const PIC2: u16 = 0xA0;

pub unsafe fn pic_init() {
    outb(PIC1, 0x11);
    outb(PIC2, 0x11);
    outb(PIC1+1, 0x20);
    outb(PIC2+1, 0x28);
    outb(PIC1+1, 4);
    outb(PIC2+1, 2);
    outb(PIC1+1, 0x01);
    outb(PIC2+1, 0x01);
    outb(PIC1+1, 0xFF);
    outb(PIC2+1, 0xFF);
}

pub unsafe fn pic_clear_mask(which: u8) {
    if which >= 8 {
        pic_clear_mask(2);
    }
    let mut mask = inb(if which < 8 {PIC1} else {PIC2}+1);
    mask &= !(1<<(which&7));
    outb(if which < 8 {PIC1} else {PIC2}+1, mask);
}