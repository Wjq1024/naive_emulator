use crate::common::{PAddr, PMEM_SIZE, Word};

use super::{MemoryAccessError, MemoryAccessOperation, MemoryManager};

#[derive(Clone, Copy)]
pub(super) struct HAddr(pub usize);

pub(super) const HMEM_OFFSET: usize = 0x80000000;
pub(super) const HMEM_SIZE: usize = PMEM_SIZE;

impl From<PAddr> for HAddr {
    fn from(value: PAddr) -> Self {
        HAddr(value.0 as usize - HMEM_OFFSET)
    }
}

pub(super) fn check_host_addr(haddr: HAddr, len: usize) -> bool {
    haddr.0 < HMEM_SIZE && haddr.0 + len < HMEM_SIZE && haddr.0 + len > haddr.0
}

impl MemoryManager {
    pub(super) fn haddr_read(
        &self,
        haddr: HAddr,
        len: usize,
    ) -> Result<Word, MemoryAccessError<HAddr>> {
        if !check_host_addr(haddr, len) {
            return Err(MemoryAccessError {
                oper: MemoryAccessOperation::READ,
                addr: haddr,
                len,
                data: None,
            });
        }
        match (haddr.0, len) {
            (l, 1) => Ok(self.mem[l] as u32),
            (l, 2) => Ok(self.mem[l] as u32 + (self.mem[l + 1] as u32) << 8),
            (l, 4) => Ok(self.mem[l] as u32 + (self.mem[l + 1] as u32)
                << 8 + (self.mem[l + 2] as u32)
                << 16 + (self.mem[l + 3] as u32)
                << 24),
            _ => Err(MemoryAccessError {
                oper: MemoryAccessOperation::READ,
                addr: haddr,
                len,
                data: None,
            }),
        }
    }

    pub(super) fn haddr_write(
        &mut self,
        haddr: HAddr,
        len: usize,
        data: Word,
    ) -> Result<(), MemoryAccessError<HAddr>> {
        if !check_host_addr(haddr, len) {
            return Err(MemoryAccessError {
                oper: MemoryAccessOperation::WRITE,
                addr: haddr,
                len,
                data: Some(data),
            });
        }
        match (haddr.0, len) {
            (l, 1) => Ok(self.mem[l] = (data & ((1 << 8) - 1)) as u8),
            (l, 2) => Ok({
                self.mem[l] = data as u8;
                self.mem[l + 1] = (data >> 8) as u8;
            }),
            (l, 4) => Ok({
                self.mem[l] = data as u8;
                self.mem[l + 1] = (data >> 8) as u8;
                self.mem[l + 2] = (data >> 16) as u8;
                self.mem[l + 3] = (data >> 24) as u8;
            }),
            _ => Err(MemoryAccessError {
                oper: MemoryAccessOperation::WRITE,
                addr: haddr,
                len,
                data: Some(data),
            }),
        }
    }
}
