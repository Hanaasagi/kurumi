// follow https://wiki.osdev.org/FAT#Directories
use super::super::File;
use alloc::string::String;

bitflags! {
    pub struct FileAttributes: u8 {
        const READONLY     = 0x01;
        const HIDDEN       = 0x02;
        const SYSTEM       = 0x04;
        const VOLUMEID     = 0x08;
        const DIRECTORY    = 0x10;
        const ARCHIVE      = 0x20;
        const LONGFILENAME = Self::READONLY.bits |
            Self::HIDDEN.bits | Self::SYSTEM.bits | Self::VOLUMEID.bits;
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

        let last_index = buff.len();
        use alloc::string::ToString;
        String::from_utf16_lossy(&buff[..last_index])
    }
}

// Standard 8.3 format
#[repr(packed, C)]
#[derive(Debug, Copy, Clone)]
pub struct FatDirectory {
    pub name:                  [u8; 11],
    attributes:            u8,
    reserved_nt:           u8,  // Reserved for use by Windows NT.
    creation_time_precise: u8,  // Creation time in tenths of a second
    creation_time:         u16, // The time that the file was created. Multiply Seconds by 2.(Hour 5 bits | Minutes 6 bits | Seconds 5 bits)
    creation_date:         u16, // The date on which the file was created.(Year 7 bits | Month 4 bits | Day 5 bits)
    last_accessed:         u16, // Last accessed date. Same format as the creation date.
    first_cluster_high:    u16, // The high 16 bits of this entry's first cluster number.
    last_modified_time:    u16, // Last modification time. Same format as the creation time.
    last_modified_date:    u16, // Last modification date. Same format as the creation date.
    first_cluster_low:     u16, // The low 16 bits of this entry's first cluster number.
    file_size:             u32,
}

impl FatDirectory {
    pub fn get_short_name(&self) -> String {
        use alloc::string::ToString;
        String::from_utf8(self.name.to_vec())
            .expect("Invalid UTF-8.").trim().to_string()
    }

    pub fn get_cluster(&self) -> u32 {
        (self.first_cluster_high as u32) << 16 | self.first_cluster_low as u32
    }

    // is long file name
    pub fn is_lfn(&self) -> bool {
        self.attributes as u8 == FileAttributes::LONGFILENAME.bits
    }

    pub fn is_folder(&self) -> bool {
        let flag = FileAttributes::from_bits_truncate(self.attributes);
        flag.contains(FileAttributes::DIRECTORY)
    }
}

#[derive(Debug, Clone)]
pub struct Directory {
    name: String,
    fat_directory: FatDirectory,
}

impl Directory {
    pub fn new(name: String, directory: FatDirectory) -> Self {
        Directory {
            name: name,
            fat_directory: directory,
        }
    }

    pub fn get_fat_dir(&self) -> &FatDirectory {
        &self.fat_directory
    }

    pub fn is_lfn(&self) -> bool {
        self.fat_directory.is_lfn()
    }

    pub fn get_name(&self) -> String {
        use alloc::string::ToString;
        self.name.to_string()
    }

    pub fn get_cluster(&self) -> u32 {
        self.fat_directory.get_cluster()
    }

    //pub fn is_folder(&self) -> bool {
        //let flag = FileAttributes::from_bits_truncate(self.fat_directory.attributes);
        //flag.contains(FileAttributes::Directory)
    //}
}

impl File for Directory {
    fn get_name(&self) -> String {
        use alloc::string::ToString;
        return self.name.to_string();
    }

    fn get_size(&self) -> usize {
        self.fat_directory.file_size as usize
    }
}
