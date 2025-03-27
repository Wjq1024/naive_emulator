use init::INSTRUCTION_SET;
use inst::{INST_IMM_MASK, INST_RD_MASK, INST_RS1_MASK, INST_RS2_MASK, Instruction};

use crate::{common::*, memory::paddr_read};

pub use init::CPU;

mod init;
mod inst;
mod signal;

const GPR_SIZE: usize = 1 << 5;

#[derive(Debug, Clone)]
pub struct Cpu {
    gpr: [Word; GPR_SIZE],
    pc: PAddr,
}

struct ExecuteState {
    pc: PAddr,
    npc: PAddr,
    ir: Word,
    rd: usize,
    rs1: usize,
    rs2: usize,
    imm: usize,
    stack: Vec<Word>,
    stop: bool,
}

impl ExecuteState {
    fn new(pc: PAddr) -> ExecuteState {
        ExecuteState {
            pc: pc,
            npc: (pc.0 + 4).into(),
            ir: 0,
            rd: 0,
            rs1: 0,
            rs2: 0,
            imm: 0,
            stack: Vec::new(),
            stop: false,
        }
    }

    fn decode(&mut self) {
        let inst = self.ir;
        self.rd = ((inst & INST_RD_MASK) >> 6) as usize;
        self.rs1 = ((inst & INST_RS1_MASK) >> 11) as usize;
        self.rs2 = ((inst & INST_RS2_MASK) >> 16) as usize;
        self.imm = ((inst & INST_IMM_MASK) >> 16) as usize;
    }
}

impl Cpu {
    fn run(&mut self, inst_num: usize) {
        if *EMULATOR_STATUS.exclusive_access() == EmulatorStatus::Stop {
            return;
        }
        *EMULATOR_STATUS.exclusive_access() = EmulatorStatus::Running;
        for _ in 0..inst_num {
            let mut exec_state = self.fetch_inst();
            Self::decode_inst(&mut exec_state);
            self.exec_inst(&mut exec_state);
            if exec_state.stop {
                *EMULATOR_STATUS.exclusive_access() = EmulatorStatus::Stop;
                break;
            }
            self.pc = exec_state.npc;
        }
        if *EMULATOR_STATUS.exclusive_access() != EmulatorStatus::Stop {
            *EMULATOR_STATUS.exclusive_access() = EmulatorStatus::Idle;
        }
    }

    fn fetch_inst(&self) -> ExecuteState {
        let pc = self.pc;
        let mut ret = ExecuteState::new(pc);
        ret.ir = paddr_read(pc, 4);
        ret
    }

    fn decode_inst(exec_state: &mut ExecuteState) {
        exec_state.decode();
    }

    fn exec_inst(&mut self, exec_state: &mut ExecuteState) {
        let inst = exec_state.ir;
        let corr_inst: &Instruction = INSTRUCTION_SET.iter().find(|x| x.is_match(inst)).unwrap();
        corr_inst.exec_inst(exec_state, self);
        self.gpr[0] = 0;
    }
}

pub fn cpu_run(inst_num: usize) {
    CPU.exclusive_access().run(inst_num);
}

#[cfg(test)]
mod tests {
    use crate::{
        common::Word,
        cpu::{Cpu, ExecuteState},
        memory::paddr_write,
    };

    use super::GPR_SIZE;

    #[test]
    fn test_decode_inst() {
        let mut exec_status = ExecuteState::new(0x8000_0000.into());
        let inst: Word = 0b00000000001_00010_00001_00011_000001;
        exec_status.ir = inst;
        Cpu::decode_inst(&mut exec_status);
        assert_eq!(exec_status.rs1, 1);
        assert_eq!(exec_status.rs2, 2);
        assert_eq!(exec_status.rd, 3);
        assert_eq!(exec_status.imm, 2 + 32);
    }

    #[test]
    fn test_exec_add() {
        let mut cpu = Cpu {
            gpr: [0; GPR_SIZE],
            pc: 0x8000_0000.into(),
        };
        cpu.gpr[1] = 17;
        cpu.gpr[2] = 18;
        cpu.gpr[3] = 0;
        paddr_write(
            0x8000_0000.into(),
            4,
            0b00000000001_00010_00001_00011_000001,
        );
        let mut exec_state = cpu.fetch_inst();
        Cpu::decode_inst(&mut exec_state);
        cpu.exec_inst(&mut exec_state);
        assert_eq!(exec_state.npc, 0x8000_0004.into());
        assert_eq!(cpu.gpr[3], 35);
    }
}
