use crate::common::Word;

use super::{Cpu, ExecuteState, signal::SignalControl};

const INST_CODE_MASK: Word = 0x3F;
pub(super) const INST_RD_MASK: Word = ((1 << 11) - 1) - ((1 << 6) - 1);
pub(super) const INST_RS1_MASK: Word = ((1 << 16) - 1) - ((1 << 11) - 1);
pub(super) const INST_RS2_MASK: Word = ((1 << 21) - 1) - ((1 << 16) - 1);
pub(super) const INST_IMM_MASK: Word = Word::MAX - ((1 << 16) - 1);

pub struct Instruction {
    pub(super) inst_code: Word,
    pub(super) ops: Vec<SignalControl>,
}

impl Instruction {
    pub fn is_match(&self, ir: Word) -> bool {
        (ir & INST_CODE_MASK) == self.inst_code
    }

    pub fn exec_inst(&self, exec_state: &mut ExecuteState, cpu: &mut Cpu) {
        assert!(self.is_match(exec_state.ir));
        self.ops.iter().for_each(|x| x.exec_signal(exec_state, cpu));
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::{Cpu, ExecuteState, GPR_SIZE, init::INSTRUCTION_SET};

    use super::Instruction;

    #[test]
    fn test_inst_match() {
        let inst = Instruction {
            inst_code: 0b011011,
            ops: Vec::new(),
        };
        assert!(!inst.is_match(0x00));
        assert!(!inst.is_match(0x2A));
        assert!(!inst.is_match(0x3F));
        assert!(inst.is_match(0x1B));
    }

    #[test]
    fn test_exec_inst() {
        let mut exec_status = ExecuteState::new(0x8000_0000.into());
        exec_status.ir = 0;
        INSTRUCTION_SET[0].exec_inst(
            &mut exec_status,
            &mut Cpu {
                gpr: [0; GPR_SIZE],
                pc: 0x8000_0000.into(),
            },
        );
        assert!(exec_status.stop);
        assert!(exec_status.stack.is_empty());
    }
}
