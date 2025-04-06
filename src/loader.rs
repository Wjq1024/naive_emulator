use crate::{
    common::{PAddr, Word},
    memory::paddr_write,
};
use std::{fs::File, io::Read};

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
    assert!(program.len() % 4 == 0);
    naive_load(&program);
}
