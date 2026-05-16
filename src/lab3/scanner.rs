#![allow(non_snake_case)]
#![allow(dead_code)]

use std::fs;
use crate::lab1::{
    category::LexemeCategory,
    graph::Graph,
    token::Token,
    reset_global_tables,
};

/// 扫描结果
#[derive(Debug)]
pub struct ScanResult {
    pub tokens: Vec<Token>,
    pub errors: Vec<String>,
}

/// 使用 lab1 预构建的 TINY 词法 DFA 扫描源程序源码
pub fn scan_with_builtin_dfa(source_text: &str) -> ScanResult {
    reset_global_tables();
    let dfa = crate::lab1::create_tiny_lexical_dfa();
    scan_with_dfa(&dfa, source_text)
}

/// 使用给定的 DFA 扫描源程序源码
pub fn scan_with_dfa(dfa: &Graph, source_text: &str) -> ScanResult {
    let tokens = dfa.long_text_search(source_text);
    ScanResult {
        tokens,
        errors: Vec::new(),
    }
}

/// 从文件读取源程序并用内置 DFA 扫描
pub fn scan_file(source_path: &str) -> ScanResult {
    let source_text = fs::read_to_string(source_path)
        .unwrap_or_else(|e| panic!("无法读取源程序文件 {}: {}", source_path, e));
    scan_with_builtin_dfa(&source_text)
}

/// 过滤出有效 token（排除注释和空白）
pub fn filter_meaningful(tokens: &[Token]) -> Vec<&Token> {
    tokens
        .iter()
        .filter(|t| {
            t.lexeme_category != LexemeCategory::SPACE_CONST
                && t.lexeme_category != LexemeCategory::NOTE
        })
        .collect()
}

/// 将 Token 序列格式化输出
pub fn format_tokens(tokens: &[Token]) -> String {
    let mut output = String::new();
    for token in tokens {
        let cat = format!("{:?}", token.lexeme_category);
        let content = if let Some(val) = token.value {
            val.to_string()
        } else if let Some(ref ident) = token.identify {
            ident.clone()
        } else {
            "<none>".to_string()
        };
        output.push_str(&format!("  {:<22} | {}\n", cat, content));
    }
    output
}