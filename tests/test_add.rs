use naive_emulator::{
    common::Word,
    cpu::{cpu_gpr, cpu_run},
    loader::naive_load,
};

fn loader() {
    let img: [Word; 4] = [
        0b00000000000_10001_00000_00001_000010,
        0b00000000000_10010_00000_00010_000010,
        0b00000000001_00010_00001_00011_000001,
        0b00000000000_00000_00000_00000_000000,
    ];
    naive_load(&img);
}

#[test]
fn test_add() {
    loader();
    cpu_run(usize::MAX);
    assert_eq!(cpu_gpr(3), 35);
}
