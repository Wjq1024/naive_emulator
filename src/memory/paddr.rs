use core::panic;

use crate::common::*;

use super::{MemoryAccessError, MemoryAccessOperation, MemoryManager, init::MEMORY_MANAGER};

impl MemoryManager {
    fn paddr_read(&self, paddr: PAddr, len: usize) -> Word {
        match len {
            1 | 2 | 4 => {
                let val = self
                    .haddr_read(paddr.into(), len)
                    .or_else(|x| {
                        Err(MemoryAccessError::<PAddr> {
                            oper: x.oper,
                            addr: paddr,
                            len: x.len,
                        })
                    })
                    .unwrap();
                val.iter()
                    .enumerate()
                    .fold(0, |acc, (idx, val)| acc + ((*val as u32) << (8 * idx)))
            }
            _ => panic!(
                "{:?}",
                MemoryAccessError::<PAddr> {
                    oper: MemoryAccessOperation::READ,
                    addr: paddr,
                    len: len,
                }
            ),
        }
    }

    fn paddr_write(&mut self, paddr: PAddr, len: usize, data: Word) {
        match len {
            1 | 2 | 4 => {
                let mut data_bytes = [0 as u8; 4];
                let _ = data_bytes.iter_mut().enumerate().map(|(idx, x)| {
                    *x = (data >> (idx * 8)) as u8;
                });
                self.haddr_write(paddr.into(), &data_bytes[0..len])
                    .or_else(|x| {
                        Err(MemoryAccessError::<PAddr> {
                            oper: x.oper,
                            addr: paddr,
                            len: x.len,
                        })
                    })
                    .unwrap()
            }
            _ => panic!(
                "{:?}",
                MemoryAccessError::<PAddr> {
                    oper: MemoryAccessOperation::WRITE,
                    addr: paddr,
                    len: len,
                }
            ),
        }
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

mod tests {}
