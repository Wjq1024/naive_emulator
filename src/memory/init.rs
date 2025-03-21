use std::sync::LazyLock;

use super::MemoryManager;
use crate::common::PMEM_SIZE;

pub static MEMORY_MANAGER: LazyLock<MemoryManager> =
    LazyLock::new(|| MemoryManager { mem: init_mem() });

pub fn init_mem() -> [u8; PMEM_SIZE] {
    [0; PMEM_SIZE]
}
