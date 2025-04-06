use std::sync::LazyLock;

use super::{
    Cpu, GPR_SIZE,
    inst::Instruction,
    signal::{ALUOperation, SignalControl},
};
use crate::{common::*, cpu::inst::InstructionType, uniprocessor::UPSafeCell};

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
            type_: InstructionType::D,
            ops: vec![SignalControl::Halt],
        },
        // add, x[rd] = x[rs1] + x[rs2]
        Instruction {
            inst_code: 0b000001,
            type_: InstructionType::A,
            ops: vec![
                SignalControl::RegRead(1),
                SignalControl::RegRead(2),
                SignalControl::ALUOp(ALUOperation::Plus),
                SignalControl::RegWrite,
            ],
        },
        // addi, x[rd] = x[rs1] + sext(imm, 16)
        Instruction {
            inst_code: 0b000010,
            type_: InstructionType::B,
            ops: vec![
                SignalControl::RegRead(1),
                SignalControl::ImmRead,
                SignalControl::ALUOp(ALUOperation::SignExtend(16)),
                SignalControl::ALUOp(ALUOperation::Plus),
                SignalControl::RegWrite,
            ],
        },
        // bne, if (x[rs1] != x[rs2]) pc += sext(imm)
        Instruction {
            inst_code: 0b000011,
            type_: InstructionType::C,
            ops: vec![
                SignalControl::RegRead(1),
                SignalControl::RegRead(2),
                SignalControl::ALUOp(ALUOperation::Negate),
                SignalControl::ALUOp(ALUOperation::Plus),
                SignalControl::CondExec,
                SignalControl::PCRead,
                SignalControl::ImmRead,
                SignalControl::ALUOp(ALUOperation::SignExtend(16)),
                SignalControl::ALUOp(ALUOperation::Plus),
                SignalControl::PCWrite,
            ],
        },
        // mul, x[rd] = x[rs1] * x[rs2]
        Instruction {
            inst_code: 0b000100,
            type_: InstructionType::A,
            ops: vec![
                SignalControl::RegRead(1),
                SignalControl::RegRead(2),
                SignalControl::ALUOp(ALUOperation::Multiply),
                SignalControl::RegWrite,
            ],
        },
        // bge, if (x[rs1] >=s x[rs2]) pc += sext(imm)
    ]
});
