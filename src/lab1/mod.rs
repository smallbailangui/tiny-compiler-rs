#![allow(non_snake_case)]
#![allow(unused_imports)]

pub mod category;
pub mod char_set;
pub mod edge;
pub mod graph;
pub mod regular_expression;
pub mod state;
pub mod token;

pub use category::LexemeCategory;
pub use char_set::{
    difference_charset_char, difference_charsets, range, show_char_set_table, union_chars,
    union_charset_char, union_charsets,
};
pub use edge::Edge;
pub use graph::{Graph, closure, generateBasicNFA, plusClosure, product, union, zeroOrOne};
pub use regular_expression::{
    RegularExpression, add_regular_expression, clear_regular_table, regular_table_snapshot,
};
pub use state::State;
pub use token::Token;

use char_set::reset_char_sets;
use graph::reset_graph_counter;

/// 构建 TINY 语言词法 DFA。
///
/// 总体步骤：
/// 1. 定义基础字符集。
/// 2. 构建关键字、运算符、标识符、注释、数字、空白等 NFA。
/// 3. 通过 `union_many` 合并为总 NFA。
/// 4. 最简化 NFA 后，再做子集构造得到 DFA。
pub fn create_tiny_lexical_dfa() -> Graph {
    // 每次构建前先清理全局表，避免脏数据影响。
    reset_global_tables();

    // 基础字符集。
    let lower = range('a', 'z');
    let upper = range('A', 'Z');
    let digit = range('0', '9');
    let letters = union_charsets(lower, upper);
    let letter_digit = union_charsets(letters, digit);
    let single_letters = create_single_letter_sets();

    // 运算符/分隔符字符集。
    let char_set_add = range('+', '+');
    let char_set_sub = range('-', '-');
    let char_set_mul = range('*', '*');
    let char_set_div = range('/', '/');
    let char_set_equal = range('=', '=');
    let char_set_less = range('<', '<');
    let char_set_left_paren = range('(', '(');
    let char_set_right_paren = range(')', ')');
    let char_set_semicolon = range(';', ';');
    let char_set_colon = range(':', ':');
    let char_set_space = range(' ', ' ');
    let char_set_left_note = range('{', '{');
    let char_set_right_note = range('}', '}');

    // 注释内容允许字符池（不包含右花括号，避免提前吞掉结束符）。
    let mut note_pool = letters;
    for extra in [
        char_set_add,
        char_set_sub,
        char_set_mul,
        char_set_div,
        char_set_equal,
        char_set_less,
        char_set_left_paren,
        char_set_right_paren,
        char_set_semicolon,
        char_set_colon,
        char_set_space,
    ] {
        note_pool = union_charsets(note_pool, extra);
    }
    let note_char_set = union_charsets(note_pool, letter_digit);

    // 关键字 NFA。
    let keyword_graph = union_many(vec![
        build_keyword_graph("if", &single_letters),
        build_keyword_graph("then", &single_letters),
        build_keyword_graph("else", &single_letters),
        build_keyword_graph("end", &single_letters),
        build_keyword_graph("repeat", &single_letters),
        build_keyword_graph("until", &single_letters),
        build_keyword_graph("read", &single_letters),
        build_keyword_graph("write", &single_letters),
    ]);

    // 运算符与界符 NFA（`:=` 用串联实现）。
    let graph_assign = product(
        generateBasicNFA("CHAR", char_set_colon, None),
        generateBasicNFA(
            "CHAR",
            char_set_equal,
            Some(LexemeCategory::ASSIGN_OPERATOR),
        ),
    );
    let operator_graph = union_many(vec![
        generateBasicNFA("CHAR", char_set_add, Some(LexemeCategory::NUMERIC_OPERATOR)),
        generateBasicNFA("CHAR", char_set_sub, Some(LexemeCategory::NUMERIC_OPERATOR)),
        generateBasicNFA("CHAR", char_set_mul, Some(LexemeCategory::NUMERIC_OPERATOR)),
        generateBasicNFA("CHAR", char_set_div, Some(LexemeCategory::NUMERIC_OPERATOR)),
        generateBasicNFA(
            "CHAR",
            char_set_equal,
            Some(LexemeCategory::COMPARE_OPERATOR),
        ),
        generateBasicNFA(
            "CHAR",
            char_set_less,
            Some(LexemeCategory::COMPARE_OPERATOR),
        ),
        generateBasicNFA(
            "CHAR",
            char_set_left_paren,
            Some(LexemeCategory::LOGIC_OPERATOR),
        ),
        generateBasicNFA(
            "CHAR",
            char_set_right_paren,
            Some(LexemeCategory::LOGIC_OPERATOR),
        ),
        generateBasicNFA(
            "CHAR",
            char_set_semicolon,
            Some(LexemeCategory::LOGIC_OPERATOR),
        ),
        graph_assign,
    ]);

    // 标识符：letter (letter|digit)*
    let graph_identifier = product(
        generateBasicNFA("CHARSET", letters, None),
        closure(generateBasicNFA(
            "CHARSET",
            letter_digit,
            Some(LexemeCategory::ID),
        )),
    );

    // 注释：{ ... }
    let graph_note = product(
        product(
            generateBasicNFA("CHAR", char_set_left_note, None),
            closure(generateBasicNFA("CHARSET", note_char_set, None)),
        ),
        generateBasicNFA("CHAR", char_set_right_note, Some(LexemeCategory::NOTE)),
    );

    // 数字（当前仅整数）与空白。
    let graph_number = plusClosure(generateBasicNFA(
        "CHARSET",
        digit,
        Some(LexemeCategory::INTEGER_CONST),
    ));
    let graph_blank = generateBasicNFA("CHAR", char_set_space, Some(LexemeCategory::SPACE_CONST));

    let lexical_graph = union_many(vec![
        keyword_graph,
        operator_graph,
        graph_identifier,
        graph_note,
        graph_number,
        graph_blank,
    ]);

    lexical_graph.minimize_nfa().NFA_to_DFA()
}

