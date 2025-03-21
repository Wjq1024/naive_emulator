use crate::common::*;

mod init;
mod paddr;

pub struct MemoryManager {
    mem: [u8; PMEM_SIZE],
}

unsafe impl Sync for MemoryManager {}
