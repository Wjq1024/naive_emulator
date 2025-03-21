pub type SWord = i32;
pub type Word = u32;
pub type PAddr = u32;

pub const PC_ENTRY: PAddr = 0x80000000;
pub const MEM_OFFSET: usize = 0x80000000;
pub const PMEM_SIZE: usize = 64 * 1024 * 1024;
