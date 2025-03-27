use crate::common::{PAddr, Word};

use super::{Cpu, ExecuteState};

pub enum ALUOperation {
    Plus,
    SignExtend(usize),
    ZeroExtend,
}

pub enum SignalControl {
    RegWrite,
    RegRead(usize),
    MemRead(PAddr),
    MemWrite(PAddr),
    ImmRead,
    NumPush(Word),
    NumPop,
    Halt,
    ALUOp(ALUOperation),
}

impl ALUOperation {
    pub fn exec_alu_operation(&self, exec_state: &mut ExecuteState) {
        match &self {
            Self::Plus => {
                let l2 = exec_state.stack.pop().unwrap();
                let l1 = exec_state.stack.pop().unwrap();
                exec_state.stack.push(l1 + l2);
            }
            _ => unimplemented!(),
        }
    }
}

impl SignalControl {
    pub fn exec_signal(&self, exec_state: &mut ExecuteState, cpu: &mut Cpu) {
        match self {
            Self::Halt => {
                exec_state.stop = true;
            }
            Self::RegWrite => {
                let val: Word = exec_state.stack.pop().unwrap();
                cpu.gpr[exec_state.rd] = val;
            }
            Self::RegRead(1) => {
                let reg_id = exec_state.rs1;
                exec_state.stack.push(cpu.gpr[reg_id]);
            }
            Self::RegRead(2) => {
                let reg_id = exec_state.rs2;
                exec_state.stack.push(cpu.gpr[reg_id]);
            }
            Self::ALUOp(alu_op) => {
                alu_op.exec_alu_operation(exec_state);
            }
            _ => unimplemented!(),
        }
    }
}
