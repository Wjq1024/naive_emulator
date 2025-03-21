use core::panic;

use crate::common::*;

use super::{MemoryManager, init::MEMORY_MANAGER};

impl MemoryManager {
    fn paddr_to_host_index(paddr: PAddr) -> usize {
        paddr as usize - MEM_OFFSET
    }

    fn check_host_addr(host_index: usize, len: usize) {
        if host_index + len > PMEM_SIZE {
            panic!("host_addr out of bounds, host_addr: {host_index}, len: {len}")
        }
    }

    fn host_read(&self, host_index: usize, len: usize) -> Word {
        Self::check_host_addr(host_index, len);
        match (host_index, len) {
            (l, 1) => self.mem[l] as u32,
            (l, 2) => self.mem[l] as u32 + (self.mem[l + 1] as u32) << 8,
            (l, 4) => {
                self.mem[l] as u32 + (self.mem[l + 1] as u32)
                    << 8 + (self.mem[l + 2] as u32)
                    << 16 + (self.mem[l + 3] as u32)
                    << 24
            }
            (_, _) => panic!("invalid len, host_addr: {host_index}, len: {len}"),
        }
    }

    fn host_write(&mut self, host_index: usize, len: usize, data: Word) {
        Self::check_host_addr(host_index, len);
        match (host_index, len) {
            (l, 1) => self.mem[l] = (data & ((1 << 8) - 1)) as u8,
            (l, 2) => {
                self.mem[l] = data as u8;
                self.mem[l + 1] = (data >> 8) as u8;
            }
            (l, 4) => {
                self.mem[l] = data as u8;
                self.mem[l + 1] = (data >> 8) as u8;
                self.mem[l + 2] = (data >> 16) as u8;
                self.mem[l + 3] = (data >> 24) as u8;
            }
            (_, _) => panic!("invalid len, host_addr: {host_index}, len: {len}, data: {data}"),
        }
    }
}

pub fn paddr_read(paddr: PAddr, len: usize) -> Word {
    MEMORY_MANAGER
        .exclusive_access()
        .host_read(MemoryManager::paddr_to_host_index(paddr), len)
}

pub fn paddr_write(paddr: PAddr, len: usize, data: Word) {
    MEMORY_MANAGER.exclusive_access().host_write(
        MemoryManager::paddr_to_host_index(paddr),
        len,
        data,
    );
}
