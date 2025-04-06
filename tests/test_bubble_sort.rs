use naive_emulator::{
    common::PAddr,
    cpu::{CPU, cpu_gpr, cpu_pc, cpu_run},
    loader::naive_load,
    memory::paddr_read,
};

fn loader() {
    let img = [
        // Instructions:
        // lui x7, 0x8000
        0b10000000000_00000_00000_00111_000101,
        // addi x7, x7, 0x004c
        0b00000000010_01100_00111_00111_000010,
        // lw x1, -4(x7)
        0b11111111111_11100_00111_00001_000110,
        // slli x1, x1, 2
        0b00000000000_00010_00001_00001_001001,
        // addi x2, x0, 4
        0b00000000000_00100_00000_00010_000010,
        // add x3, x0, 0
        0b00000000000_00000_00000_00011_000001,
        // sub x8, x1, x2
        0b00000000000_00010_00001_01000_001010,
        // add x4, x3, x7
        0b00000000000_00111_00011_00100_000001,
        // lw x5, 0(x4)
        0b00000000000_00000_00100_00101_000110,
        // lw x6, +4(x4)
        0b00000000000_00100_00100_00110_000110,
        // blt x5, x6, +12
        0b00000000000_00110_00101_01100_001000,
        // sw x6, 0(x4)
        0b00000000000_00110_00100_00000_000111,
        // sw x5, +4(x4)
        0b00000000000_00101_00100_00100_000111,
        // addi x3, x3, 4
        0b00000000000_00100_00011_00011_000010,
        // bne x3, x8, -28
        0b11111111111_00011_01000_00100_000011,
        // addi x2, x2, 4
        0b00000000000_00100_00010_00010_000010,
        // bne x2, x1, -44
        0b11111111110_00010_00001_10100_000011,
        // halt
        0b00000000000_00000_00000_00000_000000,
        // Datas:
        6,
        4,
        9,
        3,
        1,
        8,
        u32::MAX,
    ];
    naive_load(&img);
}

#[test]
fn test_bubble_sort() {
    loader();
    // for _ in 0..100 {
    //     println!("0x{:08x}", cpu_pc().0);
    //     cpu_run(1);
    //     println!("{:?}", CPU.exclusive_access());
    //     print!("mem: ");
    //     let mut addr = 0x8000_0048;
    //     while addr <= 0x8000_005C {
    //         print!("{} ", paddr_read(PAddr(addr), 4));
    //         addr += 4;
    //     }
    //     println!();
    // }
    // print!("mem: ");
    // let mut addr = 0x8000_0048;
    // while addr <= 0x8000_005C {
    //     print!("{} ", paddr_read(PAddr(addr), 4));
    //     addr += 4;
    // }
    // println!();
    // println!("{:?}", CPU.exclusive_access());
    cpu_run(usize::MAX);

    assert_eq!(paddr_read(0x8000_004C.into(), 4), u32::MAX);
    assert_eq!(paddr_read(0x8000_0050.into(), 4), 1);
    assert_eq!(paddr_read(0x8000_0054.into(), 4), 3);
    assert_eq!(paddr_read(0x8000_0058.into(), 4), 4);
    assert_eq!(paddr_read(0x8000_005C.into(), 4), 8);
    assert_eq!(paddr_read(0x8000_0060.into(), 4), 9);

    assert_eq!(paddr_read(0x8000_0048.into(), 4), 6);
}
