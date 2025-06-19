use naive_emulator::{
    cpu::{cpu_gpr, cpu_run, CPU},
    loader::naive_load,
};

fn loader() {
    let img = [
        0b00000000000_00111_00000_00001_000010,
        0b00000000000_01010_00000_00010_000010,
        0b00000000000_00001_00010_00011_000001,
        0b00000000000_00000_00000_00000_000000
    ];
    naive_load(&img);
}

#[test]
fn test_conflict1() {
    loader();
    cpu_run(usize::MAX);
    assert_eq!(cpu_gpr(1), 7);
    assert_eq!(cpu_gpr(2), 10);
    assert_eq!(cpu_gpr(3), 17);
    // println!("{:?}", CPU.exclusive_access());
}
