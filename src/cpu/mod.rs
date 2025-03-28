use init::INSTRUCTION_SET;
use inst::Instruction;

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

struct ExecuteState<'a> {
    pc: PAddr,
    npc: PAddr,
    ir: Option<Word>,
    corr_inst: Option<&'a Instruction>,
    rd: Option<usize>,
    rs1: Option<usize>,
    rs2: Option<usize>,
    imm: Option<usize>,
    stack: Vec<Word>,
    halt: bool,
    stop_exec: bool,
}

impl<'a> ExecuteState<'a> {
    fn new(pc: PAddr) -> ExecuteState<'a> {
        ExecuteState {
            pc: pc,
            npc: (pc.0 + 4).into(),
            ir: None,
            corr_inst: None,
            rd: None,
            rs1: None,
            rs2: None,
            imm: None,
            stack: Vec::new(),
            halt: false,
            stop_exec: false,
        }
    }
}

impl Cpu {
    fn run(&mut self, inst_num: usize) {
        if *EMULATOR_STATUS.exclusive_access() == EmulatorStatus::Stop {
            return;
        }
        *EMULATOR_STATUS.exclusive_access() = EmulatorStatus::Running;
        for _ in 0..inst_num {
            let mut exec_state = ExecuteState::new(self.pc);
            self.fetch_inst(&mut exec_state);
            Self::decode_inst(&mut exec_state);
            self.exec_inst(&mut exec_state);
            if exec_state.halt {
                *EMULATOR_STATUS.exclusive_access() = EmulatorStatus::Stop;
                break;
            }
            self.pc = exec_state.npc;
        }
        if *EMULATOR_STATUS.exclusive_access() != EmulatorStatus::Stop {
            *EMULATOR_STATUS.exclusive_access() = EmulatorStatus::Idle;
        }
    }

    fn fetch_inst(&self, exec_state: &mut ExecuteState) {
        exec_state.ir = Some(paddr_read(exec_state.pc, 4));
    }

    fn decode_inst(exec_state: &mut ExecuteState) {
        let inst = exec_state.ir.unwrap();
        let corr_inst: &Instruction = INSTRUCTION_SET.iter().find(|x| x.is_match(inst)).unwrap();
        exec_state.corr_inst = Some(corr_inst);
        corr_inst.decode_inst(exec_state);
    }

    fn exec_inst(&mut self, exec_state: &mut ExecuteState) {
        exec_state.corr_inst.unwrap().exec_inst(exec_state, self);
        self.gpr[0] = 0;
    }
}

pub fn cpu_run(inst_num: usize) {
    CPU.exclusive_access().run(inst_num);
}

pub fn cpu_gpr(gpr_id: usize) -> Word {
    CPU.exclusive_access().gpr[gpr_id]
}

pub fn cpu_pc() -> PAddr {
    CPU.exclusive_access().pc
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
        exec_status.ir = Some(inst);
        Cpu::decode_inst(&mut exec_status);
        assert_eq!(exec_status.rs1, Some(1));
        assert_eq!(exec_status.rs2, Some(2));
        assert_eq!(exec_status.rd, Some(3));
        assert_eq!(exec_status.imm, None);
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
        let mut exec_state = ExecuteState::new(0x8000_0000.into());
        cpu.fetch_inst(&mut exec_state);
        Cpu::decode_inst(&mut exec_state);
        cpu.exec_inst(&mut exec_state);
        assert_eq!(exec_state.npc, 0x8000_0004.into());
        assert_eq!(cpu.gpr[3], 35);
    }
}
