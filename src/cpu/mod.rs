use std::collections::VecDeque;

use init::INSTRUCTION_SET;
use inst::Instruction;
use log::info;

use crate::{common::*, memory::{paddr_read, paddr_write}};

pub use init::CPU;

mod init;
mod inst;
mod signal;

const GPR_SIZE: usize = 1 << 5;

#[derive(Clone, Debug)]
pub struct Cpu {
    gpr: [Word; GPR_SIZE],
    pc: PAddr,

    gpr_lock: [bool; GPR_SIZE],
    pc_lock: bool,
    mem_lock: bool,
}

struct ExecuteState<'a> {
    next_state: usize,
    pc: PAddr,
    npc: PAddr,
    ir: Option<Word>,

    curr_inst: Option<&'a Instruction>,

    rd: Option<usize>,
    rs1: Option<usize>,
    rs1_val: Option<Word>,
    rs2: Option<usize>,
    rs2_val: Option<Word>,
    imm: Option<usize>,
    
    rd_lock: bool,
    mm_lock: bool,
    pc_lock: bool,

    stack: Vec<Word>,
    halt: bool,
    stop_exec: bool,
}

impl<'a> ExecuteState<'a> {
    fn new(pc: PAddr) -> ExecuteState<'a> {
        ExecuteState {
            next_state: 1,
            pc: pc,
            npc: (pc.0 + 4).into(),
            ir: None,
            curr_inst: None,

            rd: None,
            rs1: None,
            rs1_val: None,
            rs2: None,
            rs2_val: None,
            imm: None,

            rd_lock: false,
            mm_lock: false,
            pc_lock: false,

            stack: Vec::new(),
            halt: false,
            stop_exec: false,
        }
    }

    fn fetch_inst(&mut self) -> bool {
        let mut cpu = CPU.exclusive_access();
        if cpu.pc_lock {
            log::info!("fetch_inst: fetch instruction FAILED");
            return false;
        }
        self.pc = cpu.pc;
        self.npc = PAddr(self.pc.0 + 4);
        log::info!("fetch_inst: fetch instruction at pc = {}", self.pc);

        self.ir = Some(paddr_read(self.pc, 4));
        let inst = self.ir.unwrap();
        if inst == 0 {
            self.halt = true;
        }
        let curr_inst: &Instruction = INSTRUCTION_SET.iter().find(|x| x.is_match(inst)).unwrap();
        self.curr_inst = Some(curr_inst);

        true
    }

    fn decode_inst(&mut self) -> bool {
        log::info!("decode_inst: inst pc = {}", self.pc);
        let curr_inst = self.curr_inst.unwrap();
        curr_inst.decode_inst(self);
        let mut cpu = CPU.exclusive_access();

        let mut isok = true;

        if !curr_inst.write_pc {
            cpu.pc = self.npc;
        } else {
            if !self.pc_lock { 
                self.pc_lock = true;
                cpu.pc_lock = true;
                log::info!("decode_inst: pc_locked");
            }
        }

        if curr_inst.read_mm || curr_inst.write_mm {
            if !self.mm_lock {
                if cpu.mem_lock {
                    isok = false;
                } else {
                    self.mm_lock = true;
                    cpu.mem_lock = true;
                    log::info!("decode_inst: mem_locked");
                }
            }
        }

        if let Some(rs1) = self.rs1 {
            if None == self.rs1_val {
                if !cpu.gpr_lock[rs1] {
                    self.rs1_val = Some(cpu.gpr[rs1]);
                } else {
                    isok = false;
                }
            }
        }
        if let Some(rs2) = self.rs2 {
            if None == self.rs2_val {
                if !cpu.gpr_lock[rs2] {
                    self.rs2_val = Some(cpu.gpr[rs2]);
                } else {
                    isok = false;
                }
            }
        }

        if let Some(rd) = self.rd {
            if !self.rd_lock {
                if curr_inst.write_back {
                    if cpu.gpr_lock[rd] {
                        isok = false;
                    } else {
                        self.rd_lock = true;
                        cpu.gpr_lock[rd] = true;
                    }
                }
            }
        }

        isok
    }

    fn exec_inst(&mut self) -> bool {
        self.curr_inst.unwrap().exec_inst(self);
        true
    }

    fn access_memory(&mut self) -> bool {
        let curr_inst = self.curr_inst.unwrap();
        if curr_inst.read_mm {
            let addr = PAddr(self.stack.pop().unwrap());
            let data = paddr_read(addr, 4);
            self.stack.push(data);
        }
        if curr_inst.write_mm {
            let data = self.stack.pop().unwrap();
            let addr = PAddr(self.stack.pop().unwrap());
            paddr_write(addr, 4, data); 
        }

        if curr_inst.read_mm || curr_inst.write_mm {
            CPU.exclusive_access().mem_lock = false;
        }
        true
    }

    fn write_back(&mut self) -> bool {
        let mut cpu = CPU.exclusive_access();
        let curr_inst = self.curr_inst.unwrap();
        if curr_inst.write_back {
            let val = self.stack.pop().unwrap();
            cpu.gpr[self.rd.unwrap()] = val;
            cpu.gpr_lock[self.rd.unwrap()] = false;
            log::info!("write_back: write {} into gpr{}",val, self.rd.unwrap());
            log::info!("write_back: gpr{}_unlocked", self.rd.unwrap());
        }
        if curr_inst.write_pc {
            cpu.pc_lock = false;
            log::info!("write_back: pc_unlocked");
            if !self.stop_exec {
                cpu.pc = PAddr(self.stack.pop().unwrap());
            } else {
                cpu.pc = self.npc;
            }
            log::info!("write_back: next pc = {}", cpu.pc);
        }
        true
    }
}

