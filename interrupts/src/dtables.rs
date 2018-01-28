use core::mem::size_of;

use idt::IdtEntry;

#[repr(C, packed)]
pub struct DescriptorTablePointer {
    // Size of the DT.
    pub limit: u16,
    // Pointer to the memory region containing the IDT.
    pub base: *const IdtEntry,
}

impl DescriptorTablePointer {
    fn new(slice: &[IdtEntry]) -> Self {
        let len = slice.len() * size_of::<IdtEntry>();
        assert!(len < 0x10000);
        DescriptorTablePointer {
            base:  slice.as_ptr(),
            limit: len as u16,
        }
    }

    pub fn new_idtp(idt: &[IdtEntry]) -> Self {
        Self::new(idt)
    }
}

// Load IDT table.
pub unsafe fn lidt(idt: &DescriptorTablePointer) {
    asm!("lidt ($0)" :: "r" (idt) : "memory");
}
