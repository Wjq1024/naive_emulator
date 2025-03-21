use std::sync::LazyLock;

use super::MemoryManager;
use crate::{common::PMEM_SIZE, uniprocessor::UPSafeCell};

pub static MEMORY_MANAGER: LazyLock<UPSafeCell<MemoryManager>> =
    LazyLock::new(|| unsafe { UPSafeCell::new(MemoryManager { mem: init_mem() }) });

pub fn init_mem() -> [u8; PMEM_SIZE] {
    [0; PMEM_SIZE]
}
