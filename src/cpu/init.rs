use std::sync::LazyLock;

use super::{
    Cpu, GPR_SIZE,
    inst::Instruction,
    signal::{ALUOperation, SignalControl},
};
use crate::{common::*, uniprocessor::UPSafeCell};

pub static CPU: LazyLock<UPSafeCell<Cpu>> = LazyLock::new(|| unsafe {
    UPSafeCell::new(Cpu {
        gpr: [0; GPR_SIZE],
        pc: PC_ENTRY,
    })
});

pub static INSTRUCTION_SET: LazyLock<Vec<Instruction>> = LazyLock::new(|| {
    vec![
        // halt, halt the machine
        Instruction {
            inst_code: 0b000000,
            ops: vec![SignalControl::Halt],
        },
        // add, rd = rs1 + rs2
        Instruction {
            inst_code: 0b000001,
            ops: vec![
                SignalControl::RegRead(1),
                SignalControl::RegRead(2),
                SignalControl::ALUOp(ALUOperation::Plus),
                SignalControl::RegWrite,
            ],
        },
        // addi, rd = rs1 + sext(imm, 16)
        Instruction {
            inst_code: 0b000010,
            ops: vec![
                SignalControl::RegRead(1),
                SignalControl::ImmRead,
                SignalControl::ALUOp(ALUOperation::SignExtend(16)),
                SignalControl::ALUOp(ALUOperation::Plus),
                SignalControl::RegWrite,
            ],
        },
    ]
});
