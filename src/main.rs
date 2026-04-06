mod lab1;

use lab1::category::LexemeCategory;
use lab1::{
    closure, create_tiny_lexical_dfa, generateBasicNFA, product, range, reset_global_tables, union,
};

/// 程序入口：首先运行正则表达式 `(a|b)*abb` 的 NFA 到 DFA 转换示例，
/// 然后运行 TINY 语言的词法分析器示例，解析一段 TINY 代码。
fn main() {
    println!("========== 测试任务 1: 验证正则表达式 (a|b)*abb ==========");
    // 执行正则表达式匹配的测试函数
    test_regex_abb();

    println!("\n========== 测试任务 2: TINY 语言词法分析验证 ==========");
    // 执行 TINY 语言的词法分析测试
    test_tiny_compiler();
}

/// 任务 1：构造 `(a|b)*abb` 的 NFA，最简化后转 DFA，并测试样例字符串。
fn test_regex_abb() {
    // 每次测试前重置全局表，避免上次状态污染。
    reset_global_tables();

    // 1) 定义字符集合。
    let char_a = range('a', 'a');
    let char_b = range('b', 'b');

    // 2) 构造基础 NFA。
    let nfa_a1 = generateBasicNFA("CHAR", char_a, None);
    let nfa_b1 = generateBasicNFA("CHAR", char_b, None);

    // 3) 构造 `(a|b)`。
    let nfa_a_or_b = union(nfa_a1, nfa_b1);

    // 4) 构造 `(a|b)*`。
    let nfa_closure = closure(nfa_a_or_b);

    // 5) 构造 `abb`。
    let nfa_a2 = generateBasicNFA("CHAR", char_a, None);
    let nfa_b2 = generateBasicNFA("CHAR", char_b, None);
    let nfa_b3 = generateBasicNFA("CHAR", char_b, Some(LexemeCategory::ID));

    // 6) 串联为最终 NFA，并执行最简化。
    let step1 = product(nfa_closure, nfa_a2);
    let step2 = product(step1, nfa_b2);
    let final_nfa = product(step2, nfa_b3).minimize_nfa();

    // 7) NFA 转 DFA。
    let dfa = final_nfa.NFA_to_DFA();

    // 8) 验证测试用例。
    let test_cases = vec!["abb", "ababb", "bbbaabb", "aba", "bba"];
    for tc in test_cases {
        let is_match = dfa.get_lexeme_category(tc).is_some();
        println!("测试字符串 {:<10} -> 匹配结果: {}", tc, is_match);
    }
}

/// 任务 2：构造 TINY 词法 DFA，并扫描一段示例程序。
fn test_tiny_compiler() {
    // 重置全局状态后构造词法自动机。
    reset_global_tables();
    let dfa = create_tiny_lexical_dfa();

    // 示例程序。
    let sample = r#"
        { Sample program in TINY language }
        read x;
        if 0 < x then
            fact := 1;
            repeat
                fact := fact * x;
                x := x - 1
            until x = 0;
            write fact
        end
    "#;

    println!("正在分析 TINY 代码: \n{}", sample);
    println!("输出的 Tokens 序列:");

    let tokens = dfa.long_text_search(sample);
    for token in tokens {
        println!("  {:?}", token);
    }
}
