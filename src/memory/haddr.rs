use crate::common::{PAddr, PMEM_SIZE, Word};

use super::{MemoryAccessError, MemoryAccessOperation, MemoryManager};

#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) struct HAddr(pub usize);

pub(super) const HMEM_OFFSET: usize = 0x80000000;
pub(super) const HMEM_SIZE: usize = PMEM_SIZE;

impl From<PAddr> for HAddr {
    fn from(value: PAddr) -> Self {
        HAddr(value.0 as usize - HMEM_OFFSET)
    }
}

pub(super) fn check_host_addr(haddr: HAddr, len: usize) -> bool {
    haddr.0 < HMEM_SIZE
        && haddr.0.wrapping_add(len) <= HMEM_SIZE
        && haddr.0.wrapping_add(len) > haddr.0
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
            (l, 2) => Ok(self.mem[l] as u32 + ((self.mem[l + 1] as u32) << 8)),
            (l, 4) => Ok(self.mem[l] as u32
                + ((self.mem[l + 1] as u32) << 8)
                + ((self.mem[l + 2] as u32) << 16)
                + ((self.mem[l + 3] as u32) << 24)),
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
            (l, 1) => Ok(self.mem[l] = data as u8),
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

#[cfg(test)]
mod tests {
    use std::usize;

    use crate::memory::init::MEMORY_MANAGER;

    use super::*;

    #[test]
    fn test_check_host_addr() {
        assert!(check_host_addr(HAddr(0), 1));
        assert!(check_host_addr(HAddr(0), 2));
        assert!(check_host_addr(HAddr(0), 4));
        assert!(check_host_addr(HAddr(12), 4));
        assert!(!check_host_addr(HAddr(HMEM_SIZE), 1));
        assert!(!check_host_addr(HAddr(HMEM_SIZE), 2));
        assert!(!check_host_addr(HAddr(HMEM_SIZE + 11), 1));
        assert!(check_host_addr(HAddr(HMEM_SIZE - 1), 1));
        assert!(!check_host_addr(HAddr(HMEM_SIZE - 1), 2));
        assert!(check_host_addr(HAddr(HMEM_SIZE - 4), 4));
        assert!(!check_host_addr(HAddr(HMEM_SIZE - 3), 4));
        assert!(!check_host_addr(HAddr(HMEM_SIZE - 4), usize::MAX));
    }

    #[test]
    fn test_haddr_read_and_write() {
        let mut inner = MEMORY_MANAGER.exclusive_access();
        assert_eq!(inner.haddr_read(HAddr(0), 1), Ok(0));
        assert_eq!(inner.haddr_read(HAddr(0), 4), Ok(0));
        assert_eq!(inner.haddr_read(HAddr(HMEM_SIZE - 4), 4), Ok(0));
        assert_eq!(inner.haddr_write(HAddr(0), 1, 0xFF), Ok(()));
        assert_eq!(inner.haddr_read(HAddr(0), 1), Ok(0xFF));
        assert_eq!(inner.haddr_write(HAddr(0), 1, 0xFFFF), Ok(()));
        assert_eq!(inner.haddr_read(HAddr(0), 1), Ok(0xFF));
        assert_eq!(inner.haddr_write(HAddr(4), 4, 0xFFFFFFFF), Ok(()));
        assert_eq!(inner.haddr_read(HAddr(4), 1), Ok(0xFF));
        assert_eq!(inner.haddr_read(HAddr(5), 1), Ok(0xFF));
        assert_eq!(inner.haddr_read(HAddr(4), 2), Ok(0xFFFF));
        assert_eq!(inner.haddr_read(HAddr(4), 4), Ok(0xFFFFFFFF));
    }
}
