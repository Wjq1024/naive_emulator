use naive_emulator::{
    cpu::{cpu_gpr, cpu_run},
    loader::load_asm_file,
};

#[test]
fn test_sum_asm() {
    load_asm_file("sum.asm");
    cpu_run(usize::MAX);
    assert_eq!(cpu_gpr(2), 10);
    assert_eq!(cpu_gpr(3), 10);
    assert_eq!(cpu_gpr(1), 55); //1+...+10=55
}