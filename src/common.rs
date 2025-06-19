use std::{fmt::Display, sync::LazyLock};

use crate::uniprocessor::UPSafeCell;

pub type SWord = i32;
pub type Word = u32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PAddr(pub u32);

impl From<u32> for PAddr {
    fn from(value: u32) -> Self {
        PAddr(value)
    }
}

impl Display for PAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:08x}", self.0)
    }
}

pub const PC_ENTRY: PAddr = PAddr(0x80000000);
pub const PMEM_SIZE: usize = 64 * 1024 * 1024;

#[derive(PartialEq)]
pub enum EmulatorStatus {
    Idle,
    Running,
    Stop,
}

pub static EMULATOR_STATUS: LazyLock<UPSafeCell<EmulatorStatus>> =
    LazyLock::new(|| unsafe { UPSafeCell::new(EmulatorStatus::Idle) });
