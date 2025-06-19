use crate::common::Word;

use super::{Cpu, ExecuteState, signal::SignalControl};

const INST_CODE_MASK: Word = 0x3F;

pub(super) enum InstructionType {
    // rs1, rs2, rd
    A,
    // rs1, imm, rd
    B,
    // rs1, rs2, imm
    C,
    // for fun
    D,
}

pub(super) struct Instruction {
    pub(super) inst_code: Word,
    pub(super) type_: InstructionType,
    pub(super) ops: Vec<SignalControl>,
    pub(super) write_mm: bool,
    pub(super) read_mm: bool,
    pub(super) write_back: bool,
    pub(super) write_pc: bool,
}

impl Instruction {
    pub fn is_match(&self, ir: Word) -> bool {
        (ir & INST_CODE_MASK) == self.inst_code
    }

    pub fn decode_inst(&self, exec_state: &mut ExecuteState) {
        match &self.type_ {
            InstructionType::A => {
                const INST_RD_MASK: Word = ((1 << 11) - 1) - ((1 << 6) - 1);
                const INST_RS1_MASK: Word = ((1 << 16) - 1) - ((1 << 11) - 1);
                const INST_RS2_MASK: Word = ((1 << 21) - 1) - ((1 << 16) - 1);
                exec_state.rd = Some(((exec_state.ir.unwrap() & INST_RD_MASK) >> 6) as usize);
                exec_state.rs1 = Some(((exec_state.ir.unwrap() & INST_RS1_MASK) >> 11) as usize);
                exec_state.rs2 = Some(((exec_state.ir.unwrap() & INST_RS2_MASK) >> 16) as usize);
            }
            InstructionType::B => {
                const INST_RD_MASK: Word = ((1 << 11) - 1) - ((1 << 6) - 1);
                const INST_RS1_MASK: Word = ((1 << 16) - 1) - ((1 << 11) - 1);
                const INST_IMM_MASK: Word = Word::MAX - ((1 << 16) - 1);
                exec_state.rd = Some(((exec_state.ir.unwrap() & INST_RD_MASK) >> 6) as usize);
                exec_state.rs1 = Some(((exec_state.ir.unwrap() & INST_RS1_MASK) >> 11) as usize);
                exec_state.imm = Some(((exec_state.ir.unwrap() & INST_IMM_MASK) >> 16) as usize);
            }
            InstructionType::C => {
                const INST_RS1_MASK: Word = ((1 << 16) - 1) - ((1 << 11) - 1);
                const INST_RS2_MASK: Word = ((1 << 21) - 1) - ((1 << 16) - 1);
                const INST_IMM1_MASK: Word = ((1 << 11) - 1) - ((1 << 6) - 1);
                const INST_IMM2_MASK: Word = Word::MAX - ((1 << 21) - 1);
                exec_state.rs1 = Some(((exec_state.ir.unwrap() & INST_RS1_MASK) >> 11) as usize);
                exec_state.rs2 = Some(((exec_state.ir.unwrap() & INST_RS2_MASK) >> 16) as usize);
                let mut imm = (exec_state.ir.unwrap() & INST_IMM1_MASK) >> 6;
                imm += (exec_state.ir.unwrap() & INST_IMM2_MASK) >> 21 << 5;
                exec_state.imm = Some(imm as usize);
            }
            InstructionType::D => (),
        }
    }

    pub fn exec_inst(&self, exec_state: &mut ExecuteState) {
        assert!(self.is_match(exec_state.ir.unwrap()));
        for sig in &self.ops {
            sig.exec_signal(exec_state);
            if exec_state.stop_exec {
                break;
            }
        }
    }
}

