use crate::{
    common::{PAddr, Word},
    memory::paddr_write,
};
use std::{fs::File, io::Read};


use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn test_load() {
    paddr_write(
        0x8000_0000.into(),
        4,
        0b00000000000_10001_00000_00001_000010,
    );
    paddr_write(
        0x8000_0004.into(),
        4,
        0b11111111111_11111_00000_00010_000010,
    );
    paddr_write(
        0x8000_0008.into(),
        4,
        0b00000000001_00010_00001_00011_000001,
    );
    paddr_write(
        0x8000_000C.into(),
        4,
        0b00000000000_00000_00000_00000_000000,
    );
}


// 将汇编指令转换为机器码的函数
fn assemble_inst(inst: &str) -> Option<Word> {
    let machine_code = match inst.trim() {
        // addi, R(1) = R(0) + 1
        "li r1, 1" => Some(0b00000000000_00001_00000_00001_000010),
        // addi, R(2) = R(0) + 6
        "li r2, 6" => Some(0b00000000000_00110_00000_00010_000010),
        // addi, R(3) = R(0) + 1
        "li r3, 1" => Some(0b00000000000_00001_00000_00011_000010),
        // mul, R(1) = R(1) * R(3)
        "mul r1, r1, r3" => Some(0b00000000000_00011_00001_00001_000100),
        // addi, R(3) = R(3) + 1
        "addi r3, r3, 1" => Some(0b00000000000_00001_00011_00011_000010),
        // bne, if (R(3) != R(2)) pc += -0x8
        "bne r3, r2, loop" => Some(0b11111111111_00010_00011_11000_000011),
        // halt
        "li r0, 0" => Some(0b00000000000_00000_00000_00000_000000),
        // addi, R(1) = R(0) + 0
        "li r1, 0" => Some(0b00000000000_00000_00000_00001_000010),
        // addi, R(2) = R(0) + 10
        "li r2, 10" => Some(0b00000000000_01010_00000_00010_000010),
        // addi, R(3) = R(0) + 0
        "li r3, 0" => Some(0b00000000000_00000_00000_00011_000010),

        // add, R(1) = R(1) + R(3)
        "add r1, r1, r3" => Some(0b00000000000_00011_00001_00001_000001),
        // bne, if (R(3) != R(2)) pc += -0x8

        _ => None,
    };
    if let Some(code) = machine_code {
        println!("Instruction: {}, Machine code: {:#b}", inst, code);
    }
    machine_code
}

// 读取汇编文件并转换为机器码
pub fn assemble_asm_file(file_path: &str) -> Vec<Word> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut machine_code = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            if let Some(inst) = assemble_inst(trimmed) {
                machine_code.push(inst);
            }
        }
    }

    machine_code
}

// 加载汇编文件并将其转换为机器码后加载到内存中
pub fn load_asm_file(file_path: &str) {
    let machine_code = assemble_asm_file(file_path);
    naive_load(&machine_code);
}

// 原有的 naive_load 函数
pub fn naive_load(img: &[Word]) {
    img.iter().enumerate().for_each(|(idx, inst)| {
        paddr_write(PAddr(0x8000_0000 + idx as u32 * 4), 4, *inst);
    });
}

pub fn binary_file_load(file_path: String) {
    let mut file = File::open(file_path).unwrap();
    let mut program = Vec::<Word>::new();
    let mut buf = [0_u8; 4];
    while let Ok(len) = file.read(&mut buf) {
        if len == 0 {
            break;
        }
        let word = buf.iter().enumerate().fold(0 as Word, |acc, (idx, byte)| {
            acc + ((*byte as Word) << (idx * 8))
        });
        program.push(word);
    }
    naive_load(&program);
}
