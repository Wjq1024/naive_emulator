use std::sync::LazyLock;

use super::{HMEM_SIZE, MemoryManager};
use crate::uniprocessor::UPSafeCell;

pub static MEMORY_MANAGER: LazyLock<UPSafeCell<MemoryManager>> =
    LazyLock::new(|| unsafe { UPSafeCell::new(MemoryManager { mem: init_mem() }) });

pub fn init_mem() -> [u8; HMEM_SIZE] {
    [0; HMEM_SIZE]
}
