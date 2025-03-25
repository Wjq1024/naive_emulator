pub type SWord = i32;
pub type Word = u32;

#[derive(Clone, Copy, Debug)]
pub struct PAddr(pub u32);

pub const PC_ENTRY: PAddr = PAddr(0x80000000);
pub const PMEM_SIZE: usize = 64 * 1024 * 1024;
