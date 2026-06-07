#[path = "../lab1/mod.rs"]
mod lab1;

use lab1::category::LexemeCategory;
use lab1::{
    closure, create_tiny_lexical_dfa, difference_charsets, generateBasicNFA, product, range,
    reset_global_tables, show_char_set_table, union, union_charsets,
};

fn main() {
    println!("========== 测试任务 0: 验证字符集运算 ==========");
    test_char_sets();

    println!("\n========== 测试任务 1: 验证正则表达式 (a|b)*abb ==========");
    test_regex_abb();

    println!("\n========== 测试任务 2: TINY 语言词法分析验证 ==========");
    test_tiny_compiler();
}

fn test_char_sets() {
    reset_global_tables();

    let set_a_d = range('a', 'd');
    let set_c_f = range('c', 'f');

    println!("集合 1: [a-d], ID: {}", set_a_d);
    println!("集合 2: [c-f], ID: {}", set_c_f);

    let union_set = union_charsets(set_a_d, set_c_f);
    println!("[a-d] U [c-f], 新 ID: {}", union_set);

    let diff_set = difference_charsets(set_a_d, set_c_f);
    println!("[a-d] - [c-f], 新 ID: {}", diff_set);

    println!("\n当前系统内全局字符集分布:");
    show_char_set_table();
}

fn test_regex_abb() {
    reset_global_tables();

    let char_a = range('a', 'a');
    let char_b = range('b', 'b');

    let nfa_a1 = generateBasicNFA("CHAR", char_a, None);
    let nfa_b1 = generateBasicNFA("CHAR", char_b, None);

    let nfa_a_or_b = union(nfa_a1, nfa_b1);
    let nfa_closure = closure(nfa_a_or_b);

    let nfa_a2 = generateBasicNFA("CHAR", char_a, None);
    let nfa_b2 = generateBasicNFA("CHAR", char_b, None);
    let nfa_b3 = generateBasicNFA("CHAR", char_b, Some(LexemeCategory::ID));

    let step1 = product(nfa_closure, nfa_a2);
    let step2 = product(step1, nfa_b2);
    let final_nfa = product(step2, nfa_b3).minimize_nfa();

    let dfa = final_nfa.NFA_to_DFA();
    let test_cases = vec!["abb", "ababb", "bbbaabb", "aba", "bba", "12.323"];
    for tc in test_cases {
        let is_match = dfa.get_lexeme_category(tc).is_some();
        println!("测试字符串 {:<10} -> 匹配结果: {}", tc, is_match);
    }
}

fn test_tiny_compiler() {
    reset_global_tables();
    let dfa = create_tiny_lexical_dfa();

    let sample = r#"
        { Sample program in TINY language with real numbers }
        read x;
        if 0.5 < x then
            fact := 1.0;
            rate := 1.2e-3;
            max_val := 3E+5;
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