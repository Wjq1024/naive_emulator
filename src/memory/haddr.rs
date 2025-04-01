use crate::common::{PAddr, PMEM_SIZE};

use super::{MemoryAccessError, MemoryAccessOperation, MemoryManager};

#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) struct HAddr(pub usize);

pub(super) const HMEM_OFFSET: usize = 0x80000000;
pub(super) const HMEM_SIZE: usize = PMEM_SIZE;

impl From<PAddr> for HAddr {
    fn from(value: PAddr) -> Self {
        let offset = value.0 as usize;
        if offset < HMEM_OFFSET || offset >= HMEM_OFFSET + HMEM_SIZE {
            panic!(
                "Invalid physical address: 0x{:X} (must be in range [0x{:X}, 0x{:X}))",
                offset,
                HMEM_OFFSET,
                HMEM_OFFSET + HMEM_SIZE
            );
        }
        HAddr(offset - HMEM_OFFSET)
    }
}


impl From<usize> for HAddr {
    fn from(value: usize) -> Self {
        HAddr(value)
    }
}

pub fn check_host_addr(haddr: HAddr, len: usize) -> bool {
    haddr.0 < HMEM_SIZE
        && haddr.0.wrapping_add(len) <= HMEM_SIZE
        && haddr.0.wrapping_add(len) > haddr.0
}

impl MemoryManager {
    pub(super) fn haddr_read(
        &self,
        haddr: HAddr,
        len: usize,
    ) -> Result<&[u8], MemoryAccessError<HAddr>> {
        match check_host_addr(haddr, len) {
            true => Ok(&self.mem[haddr.0..haddr.0 + len]),
            false => Err(MemoryAccessError {
                oper: MemoryAccessOperation::Read,
                addr: haddr,
                len,
            }),
        }
    }

    pub(super) fn haddr_write(
        &mut self,
        haddr: HAddr,
        data: &[u8],
    ) -> Result<(), MemoryAccessError<HAddr>> {
        let len = data.len();
        match check_host_addr(haddr, len) {
            true => {
                self.mem[haddr.0..haddr.0 + len].copy_from_slice(data);
                Ok(())
            }
            false => Err(MemoryAccessError {
                oper: MemoryAccessOperation::Write,
                addr: haddr,
                len,
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
        assert_eq!(inner.haddr_write(HAddr(0), [0; 4].as_slice()), Ok(()));
        assert_eq!(inner.haddr_read(HAddr(0), 1), Ok([0].as_slice()));
        assert_eq!(inner.haddr_read(HAddr(0), 4), Ok([0; 4].as_slice()));
        assert_eq!(
            inner.haddr_write(HAddr(HMEM_SIZE - 4), [0; 4].as_slice()),
            Ok(())
        );
        assert_eq!(
            inner.haddr_read(HAddr(HMEM_SIZE - 4), 4),
            Ok([0; 4].as_slice())
        );
        assert_eq!(inner.haddr_write(0.into(), [0xFF].as_slice()), Ok(()));
        assert_eq!(inner.haddr_read(0.into(), 1), Ok([0xFF].as_slice()));
        assert_eq!(inner.haddr_write(HAddr(0), [0xFF; 2].as_slice()), Ok(()));
        assert_eq!(inner.haddr_write(HAddr(2), [0x00; 2].as_slice()), Ok(()));
        assert_eq!(inner.haddr_read(HAddr(0), 1), Ok([0xFF].as_slice()));
        assert_eq!(inner.haddr_read(HAddr(1), 1), Ok([0xFF].as_slice()));
        assert_eq!(inner.haddr_read(HAddr(1), 2), Ok([0xFF, 0x00].as_slice()));
        assert_eq!(
            inner.haddr_read(HAddr(1), 4),
            Ok([0xFF, 0x00, 0x00, 0x00].as_slice())
        );
        assert_eq!(inner.haddr_write(HAddr(4), [0xFF; 4].as_slice()), Ok(()));
        assert_eq!(inner.haddr_read(HAddr(4), 1), Ok([0xFF].as_slice()));
        assert_eq!(inner.haddr_read(HAddr(5), 1), Ok([0xFF].as_slice()));
        assert_eq!(inner.haddr_read(HAddr(4), 2), Ok([0xFF; 2].as_slice()));
        assert_eq!(inner.haddr_read(HAddr(4), 4), Ok([0xFF; 4].as_slice()));
        assert_eq!(inner.haddr_read(HAddr(3), 2), Ok([0x00, 0xFF].as_slice()));
    }
}
