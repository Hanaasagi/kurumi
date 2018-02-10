// BIOS Parameter Block see https://wiki.osdev.org/FAT#Boot_Record
#[derive(Debug, Clone, Copy)]
#[repr(packed, C)]
pub struct Bpb {
    pub bootjmp:                [u8; 3],
    pub oem_identifier:         [u8; 8],
    pub bytes_per_sector:       u16,
    pub sectors_per_cluster:    u8,
    pub reserved_sectors_count: u16,
    pub table_count:            u8,
    pub root_entry_count:       u16,
    pub total_sectors:          u16,
    pub media_descriptor_type:  u8,
    pub sectors_per_fat:        u16,
    pub sectors_per_track:      u16,
    pub head_size_count:        u16,
    pub hidden_sectors_count:   u32,
    pub total_sectors_large:    u32,
}

// Extended Boot Record
#[derive(Debug, Clone, Copy)]
#[repr(packed, C)]
pub struct Ebpb {
    pub bpb:               Bpb,
    pub sectors_per_fat:   u32,
    pub flags:             u16,
    pub version_number:    u16,
    pub root_dir_cluster:  u32,
    pub fsinfo_sector:     u16,
    pub backup_mbr_sector: u16,
    pub reserved:          [u8; 12],
    pub drive_number:      u8,
    pub flags_nt:          u8,
    pub signature:         u8,
    pub volume_id:         u32,
    pub volume_label:      [u8; 11],
    pub system_identifier: [u8; 8]
}
