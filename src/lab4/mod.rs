#![allow(dead_code)]

pub mod ast;
pub mod codegen;
pub mod error;
pub mod parser;
pub mod scanner;

use std::{
    path::Path,
    sync::Mutex,
};

use error::CompileError;

/// 全局测试互斥锁，用于序列化依赖全局状态的测试。
static TEST_MUTEX: Mutex<()> = Mutex::new(());

/// 获取全局测试锁。
/// 调用者应在测试函数开头获取此锁，确保依赖全局可变状态的测试安全运行。
pub fn global_test_lock() -> std::sync::MutexGuard<'static, ()> {
    TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
}

/// 编译 TINY 源字符串，返回 TM 汇编文本。
pub fn compile_str(source: &str) -> Result<String, CompileError> {
    let tokens = scanner::scan(source)?;
    let program = parser::parse(&tokens)?;
    codegen::generate_tm(&program)
}

/// 读取 TINY 源文件，写入 TM 输出文件。
pub fn compile_file(input: &Path, output: &Path) -> Result<(), CompileError> {
    let source = std::fs::read_to_string(input)
        .map_err(|e| CompileError::Io(format!("读取失败 {}: {e}", input.display())))?;
    let tm = compile_str(&source)?;
    std::fs::write(output, tm)
        .map_err(|e| CompileError::Io(format!("写入失败 {}: {e}", output.display())))?;
    Ok(())
}

/// 运行实验 4 的内置测试：编译 sample.tny → sample.tm。
pub fn run_lab4_test() -> Result<(), CompileError> {
    println!("实验 4：TINY 语言编译器\n");

    let samples = [
        ("sample.tny", "sample.tm", "阶乘计算"),
        ("sample2.tny", "sample2.tm", "两数相加"),
        ("sample3.tny", "sample3.tm", "if-else 算术"),
    ];

    for (input_name, output_name, desc) in &samples {
        let input = Path::new("src/lab4").join(input_name);
        let output = Path::new("src/lab4").join(output_name);

        println!("--- {desc} ({input_name}) ---");
        println!("  输入文件: {}", input.display());
        compile_file(&input, &output)?;
        println!("  输出文件: {}", output.display());
        println!("  编译成功！");

        let tm_code = std::fs::read_to_string(&output)
            .map_err(|e| CompileError::Io(format!("无法读取输出的 tm 文件: {e}")))?;
        let total = tm_code.lines().count();
        println!("  TM 代码共 {} 行", total);
        for line in tm_code.lines().take(8) {
            println!("    {line}");
        }
        if total > 8 {
            println!("    ... (省略其余行)");
        }
        println!();
    }

    println!("========== 全部 3 个样例编译成功 ==========");
    Ok(())
}
