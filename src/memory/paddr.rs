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
                data_bytes.iter_mut().enumerate().for_each(|(idx, x)| {
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

    use super::{paddr_read, paddr_write};

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
        paddr_read((HMEM_OFFSET as u32 + HMEM_SIZE as u32 - 4).into(), 1);
    }

    #[test]
    fn test_paddr_read_and_write() {
        let w_before = paddr_read((0x8000_4000 - 4).into(), 4);
        let w_after = paddr_read((0x8000_4000 + 4).into(), 4);
        paddr_write(0x8000_4000.into(), 4, 0xAABBCCDD);
        assert_eq!(paddr_read(0x8000_4000.into(), 4), 0xAABBCCDD);
        assert_eq!(paddr_read(0x8000_4000.into(), 2), 0xCCDD);
        assert_eq!(paddr_read(0x8000_4000.into(), 1), 0xDD);
        assert_eq!(paddr_read((0x8000_4000 - 4).into(), 4), w_before);
        assert_eq!(paddr_read((0x8000_4000 + 4).into(), 4), w_after);
    }
}
