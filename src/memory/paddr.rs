use crate::common::*;

use super::{MemoryAccessError, MemoryManager, init::MEMORY_MANAGER};
impl MemoryManager {
    pub(super) fn paddr_read(&self, paddr: PAddr, len: usize) -> Word {
        self.haddr_read(paddr.into(), len)
            .or_else(|x| {
                Err(MemoryAccessError::<PAddr> {
                    oper: x.oper,
                    addr: paddr,
                    len: x.len,
                    data: x.data,
                })
            })
            .unwrap()
    }

    pub(super) fn paddr_write(&mut self, paddr: PAddr, len: usize, data: Word) {
        self.haddr_write(paddr.into(), len, data)
            .or_else(|x| {
                Err(MemoryAccessError::<PAddr> {
                    oper: x.oper,
                    addr: paddr,
                    len: x.len,
                    data: x.data,
                })
            })
            .unwrap()
    }
}

pub fn paddr_read(paddr: PAddr, len: usize) -> Word {
    MEMORY_MANAGER.exclusive_access().paddr_read(paddr, len)
}

pub fn paddr_write(paddr: PAddr, len: usize, data: Word) {
    MEMORY_MANAGER
        .exclusive_access()
        .paddr_write(paddr, len, data)
}