impl<'a> Iterator for ExecuteState<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_state == 6 {
            return None;
        }
        let isok = match self.next_state {
            1 => self.fetch_inst(),
            2 => self.decode_inst(),
            3 => self.exec_inst(),
            4 => self.access_memory(),
            5 => self.write_back(),
            _ => unimplemented!()
        };
        if isok {
            self.next_state += 1;
        }
        if isok && self.next_state == 2_usize {
            return Some(1);
        }
        Some(0)
    }
}

impl Cpu {
    fn run(inst_num: usize) {
        if *EMULATOR_STATUS.exclusive_access() == EmulatorStatus::Stop {
            return;
        }
        *EMULATOR_STATUS.exclusive_access() = EmulatorStatus::Running;
        //let mut counter = 0_usize;
        let mut exec_vec = VecDeque::<ExecuteState>::new();
        exec_vec.push_back(ExecuteState::new(CPU.exclusive_access().pc));
        while !exec_vec.is_empty() {
            let mut new_exec = false; 
            for exec_stt in exec_vec.iter_mut() {
                if let Some(1) = exec_stt.next() {
                    new_exec = true;
                }
                if exec_stt.halt {
                    *EMULATOR_STATUS.exclusive_access() = EmulatorStatus::Stop;
                }
            }
            if new_exec && *EMULATOR_STATUS.exclusive_access() != EmulatorStatus::Stop {
                exec_vec.push_back(ExecuteState::new(CPU.exclusive_access().pc));
            }
            while let Some(exec_stt) = exec_vec.front() {
                if exec_stt.next_state >= 6 {
                    //counter
                    exec_vec.pop_front();
                } else {
                    break;
                }
            }
        }
        if *EMULATOR_STATUS.exclusive_access() != EmulatorStatus::Stop {
            *EMULATOR_STATUS.exclusive_access() = EmulatorStatus::Idle;
        }
    }

    // fn fetch_inst(&self, exec_state: &mut ExecuteState) {
    //     exec_state.ir = Some(paddr_read(exec_state.pc, 4));
    // }

    // fn decode_inst(exec_state: &mut ExecuteState) {
    //     let inst = exec_state.ir.unwrap();
    //     let corr_inst: &Instruction = INSTRUCTION_SET.iter().find(|x| x.is_match(inst)).unwrap();
    //     exec_state.corr_inst = Some(corr_inst);
    //     corr_inst.decode_inst(exec_state);
    // }

    // fn exec_inst(&mut self, exec_state: &mut ExecuteState) {
    //     exec_state.corr_inst.unwrap().exec_inst(exec_state);
    //     self.gpr[0] = 0;
    // }
}

pub fn cpu_run(inst_num: usize) {
    Cpu::run(inst_num);
}

pub fn cpu_gpr(gpr_id: usize) -> Word {
    CPU.exclusive_access().gpr[gpr_id]
}

pub fn cpu_pc() -> PAddr {
    CPU.exclusive_access().pc
}
