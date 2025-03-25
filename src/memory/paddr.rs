use crate::common::*;

use super::{
    HAddr, MemoryAccessError, MemoryAccessOperation, MemoryManager, check_host_addr,
    init::MEMORY_MANAGER,
};

impl MemoryManager {
    fn host_read(&self, haddr: HAddr, len: usize) -> Result<Word, MemoryAccessError<HAddr>> {
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

    fn host_write(
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

pub fn paddr_read(paddr: PAddr, len: usize) -> Word {
    MEMORY_MANAGER
        .exclusive_access()
        .host_read(paddr.into(), len)
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

pub fn paddr_write(paddr: PAddr, len: usize, data: Word) {
    MEMORY_MANAGER
        .exclusive_access()
        .host_write(paddr.into(), len, data)
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
