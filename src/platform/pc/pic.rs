use x86::io::{inb,outb};

pub unsafe fn pic_remap() {
    outb(0x20, 0x11);
    outb(0xA0, 0x11);
    outb(0x21, 0x20);
    outb(0xA1, 0x28);
    outb(0x21, 4);
    outb(0xA1, 2);
    outb(0x21, 1);
    outb(0xA1, 1);
    outb(0x21, 0xFF);
    outb(0xA1, 0xFF);
}

pub unsafe fn pic_irq_enable(which: u8) {
    if which >= 8 {
        outb(0x21, inb(0x21) & 0xFB);
        outb(0xA1, inb(0xA1) & !(1<<(which-8)));
    } else {
        outb(0x21, inb(0x21) & !(1<<which));
    }
}

pub unsafe fn pic_irq_disable(which: u8) {
    if which >= 8 {
        outb(0xA1, inb(0xA1) | (1<<(which-8)));
    } else {
        outb(0x21, inb(0x21) | (1<<which));
    }
}

pub unsafe fn pic_eoi(which: u8) {
    if which >= 8 {
        outb(0xA0, 0x20);
    }
    outb(0x20, 0x20);
}