// follow https://wiki.osdev.org/FAT#Directories

bitflags! {
    pub struct FileAttributes: u8 {
        const ReadOnly  = 0x01,
        const Hidden    = 0x02,
        const System    = 0x04,
        const VolumeId  = 0x08,
        const Directory = 0x10,
        const Archive   = 0x20,
        const LongFileName= Self::ReadOnly |
            Self::Hidden | Self::System | Self::VolumeId,
    }
}

#[repr(packed, C)]
#[derive(Debug, Copy, Clone)]
pub struct LongFileName {
    order:           u8,       // The order of this entry in the sequence of long file name entries
    name_first:      [u16; 5], // The first 5, 2-byte characters of this entry
    attributes:      u8,       // Attribute. Always equals 0x0F. (the long file name attribute)
    long_entry_type: u8,       // Long entry type. Zero for name entries.
    checksum:        u8,
    name_middle:     [u16; 6],
    reserved:        u16,      // always zero
    name_final:      [u16; 2],
}

impl LongFileName {
    pub fn get_name(&self) -> String {
        // long file name total 13 bytes
        let mut buff = vec![0u16; 13];
        buff[..5].clone_from_slice(&self.name_first);
        buff[5..11].clone_from_slice(&self.name_middle);
        buff[11..].clone_from_slice(&self.name_final);

        let mut last_index = buff.len();
        use alloc::string::ToString;
        return String::from_utf16_lossy(&buff[..last_index]);
    }
}
