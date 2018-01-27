// Write 8 bits to port
pub unsafe fn outb(port: u16, val: u8) {
    asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(val));
}

// Read 8 bits from port
pub unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    asm!("inb %dx, %al" : "={ax}"(ret) : "{dx}"(port) :: "volatile");
    ret
}

// Forces the CPU to wait for an I/O operation to complete.
// only use this when there's nothing like
// a status register or an IRQ to tell you the info has been received.
pub unsafe fn io_wait() {
    asm!("jmp 1f;1:jmp 2f;2:" :::: "volatile");
}
