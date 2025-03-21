use crate::common::Word;

use super::signal::SignalControll;
use std::sync::LazyLock;

pub struct Instruction {
    pattern: &'static str,
    ops: Vec<SignalControll>,
}

impl Instruction {
    pub fn new(pattern: &'static str, signal_controlls: Vec<SignalControll>) -> Self {
        unimplemented!()
    }

    pub fn try_match(ir: Word) -> bool {
        unimplemented!()
    }
}

pub const INST_SET: LazyLock<Vec<Instruction>> = LazyLock::new(|| unimplemented!());
