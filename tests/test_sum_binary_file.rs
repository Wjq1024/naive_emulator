use naive_emulator::{
    cpu::{cpu_gpr, cpu_run},
    loader::binary_file_load,
};

#[test]
fn test_add_from_1_to_10() {
    binary_file_load("/home/lijn/naive_emulator/test_obj/sum.o".to_owned());
    cpu_run(usize::MAX);
    assert_eq!(cpu_gpr(2), 10);
    assert_eq!(cpu_gpr(3), 10);
    assert_eq!(cpu_gpr(1), 55);
}
