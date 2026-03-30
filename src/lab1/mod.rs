#![allow(non_snake_case)]

pub mod char_set;
pub mod edge;
pub mod graph;
pub mod regular_expression;
pub mod state;
pub mod token;

pub use char_set::{
    difference_charset_char,
    difference_charsets,
    range,
    show_char_set_table,
    union_chars,
    union_charsets,
    union_charset_char,
};
pub use edge::Edge;
pub use graph::{closure, generateBasicNFA, plusClosure, product, union, zeroOrOne, Graph};
pub use regular_expression::{add_regular_expression, clear_regular_table, regular_table_snapshot, RegularExpression};
pub use state::State;
pub use token::Token;

use char_set::reset_char_sets;
use graph::reset_graph_counter;

/// 构建 TINY 语言的 DFA，涵盖关键字、运算符、identifier、number、note 以及空白
pub fn create_tiny_lexical_dfa() -> Graph {
    reset_global_tables();

    // 基础字符集合
    let lower = range('a', 'z');
    let upper = range('A', 'Z');
    let digit = range('0', '9');
    let letters = union_charsets(lower, upper);
    let letter_digit = union_charsets(letters, digit);
    let single_letters = create_single_letter_sets();

    // 运算符和分隔符字符集
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

    // 注释内部允许的字符集合
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

    // 关键字 NFA
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

    // 运算符与界符 NFA
    let graph_assign = product(
        generateBasicNFA("CHAR", char_set_colon, ""),
        generateBasicNFA("CHAR", char_set_equal, ":="),
    );
    let operator_graph = union_many(vec![
        generateBasicNFA("CHAR", char_set_add, "+"),
        generateBasicNFA("CHAR", char_set_sub, "-"),
        generateBasicNFA("CHAR", char_set_mul, "*"),
        generateBasicNFA("CHAR", char_set_div, "/"),
        generateBasicNFA("CHAR", char_set_equal, "="),
        generateBasicNFA("CHAR", char_set_less, "<"),
        generateBasicNFA("CHAR", char_set_left_paren, "("),
        generateBasicNFA("CHAR", char_set_right_paren, ")"),
        generateBasicNFA("CHAR", char_set_semicolon, ";"),
        graph_assign,
    ]);

    // 标识符 (letter letterOrDigit*)
    let graph_identifier = product(
        generateBasicNFA("CHARSET", letters, ""),
        closure(generateBasicNFA("CHARSET", letter_digit, "identifier")),
    );

    // 注释 { ... }
    let graph_note = product(
        product(
            generateBasicNFA("CHAR", char_set_left_note, ""),
            closure(generateBasicNFA("CHARSET", note_char_set, "")),
        ),
        generateBasicNFA("CHAR", char_set_right_note, "NOTE"),
    );

    // 数字与空白
    let graph_number = plusClosure(generateBasicNFA("CHARSET", digit, "Number"));
    let graph_blank = generateBasicNFA("CHAR", char_set_space, "BLANK");

    let lexical_graph = union_many(vec![
        keyword_graph,
        operator_graph,
        graph_identifier,
        graph_note,
        graph_number,
        graph_blank,
    ]);

    lexical_graph.NFA_to_DFA()
}

pub fn reset_global_tables() {
    reset_char_sets();
    reset_graph_counter();
    clear_regular_table();
}

fn create_single_letter_sets() -> Vec<i32> {
    ('a'..='z').map(|ch| range(ch, ch)).collect()
}

fn char_set_for(single_letters: &[i32], ch: char) -> i32 {
    let idx = (ch as u8 - b'a') as usize;
    single_letters[idx]
}

fn build_keyword_graph(word: &str, single_letters: &[i32]) -> Graph {
    let chars: Vec<char> = word.chars().collect();
    let first = *chars.first().expect("keyword 不能为空");
    let mut graph = generateBasicNFA("CHAR", char_set_for(single_letters, first), "");
    if chars.len() == 1 {
        graph.pStateTable.last_mut().unwrap().LexemeCategory = "KEYWORD".to_string();
        return graph;
    }
    for (index, ch) in chars.iter().enumerate().skip(1) {
        let is_last = index == chars.len() - 1;
        let category = if is_last { "KEYWORD" } else { "" };
        graph = product(graph, generateBasicNFA("CHAR", char_set_for(single_letters, *ch), category));
    }
    graph
}

fn union_many(graphs: Vec<Graph>) -> Graph {
    let mut iter = graphs.into_iter();
    let mut acc = iter.next().expect("union_many 至少需要一个图");
    for graph in iter {
        acc = union(acc, graph);
    }
    acc
}
