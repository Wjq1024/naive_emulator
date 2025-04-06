use bitflags::bitflags;

use crate::common::{PAddr, SWord, Word};

use super::{Cpu, ExecuteState};

#[derive(Debug)]
pub enum ALUOperation {
    Add,
    Negate,
    Multiply,
    SignExtend(usize),
    ZeroExtend,
    ShiftLeftLogical,
    ShiftRightLogical,
    ShiftRightArithmetic,
    Compare,
}

bitflags! {
    pub struct CompareCode: Word {
        const CF = 1 << 0;
        const ZF = 1 << 1;
        const SF = 1 << 2;
        const OF = 1 << 3;
    }
}

#[derive(Debug)]
pub enum Condition {
    NotEqual,
    SignLess,
}

impl Condition {
    fn check(&self, code: CompareCode) -> bool {
        match self {
            Condition::NotEqual => !code.contains(CompareCode::ZF),
            Condition::SignLess => code.contains(CompareCode::SF) ^ code.contains(CompareCode::OF),
        }
    }
}

#[derive(Debug)]
pub enum SignalControl {
    RegWrite,
    RegRead(usize),
    PCRead,
    PCWrite,
    MemRead,
    MemWrite,
    ImmRead,
    NumPush(Word),
    NumPop,
    Halt,
    CondExec(Condition),
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
            Self::Add => {
                let l2 = exec_state.stack.pop().unwrap();
                let l1 = exec_state.stack.pop().unwrap();
                exec_state.stack.push(l1.wrapping_add(l2));
            }
            Self::ZeroExtend => (),
            Self::SignExtend(bits) => {
                let val = exec_state.stack.pop().unwrap();
                exec_state.stack.push(Self::sign_extend(val, *bits));
            }
            Self::Negate => {
                let mut val: SWord = exec_state.stack.pop().unwrap() as SWord;
                val = -val;
                exec_state.stack.push(val as Word);
            }
            Self::Multiply => {
                let l2 = exec_state.stack.pop().unwrap();
                let l1 = exec_state.stack.pop().unwrap();
                exec_state.stack.push(l1.wrapping_mul(l2));
            }
            Self::Compare => {
                let l2 = -(exec_state.stack.pop().unwrap() as SWord);
                let l1 = exec_state.stack.pop().unwrap() as SWord;
                let t = l1.wrapping_add(l2);
                let mut code = CompareCode::empty();
                if (t as Word) < (l1 as Word) {
                    code |= CompareCode::CF;
                }
                if t == 0 {
                    code |= CompareCode::ZF;
                }
                if t < 0 {
                    code |= CompareCode::SF;
                }
                if ((l1 < 0) == (l2 < 0)) && ((t < 0) != (l1 < 0)) {
                    code |= CompareCode::OF;
                }
                exec_state.stack.push(code.bits() as Word);
            }
            _ => unimplemented!(),
        }
    }
}

impl SignalControl {
    pub fn exec_signal(&self, exec_state: &mut ExecuteState, cpu: &mut Cpu) {
        match self {
            Self::Halt => {
                exec_state.halt = true;
            }
            Self::RegWrite => {
                let val: Word = exec_state.stack.pop().unwrap();
                cpu.gpr[exec_state.rd.unwrap()] = val;
            }
            Self::RegRead(1) => {
                let reg_id = exec_state.rs1.unwrap();
                exec_state.stack.push(cpu.gpr[reg_id]);
            }
            Self::RegRead(2) => {
                let reg_id = exec_state.rs2.unwrap();
                exec_state.stack.push(cpu.gpr[reg_id]);
            }
            Self::PCRead => {
                exec_state.stack.push(exec_state.pc.0);
            }
            Self::PCWrite => {
                exec_state.npc = PAddr(exec_state.stack.pop().unwrap());
            }
            Self::ImmRead => {
                exec_state.stack.push(exec_state.imm.unwrap() as Word);
            }
            Self::CondExec(cond) => {
                let ucode = exec_state.stack.pop().unwrap();
                let code = CompareCode::from_bits(ucode).unwrap();
                if !cond.check(code) {
                    exec_state.stop_exec = true;
                }
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
