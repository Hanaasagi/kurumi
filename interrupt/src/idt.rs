bitflags! {
    pub struct IdtFlags: u8 {
        const PRESENT   = 1 << 7;
        const RING_0    = 0 << 5;
        const RING_1    = 1 << 5;
        const RING_2    = 2 << 5;
        const RING_3    = 3 << 5;
        const SS        = 1 << 4;
        const INTERRUPT = 0xE;
        const TRAP      = 0xF;
    }
}

// IdtEntry https://wiki.osdev.org/Interrupt_Descriptor_Table#Structure_AMD6
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IdtEntry {
    offsetl:  u16,   // offset bits 0..15
    selector: u16,   // a code segment selector in GDT or LDT
    ist:      u8,    // bits 0..2 holds Interrupt Stack Table offset, rest of bits zero.
    flags:    u8,    // type and attributes
    offsetm:  u16,   // offset bits 16..31
    offseth:  u32,   // offset bits 32..63
    zero:     u32,   // reserved
}


impl IdtEntry {

    pub const fn new() -> IdtEntry {
        IdtEntry {
            offsetl:   0,
            selector:  0,
            ist:      0,
            flags: 0,
            offsetm:   0,
            offseth:   0,
            zero:     0
        }
    }

    pub fn set_flags(&mut self, flags: IdtFlags) {
        self.flags = flags.bits;
    }

    pub fn set_offset(&mut self, selector: u16, base: usize) {
        self.selector = selector;
        self.offsetl = base as u16;
        self.offsetm = (base >> 16) as u16;
        self.offseth = (base >> 32) as u32;
    }

    pub fn set_func(&mut self, func: unsafe extern fn()) {
        self.set_flags(IdtFlags::PRESENT | IdtFlags::RING_0 | IdtFlags::INTERRUPT);
        self.set_offset(0x08, func as usize);
    }
}
