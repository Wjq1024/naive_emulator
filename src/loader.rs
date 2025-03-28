use crate::{
    common::{PAddr, Word},
    memory::paddr_write,
};

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
