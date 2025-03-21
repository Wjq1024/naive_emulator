use std::sync::LazyLock;

use super::{Cpu, GPR_SIZE};
use crate::common::*;

pub static CPU: LazyLock<Cpu> = LazyLock::new(|| Cpu {
    gpr: [0; GPR_SIZE],
    pc: PC_ENTRY,
});
