#![allow(dead_code)]

use io::{inb, outb, io_wait};

// reinitialize the PIC controllers, giving them specified vector offsets
// rather than 8h and 70h, as configured by default
//
const  PIC1: u16         = 0x20;          // IO base address for master PIC
const  PIC2: u16         = 0xA0;          // IO base address for slave PIC
const  PIC1_COMMAND: u16 = PIC1;
const  PIC1_DATA: u16    = (PIC1+1);
const  PIC2_COMMAND: u16 = PIC2;
const  PIC2_DATA: u16    = (PIC2+1);

const ICW1_ICW4: u8       = 0x01;       // ICW4 (not) needed
const ICW1_SINGL: u8      = 0x02;       // Single (cascade) mode
const ICW1_INTERVAL4: u8  = 0x04;       // Call address interval 4 (8)
const ICW1_LEVEL: u8      = 0x08;       // Level triggered (edge) mode
const ICW1_INIT: u8       = 0x10;       // Initialization - required!

const ICW4_8086: u8       = 0x01;       // 8086/88 (MCS-80/85) mode
const ICW4_AUTO: u8       = 0x02;       // Auto (normal) EOI
const ICW4_BUF_SLAVE: u8  = 0x08;       // Buffered mode/slave
const ICW4_BUF_MASTER: u8 = 0x0C;       // Buffered mode/master
const ICW4_SFNM: u8       = 0x10;       // Special fully nested (not)

// arguments:
//     offset1 - vector offset for master PIC
//               vectors on the master become offset1..offset1+7
//     offset2 - same for slave PIC: offset2..offset2+7

pub fn remap() {
    unsafe {
        // save masks
        let a1 = inb(PIC1_DATA);
        let a2 = inb(PIC2_DATA);

        let offset1: u8 = 0x20;
        let offset2: u8 = 0x28;
        outb(PIC1_COMMAND, ICW1_INIT+ICW1_ICW4);  // starts the initialization sequence (in cascade mode)
        io_wait();
        outb(PIC2_COMMAND, ICW1_INIT+ICW1_ICW4);
        io_wait();
        outb(PIC1_DATA, offset1);                 // ICW2: Master PIC vector offset
        io_wait();
        outb(PIC2_DATA, offset2);                 // ICW2: Slave PIC vector offset
        io_wait();
        outb(PIC1_DATA, 4);                       // ICW3: tell Master PIC that there is a slave PIC at IRQ2 (0000 0100)
        io_wait();
        outb(PIC2_DATA, 2);                       // ICW3: tell Slave PIC its cascade identity (0000 0010)
        io_wait();

        // set both PICs to 8086 mode
        outb(PIC1_DATA, ICW4_8086);
        io_wait();
        outb(PIC2_DATA, ICW4_8086);
        io_wait();

        // restore saved masks.
        outb(PIC1_DATA, a1);
        outb(PIC2_DATA, a2);
    }
}

// End-of-interrupt command code
const PIC_EOI: u8 = 0x20;

pub fn send_eoi(interrupt_number: isize) {
    // TODO
    if interrupt_number >= 8 {
        unsafe { outb(PIC2_COMMAND,PIC_EOI); }
    }
    unsafe {outb(PIC1_COMMAND,PIC_EOI); }
}
