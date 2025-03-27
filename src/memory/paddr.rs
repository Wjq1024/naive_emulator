use core::panic;

use crate::common::*;

use super::{MemoryAccessError, MemoryAccessOperation, MemoryManager, init::MEMORY_MANAGER};

impl MemoryManager {
    fn paddr_read(&self, paddr: PAddr, len: usize) -> Word {
        match (paddr.0, len) {
            (pa, 1 | 2 | 4) if pa % 4 == 0 => {
                let val = self
                    .haddr_read(paddr.into(), len)
                    .map_err(|x| MemoryAccessError::<PAddr> {
                        oper: x.oper,
                        addr: paddr,
                        len: x.len,
                    })
                    .unwrap();
                val.iter()
                    .enumerate()
                    .fold(0, |acc, (idx, val)| acc + ((*val as u32) << (8 * idx)))
            }
            _ => panic!(
                "{:?}",
                MemoryAccessError::<PAddr> {
                    oper: MemoryAccessOperation::Read,
                    addr: paddr,
                    len: len,
                }
            ),
        }
    }

    fn paddr_write(&mut self, paddr: PAddr, len: usize, data: Word) {
        match (paddr.0, len) {
            (pa, 1 | 2 | 4) if pa % 4 == 0 => {
                let mut data_bytes = [0_u8; 4];
                let _ = data_bytes.iter_mut().enumerate().map(|(idx, x)| {
                    *x = (data >> (idx * 8)) as u8;
                });
                self.haddr_write(paddr.into(), &data_bytes[0..len])
                    .map_err(|x| MemoryAccessError::<PAddr> {
                        oper: x.oper,
                        addr: paddr,
                        len: x.len,
                    })
                    .unwrap()
            }
            _ => panic!(
                "{:?}",
                MemoryAccessError::<PAddr> {
                    oper: MemoryAccessOperation::Write,
                    addr: paddr,
                    len,
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

mod tests {
    use crate::memory::{HMEM_SIZE, haddr::HMEM_OFFSET};

    use super::paddr_read;

    #[test]
    #[should_panic]
    fn test_check_bound1() {
        paddr_read((HMEM_OFFSET as u32).into(), 3);
    }

    #[test]
    #[should_panic]
    fn test_check_bound2() {
        paddr_read((HMEM_OFFSET as u32 + 1).into(), 1);
    }

    #[test]
    #[should_panic]
    fn test_check_bound3() {
        paddr_read((HMEM_OFFSET as u32 - 1).into(), 1);
    }

    #[test]
    #[should_panic]
    fn test_check_bound4() {
        paddr_read((HMEM_OFFSET as u32 + HMEM_SIZE as u32).into(), 1);
    }

    #[test]
    fn test_check_bound5() {
        paddr_read((HMEM_OFFSET as u32 + 20).into(), 2);
    }
}
