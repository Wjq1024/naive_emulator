use haddr::HMEM_SIZE;

mod haddr;
mod init;
mod paddr;

pub use paddr::{paddr_read, paddr_write};

pub struct MemoryManager {
    mem: [u8; HMEM_SIZE],
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
