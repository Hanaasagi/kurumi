// follow https://wiki.osdev.org/ATA_PIO_Mode
use core::slice;
use io::{inb, outb, inw};

// An ATA bus typically has 9 I/O ports that control its behavior.
// For the primary bus, these I/O ports are 0x1F0 through 0x1F7, and 0x3F6.
// The values in this table are relative to the so-called I/O port base address.
// So a port value of 1 actually means 0x1F0 + 1 = 0x1F1.
// This is done because the base address may vary depending on the hardware.
//
// Port Offset 	Function 	                           Description
// 0 	        Data Port 	                           Read/Write PIO data bytes on this port.
// 1 	        Features / Error Information 	       Usually used for ATAPI devices.
// 2 	        Sector Count 	                       Number of sectors to read/write (0 is a special value).
// 3 	        Sector Number / LBAlo 	               This is CHS / LBA28 / LBA48 specific.
// 4 	        Cylinder Low / LBAmid 	               Partial Disk Sector address.
// 5 	        Cylinder High / LBAhi 	               Partial Disk Sector address.
// 6 	        Drive / Head Port 	                   Used to select a drive and/or head.
// 7 	        Command port / Regular Status port 	   Used to send commands or read the current status.
bitflags! {
    struct ATA_Bus: u16 {
        const data_port    = 0x1F0;
        const error_info   = 0x1F1;
        const sector_count = 0x1F2;
        const lba_low      = 0x1F3;
        const lba_mid      = 0x1F4;
        const lba_high     = 0x1F5;
        const drive        = 0x1F6;
        const command      = 0x1F7;
        const status       = 0x3F6;
    }
}

const READ_SECTORS: u8 = 0x20;

pub struct Ata {}

impl Ata {

    unsafe fn poll<F>(condition: F) -> u8 where F: Fn(u8) -> bool {
        let mut reg_value: u8;
        loop {
            reg_value = inb(ATA_Bus::status.bits);
            if condition(reg_value) {
                return reg_value;
            }
        }
    }

    // Send 0xE0 for the "master" or 0xF0 for the "slave", ORed with the highest 4 bits of the LBA to port 0x1F6: outb(0x1F6, 0xE0 | (slavebit << 4) | ((LBA >> 24) & 0x0F))
    // Send a NULL byte to port 0x1F1, if you like (it is ignored and wastes lots of CPU time): outb(0x1F1, 0x00)
    // Send the sectorcount to port 0x1F2: outb(0x1F2, (unsigned char) count)
    // Send the low 8 bits of the LBA to port 0x1F3: outb(0x1F3, (unsigned char) LBA))
    // Send the next 8 bits of the LBA to port 0x1F4: outb(0x1F4, (unsigned char)(LBA >> 8))
    // Send the next 8 bits of the LBA to port 0x1F5: outb(0x1F5, (unsigned char)(LBA >> 16))
    // Send the "READ SECTORS" command (0x20) to port 0x1F7: outb(0x1F7, 0x20)
    // Wait for an IRQ or poll.
    // Transfer 256 16-bit values, a uint16_t at a time, into your buffer from I/O port 0x1F0. (In assembler, REP INSW works well for this.)
    // Then loop back to waiting for the next IRQ (or poll again -- see next note) for each successive sector.
    unsafe fn read(block: u64, buffer: &mut [u8]) -> Result<u8, &str> {
        // check
        if buffer.len() == 0 {
            return Err("Size of buffer can't be 0.");
        } else if buffer.len() % 512 != 0 {
            return Err("Buffer size must be a multiplication of sector size.");
        } else if buffer.len() / 512 > 127 {
            return Err("Can only read 127 sectors at a time in LBA28 mode.");
        }

        let sector_count = (buffer.len() / 512) as u8;
        let command: u8 = 0xE0 | ((block >> 24) & 0x0F) as u8 | (0x40) as u8; // bit 6 enabled for 28 bit LBA mode.
        outb(ATA_Bus::drive.bits, command);
        outb(ATA_Bus::sector_count.bits, sector_count) ;
        outb(ATA_Bus::lba_low.bits, block as u8);
        outb(ATA_Bus::lba_mid.bits, (block >> 8)  as u8);
        outb(ATA_Bus::lba_low.bits, (block >> 16) as u8);
        outb(ATA_Bus::command.bits, READ_SECTORS);

        for sector in 0..sector_count {
            // poll
            let status = Self::poll(
                |x| (x & 0x80 == 0 && x & 0x8 != 0) || x & 0x1 != 0 || x & 0x20 != 0
            );

            if status & 1 != 0 {
                if sector == 0 {
                    return Err("No sectors read.");
                }
                // return amount of read sectors
                return Ok(sector);
            } else if status & 0x20 != 0 {
                return Err("Drive Fault occured.");
            }

            // Read data to buffer
            let buff = slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u16, buffer.len()/2);
            for i in 0..buff.len() {
                buff[i+(sector as usize*256)] = inw(ATA_Bus::data_port.bits);
            }

            // After transferring the last uint16_t of a PIO data block to the data IO port,
            // give the drive a 400ns delay to reset its DRQ bit
            for _ in 0..4 {
                inb(ATA_Bus::status.bits);
            }
        }
        // return the amount of sectors read
        Ok(sector_count)
    }

    unsafe fn write_at(&self, block: u64, buffer: &[u8]) -> Result<u8, &str> {
        unimplemented!();
    }
}
