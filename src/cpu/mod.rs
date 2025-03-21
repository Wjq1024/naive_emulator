use init::CPU;

use crate::common::*;

mod init;
mod inst;
mod signal;

const GPR_SIZE: usize = 32;

pub struct Cpu {
    gpr: [Word; GPR_SIZE],
    pc: PAddr,
}

struct ExecStatus {
    pc: PAddr,
    snpc: PAddr,
    dnpc: PAddr,
    ir: Word,
    stack: Vec<Word>,
}

impl ExecStatus {
    pub fn new(pc: PAddr) -> ExecStatus {
        ExecStatus {
            pc: pc,
            snpc: pc,
            dnpc: pc,
            ir: 0,
            stack: Vec::new(),
        }
    }
}

impl Cpu {
    fn run(&self, inst_num: usize) {
        for _ in 0..inst_num {}
    }

    fn fetch_inst(&self) -> ExecStatus {
        unimplemented!()
    }

    fn decode_and_exec(exec_status: ExecStatus) -> bool {
        unimplemented!()
    }
}

pub fn cpu_run(inst_num: usize) {
    unimplemented!()
}
