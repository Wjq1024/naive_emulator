use std::sync::LazyLock;

use super::{
    Cpu, GPR_SIZE,
    inst::Instruction,
    signal::{ALUOperation, SignalControl},
};
use crate::{
    common::*,
    cpu::{inst::InstructionType, signal::Condition},
    uniprocessor::UPSafeCell,
};

pub static CPU: LazyLock<UPSafeCell<Cpu>> = LazyLock::new(|| unsafe {
    UPSafeCell::new(Cpu {
        gpr: [0; GPR_SIZE],
        pc: PC_ENTRY,

        gpr_lock: [false; GPR_SIZE],
        pc_lock: false,
        mem_lock: false
    })
});

pub static INSTRUCTION_SET: LazyLock<Vec<Instruction>> = LazyLock::new(|| {
    vec![
        // halt, halt the machine
        Instruction {
            inst_code: 0b000000,
            type_: InstructionType::D,
            ops: vec![SignalControl::Halt],
            read_mm: false,
            write_mm: false,
            write_back: false,
            write_pc: false,
        },
        // add, x[rd] = x[rs1] + x[rs2]
        Instruction {
            inst_code: 0b000001,
            type_: InstructionType::A,
            ops: vec![
                SignalControl::RegRead(1),
                SignalControl::RegRead(2),
                SignalControl::ALUOp(ALUOperation::Add),
            ],
            read_mm: false,
            write_mm: false,
            write_back: true,
            write_pc: false,
        },
        // addi, x[rd] = x[rs1] + sext(imm, 16)
        Instruction {
            inst_code: 0b000010,
            type_: InstructionType::B,
            ops: vec![
                SignalControl::RegRead(1),
                SignalControl::ImmRead,
                SignalControl::ALUOp(ALUOperation::SignExtend(16)),
                SignalControl::ALUOp(ALUOperation::Add),
            ],
            read_mm: false,
            write_mm: false,
            write_back: true,
            write_pc: false,
        },
        // bne, if (x[rs1] != x[rs2]) pc += sext(imm)
        Instruction {
            inst_code: 0b000011,
            type_: InstructionType::C,
            ops: vec![
                SignalControl::RegRead(1),
                SignalControl::RegRead(2),
                SignalControl::ALUOp(ALUOperation::Compare),
                SignalControl::CondExec(Condition::NotEqual),
                SignalControl::PCRead,
                SignalControl::ImmRead,
                SignalControl::ALUOp(ALUOperation::SignExtend(16)),
                SignalControl::ALUOp(ALUOperation::Add),
            ],
            read_mm: false,
            write_mm: false,
            write_back: false,
            write_pc: true,
        },
        // mul, x[rd] = x[rs1] * x[rs2]
        Instruction {
            inst_code: 0b000100,
            type_: InstructionType::A,
            ops: vec![
                SignalControl::RegRead(1),
                SignalControl::RegRead(2),
                SignalControl::ALUOp(ALUOperation::Multiply),
            ],
            read_mm: false,
            write_mm: false,
            write_back: true,
            write_pc: false,
        },
        // lui, x[rd] = (sext(imm) << 16)
        Instruction {
            inst_code: 0b000101,
            type_: InstructionType::B,
            ops: vec![
                SignalControl::ImmRead,
                SignalControl::ALUOp(ALUOperation::SignExtend(16)),
                SignalControl::NumPush(16),
                SignalControl::ALUOp(ALUOperation::ShiftLeftLogical),
            ],
            read_mm: false,
            write_mm: false,
            write_back: true,
            write_pc: false,
        },
        // lw, x[rd] = M[x[rs1] + sext(imm)]
        Instruction {
            inst_code: 0b000110,
            type_: InstructionType::B,
            ops: vec![
                SignalControl::ImmRead,
                SignalControl::ALUOp(ALUOperation::SignExtend(16)),
                SignalControl::RegRead(1),
                SignalControl::ALUOp(ALUOperation::Add),
            ],
            read_mm: true,
            write_mm: false,
            write_back: true,
            write_pc: false,
        },
        // sw, M[x[rs1] + sext(imm)] = x[rs2]
        Instruction {
            inst_code: 0b000111,
            type_: InstructionType::C,
            ops: vec![
                SignalControl::RegRead(1),
                SignalControl::ImmRead,
                SignalControl::ALUOp(ALUOperation::SignExtend(16)),
                SignalControl::ALUOp(ALUOperation::Add),
                SignalControl::RegRead(2),
            ],
            read_mm: false,
            write_mm: true,
            write_back: false,
            write_pc: false,
        },
        // blt, if (rs1 <s rs2) pc += sext(offset)
        Instruction {
            inst_code: 0b001000,
            type_: InstructionType::C,
            ops: vec![
                SignalControl::RegRead(1),
                SignalControl::RegRead(2),
                SignalControl::ALUOp(ALUOperation::Compare),
                SignalControl::CondExec(Condition::SignLess),
                SignalControl::PCRead,
                SignalControl::ImmRead,
                SignalControl::ALUOp(ALUOperation::SignExtend(16)),
                SignalControl::ALUOp(ALUOperation::Add),
            ],
            read_mm: false,
            write_mm: false,
            write_back: false,
            write_pc: true,
        },
        // slli, x[rd] = x[rs1] << imm
        Instruction {
            inst_code: 0b001001,
            type_: InstructionType::B,
            ops: vec![
                SignalControl::RegRead(1),
                SignalControl::ImmRead,
                SignalControl::ALUOp(ALUOperation::ShiftLeftLogical),
            ],
            read_mm: false,
            write_mm: false,
            write_back: true,
            write_pc: false,
        },
        // sub, x[rd] = x[rs1] - x[rs2]
        Instruction {
            inst_code: 0b001010,
            type_: InstructionType::A,
            ops: vec![
                SignalControl::RegRead(1),
                SignalControl::RegRead(2),
                SignalControl::ALUOp(ALUOperation::Negate),
                SignalControl::ALUOp(ALUOperation::Add),
            ],
            read_mm: false,
            write_mm: false,
            write_back: true,
            write_pc: false,
        },
    ]
});
