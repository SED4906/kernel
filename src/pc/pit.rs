use x86::io::outb;

use crate::serial_println;

use super::pic::pic_clear_mask;

pub unsafe fn pit_init() {
    outb(0x43, 0x34);
    outb(0x40, 0x9C);
    outb(0x40, 0x2E);
    pic_clear_mask(0);
    serial_println!("Initialized PIT at 100Hz");
}