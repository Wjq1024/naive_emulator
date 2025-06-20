use naive_emulator::{
    cpu::{cpu_gpr, cpu_run},
    memory::paddr_write,
};

fn loader() {
    paddr_write(
        0x8000_0000.into(),
        4,
        0b00000000000_10001_00000_00001_000010,
    );
    paddr_write(
        0x8000_0004.into(),
        4,
        0b00000000000_10010_00000_00010_000010,
    );
    paddr_write(
        0x8000_0008.into(),
        4,
        0b00000000000_00010_00001_01000_000011,
    );
    paddr_write(
        0x8000_000C.into(),
        4,
        0b00000000001_00010_00001_00011_000001,
    );
    paddr_write(
        0x8000_0010.into(),
        4,
        0b00000000000_00000_00000_00000_000000,
    );
}

#[test]
fn test_bne() {
    loader();
    cpu_run(usize::MAX);
    assert_eq!(cpu_gpr(3), 0);
}
