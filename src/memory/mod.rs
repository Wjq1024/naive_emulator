use haddr::HMEM_SIZE;

mod haddr;
mod init;
mod paddr;

pub use paddr::{paddr_read, paddr_write};

pub struct MemoryManager {
    mem: Vec<u8>,
}

#[derive(Debug, PartialEq)]
enum MemoryAccessOperation {
    Read,
    Write,
}

#[derive(Debug, PartialEq)]
struct MemoryAccessError<T> {
    oper: MemoryAccessOperation,
    addr: T,
    len: usize,
}
