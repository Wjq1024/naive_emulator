use naive_emulator::{
    cpu::{cpu_gpr, cpu_run},
    loader::binary_file_load,
};

fn test_binary_files() {
    binary_file_load("tests/data/sum.o".to_owned());
    cpu_run(usize::MAX);
    assert_eq!(cpu_gpr(2), 10);
    assert_eq!(cpu_gpr(3), 10);
    assert_eq!(cpu_gpr(1), 55);
}
