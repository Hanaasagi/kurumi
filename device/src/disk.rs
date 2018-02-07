pub trait Disk {
    unsafe fn read(&self, block: u64, buffer: &mut [u8]) -> Result<u8, &str>;
    unsafe fn write_at(&self, block: u64, buffer: &[u8]) -> Result<u8, &str>;
}
