use naive_emulator::{
    cpu::{cpu_gpr, cpu_run},
    loader::naive_load,
};

fn loader() {
    let img = [
        // addi, R(1) = R(0) + 0
        0b00000000000_00000_00000_00001_000010,
        // addi, R(2) = R(0) + 10
        0b00000000000_01010_00000_00010_000010,
        // addi, R(3) = R(0) + 0
        0b00000000000_00000_00000_00011_000010,
        // addi, R(3) = R(3) + 1
        0b00000000000_00001_00011_00011_000010,
        // add, R(1) = R(1) + R(3)
        0b00000000000_00011_00001_00001_000001,
        // bne, if (R(3) != R(2)) pc += -0x8
        0b11111111111_00010_00011_11000_000011,
        // halt
        0b00000000000_00000_00000_00000_000000,
    ];
    naive_load(&img);
}

#[test]
fn test_add_from_1_to_10() {
    loader();
    cpu_run(usize::MAX);
    assert_eq!(cpu_gpr(2), 10);
    assert_eq!(cpu_gpr(3), 10);
    assert_eq!(cpu_gpr(1), 55);
}
