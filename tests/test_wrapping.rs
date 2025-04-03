use naive_emulator::{
    common::Word,
    cpu::{Cpu, ExecuteState, signal::ALUOperation},
};

#[test]
fn test_wrapping_operations() {
    // 创建一个新的执行状态
    let mut exec_state = ExecuteState::new(0x8000_0000.into());

    // 测试 wrapping_add 溢出
    // 最大的 Word 值
    let max_word: Word = Word::MAX;
    // 压入两个最大的 Word 值到栈中
    exec_state.stack.push(max_word);
    exec_state.stack.push(max_word);
    // 执行加法操作
    ALUOperation::Plus.exec_alu_operation(&mut exec_state);
    // 预期结果：溢出后回绕到 0xFFFFFFFE
    let result_add = exec_state.stack.pop().unwrap();
    assert_eq!(result_add, 0xFFFFFFFE);

    // 重置执行状态的栈
    exec_state.stack.clear();

    // 测试 wrapping_mul 溢出
    // 一个较大的 Word 值
    let large_word: Word = 0xFFFF_FFFF;
    // 压入两个较大的 Word 值到栈中
    exec_state.stack.push(large_word);
    exec_state.stack.push(large_word);
    // 执行乘法操作
    ALUOperation::Multiply.exec_alu_operation(&mut exec_state);
    // 预期结果：溢出后回绕到 1
    let result_mul = exec_state.stack.pop().unwrap();
    assert_eq!(result_mul, 1);
}