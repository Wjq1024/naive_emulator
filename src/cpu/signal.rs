use crate::common::{PAddr, Word};

use super::{Cpu, ExecuteState};

#[derive(Debug)]
pub enum ALUOperation {
    Plus,
    SignExtend(usize),
    ZeroExtend,
}

#[derive(Debug)]
pub enum SignalControl {
    RegWrite,
    RegRead(usize),
    PCRead,
    PCWrite,
    MemRead(PAddr),
    MemWrite(PAddr),
    ImmRead,
    NumPush(Word),
    NumPop,
    Halt,
    ALUOp(ALUOperation),
}

impl ALUOperation {
    fn sign_extend(value: Word, bits: usize) -> Word {
        match value >> (bits - 1) & 1 {
            0 => Self::zero_extend(value, bits),
            1 => value | (Word::MAX - (1 << bits) + 1),
            _ => panic!("unreachable pattern!"),
        }
    }

    fn zero_extend(value: Word, bits: usize) -> Word {
        value & ((1 << bits) - 1)
    }

    pub fn exec_alu_operation(&self, exec_state: &mut ExecuteState) {
        match self {
            Self::Plus => {
                let l2 = exec_state.stack.pop().unwrap();
                let l1 = exec_state.stack.pop().unwrap();
                exec_state.stack.push(l1 + l2);
            }
            Self::ZeroExtend => (),
            Self::SignExtend(bits) => {
                let val = exec_state.stack.pop().unwrap();
                exec_state.stack.push(Self::sign_extend(val, *bits));
            }
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
            Self::PCRead => {
                exec_state.stack.push(exec_state.pc.0);
            }
            Self::PCWrite => {
                exec_state.npc = PAddr(exec_state.stack.pop().unwrap());
            }
            Self::ImmRead => {
                exec_state.stack.push(exec_state.imm as Word);
            }
            Self::ALUOp(alu_op) => {
                alu_op.exec_alu_operation(exec_state);
            }
            _ => unimplemented!("{:?}", self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ALUOperation;

    #[test]
    fn test_extension() {
        assert_eq!(ALUOperation::sign_extend(0x1B, 5), 0xFFFFFFFB);
        assert_eq!(ALUOperation::sign_extend(0x1B, 6), 0x0000001B);
        assert_eq!(ALUOperation::sign_extend(0x1B, 3), 0x00000003);
        assert_eq!(ALUOperation::sign_extend(0x1B, 2), 0xFFFFFFFF);
        assert_eq!(ALUOperation::zero_extend(0x1B, 2), 0x00000003);
    }
}
