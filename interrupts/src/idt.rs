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
    flags:    u8, // type and attributes
    offsetm:  u16,   // offset bits 16..31
    offseth:  u32,   // offset bits 32..63
    zero:     u32,   // reserved
}


impl IdtEntry {

    // A "missing" IdtEntry.
    //
    // If the CPU tries to invoke a missing interrupt, it will instead
    // send a General Protection fault (13), with the interrupt number and
    // some other data stored in the error code.
    pub const MISSING: IdtEntry = IdtEntry {
        offsetl:  0,
        selector: 0,
        ist:      0,
        flags:    0,
        offsetm:  0,
        offseth:  0,
        zero:     0,
    };

    // Create a new IdtEntry pointing at `handler`, which must be a function
    // with interrupt calling conventions.  (This must be currently defined in
    // assembly language.)  The `gdt_code_selector` value must be the offset of
    // code segment entry in the GDT.
    //
    // The "Present" flag set, which is the most common case.  If you need
    // something else, you can construct it manually.
    pub fn new(handler: usize, gdt_code_selector: u16) -> IdtEntry {
        IdtEntry {
            selector: gdt_code_selector,
            offsetl:  handler as u16,
            offsetm:  (handler >> 16) as u16,
            offseth:  (handler >> 32) as u32,
            ist:      0,
            // Nice bitflags operations don't work in const fn, hence these
            // ad-hoc methods.
            //
            flags: (IdtFlags::PRESENT | IdtFlags::RING_0 | IdtFlags::INTERRUPT).bits,
            zero:     0,
        }
    }
}
