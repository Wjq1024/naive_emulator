use naive_emulator::{
    cpu::{cpu_gpr, cpu_run, CPU},
    loader::naive_load,
};

fn loader() {
    let img = [
        // addi, R(1) = R(0) + 0
        // addi x1, x0, 0
        // 0x80000000
        0b00000000000_00000_00000_00001_000010,
        // addi, R(2) = R(0) + 10
        // addi x2, x0, 10
        // 0x80000004
        0b00000000000_01010_00000_00010_000010,
        // addi, R(3) = R(0) + 0
        // addi x3, x0, 0
        // 0x80000008
        0b00000000000_00000_00000_00011_000010,
        // addi, R(3) = R(3) + 1j
        // addi x3, x3, 1
        // 0x8000000c
        0b00000000000_00001_00011_00011_000010,
        // add, R(1) = R(1) + R(3)
        // addi x1, x1, x3
        // 0x80000010
        0b00000000000_00011_00001_00001_000001,
        // bne, if (R(3) != R(2)) pc += -0x8
        // bne x3, x2, -8
        // 0x80000014lea
        0b11111111111_00010_00011_11000_000011,
        // halt
        // halt
        // 0x80000018
        0b00000000000_00000_00000_00000_000000,
    ];
    naive_load(&img);
}

#[test]
fn test_sum() {
    loader();
    cpu_run(usize::MAX);
    println!("{:?}", CPU.exclusive_access());

    assert_eq!(cpu_gpr(2), 10);
    assert_eq!(cpu_gpr(3), 10);
    assert_eq!(cpu_gpr(1), 55);
}
