use crate::common::*;

use super::MemoryManager;

impl MemoryManager {
    fn paddr_to_host_index(paddr: PAddr) -> usize {
        paddr as usize - MEM_OFFSET
    }

    fn host_read(&self, host_index: usize, len: usize) -> Word {
        unimplemented!()
    }

    fn host_write(&mut self, host_index: usize, len: usize) {
        unimplemented!()
    }
}

pub fn paddr_read(paddr: PAddr, len: usize) -> Word {
    unimplemented!()
}

pub fn paddr_write(paddr: PAddr, len: usize) {
    unimplemented!()
}
