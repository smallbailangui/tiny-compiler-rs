mod lab1;

use lab1::category::LexemeCategory;
use lab1::{
    closure, create_tiny_lexical_dfa, generateBasicNFA, product, range, reset_global_tables, union,
};

fn main() {
    println!("========== 测试任务 1: 验证正则表达式 (a|b)*abb ==========");
    test_regex_abb();

    println!("\n========== 测试任务 2: TINY 语言词法分析验证 ==========");
    test_tiny_compiler();
}

/// 任务 1：测试 (a|b)*abb
fn test_regex_abb() {
    reset_global_tables();

    // 1. 定义字符集
    let char_a = range('a', 'a');
    let char_b = range('b', 'b');

    // 2. 构造基础 NFA
    let nfa_a1 = generateBasicNFA("CHAR", char_a, None);
    let nfa_b1 = generateBasicNFA("CHAR", char_b, None);

    // 3. 构造 (a|b)
    let nfa_a_or_b = union(nfa_a1, nfa_b1);

    // 4. 构造 (a|b)*
    let nfa_closure = closure(nfa_a_or_b);

    // 5. 构造 abb 的基础 NFA
    let nfa_a2 = generateBasicNFA("CHAR", char_a, None);
    let nfa_b2 = generateBasicNFA("CHAR", char_b, None);
    // 最后一个状态赋予一个类别以便识别
    let nfa_b3 = generateBasicNFA("CHAR", char_b, Some(LexemeCategory::ID)); 

    // 6. 串联 (a|b)* 和 a, b, b
    let step1 = product(nfa_closure, nfa_a2);
    let step2 = product(step1, nfa_b2);
    let final_nfa = product(step2, nfa_b3);

    // 7. NFA 转 DFA
    let dfa = final_nfa.NFA_to_DFA();

    // 8. 测试字符串
    let test_cases = vec!["abb", "ababb", "bbbaabb", "aba", "bba"];
    for tc in test_cases {
        let is_match = dfa.get_lexeme_category(tc).is_some();
        println!("测试字符串 {:<10} -> 匹配结果: {}", tc, is_match);
    }
}

/// 任务 2：验证 TINY 语言
fn test_tiny_compiler() {
    reset_global_tables();
    
    let dfa = create_tiny_lexical_dfa();
    
    // 使用一段相对完整的 TINY 代码来展示你的词法分析器有多强大
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
        // 打印出识别到的 Token，你会发现空格和注释都被完美过滤了
        println!("  {:?}", token);
    }
}