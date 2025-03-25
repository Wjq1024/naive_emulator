use crate::common::{PAddr, PMEM_SIZE};

mod init;
mod paddr;

pub struct MemoryManager {
    mem: [u8; HMEM_SIZE],
}

unsafe impl Sync for MemoryManager {}

#[derive(Clone, Copy)]
struct HAddr(pub usize);

const HMEM_OFFSET: usize = 0x80000000;
const HMEM_SIZE: usize = PMEM_SIZE;

impl From<PAddr> for HAddr {
    fn from(value: PAddr) -> Self {
        HAddr(value.0 as usize - HMEM_OFFSET)
    }
}

fn check_host_addr(haddr: HAddr, len: usize) -> bool {
    haddr.0 < HMEM_SIZE && haddr.0 + len < HMEM_SIZE && haddr.0 + len > haddr.0
}

#[derive(Debug)]
enum MemoryAccessOperation {
    READ,
    WRITE,
}

#[derive(Debug)]
struct MemoryAccessError<T> {
    oper: MemoryAccessOperation,
    addr: T,
    len: usize,
    data: Option<u32>,
}
