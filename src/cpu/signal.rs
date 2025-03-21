use crate::common::PAddr;

pub enum ALUOperation {
    Plus,
    SignExtension(usize),
    ZeroExtension,
}

pub enum SignalControll {
    RegWrite(usize),
    RegRead(usize),
    MemRead(PAddr),
    MemWrite(PAddr),
    ImmRead(usize, usize),
    ALUOp(ALUOperation),
}
