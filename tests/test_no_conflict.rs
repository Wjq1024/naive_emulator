use naive_emulator::{
    cpu::{cpu_gpr, cpu_run, CPU},
    loader::naive_load,
};

fn loader() {
    let img = [
        0b00000000000_00000_00000_00001_000010,
        0b00000000000_01010_00000_00010_000010,
        0b00000000000_00101_00000_00011_000010,
        0b00000000000_00110_00000_00100_000010,
        0b00000000000_00011_00000_00101_000010,
        0b00000000000_00010_00001_00110_000001,
        0b00000000000_00000_00000_00000_000000,
    ];
    naive_load(&img);
}

#[test]
fn test_no_conflict() {
    loader();
    cpu_run(usize::MAX);
    assert_eq!(cpu_gpr(1), 0);
    assert_eq!(cpu_gpr(2), 10);
    assert_eq!(cpu_gpr(3), 5);
    assert_eq!(cpu_gpr(4), 6);
    assert_eq!(cpu_gpr(5), 3);
    assert_eq!(cpu_gpr(6), 10);
    println!("{:?}", CPU.exclusive_access());
}