/// 重置词法构造使用到的所有全局表。
pub fn reset_global_tables() {
    reset_char_sets();
    reset_graph_counter();
    clear_regular_table();
}

/// 构建 `a..z` 的单字符字符集列表。
fn create_single_letter_sets() -> Vec<i32> {
    ('a'..='z').map(|ch| range(ch, ch)).collect()
}

/// 把小写字母映射到 `single_letters` 中对应的字符集 id。
fn char_set_for(single_letters: &[i32], ch: char) -> i32 {
    let idx = (ch as u8 - b'a') as usize;
    single_letters[idx]
}

/// 把一个关键字构造成串联 NFA。
fn build_keyword_graph(word: &str, single_letters: &[i32]) -> Graph {
    let chars: Vec<char> = word.chars().collect();
    let first = *chars.first().expect("keyword 不能为空");

    // 先用首字符建立初始 NFA。
    let mut graph = generateBasicNFA("CHAR", char_set_for(single_letters, first), None);

    // 单字母关键字特殊处理：直接标记终态类别。
    if chars.len() == 1 {
        graph.pStateTable.last_mut().unwrap().LexemeCategory = Some(LexemeCategory::KEYWORD);
        return graph;
    }

    // 其余字符依次串联，最后一个字符落在 KEYWORD 终态。
    for (index, ch) in chars.iter().enumerate().skip(1) {
        let is_last = index == chars.len() - 1;
        let category = if is_last {
            Some(LexemeCategory::KEYWORD)
        } else {
            None
        };
        graph = product(
            graph,
            generateBasicNFA("CHAR", char_set_for(single_letters, *ch), category),
        );
    }

    graph
}

/// 将多个图按顺序做并运算。
fn union_many(graphs: Vec<Graph>) -> Graph {
    let mut iter = graphs.into_iter();
    let mut acc = iter.next().expect("union_many 至少需要一个图");
    for graph in iter {
        acc = union(acc, graph);
    }
    acc
}

