#![allow(dead_code)]

use crate::lab1::{create_tiny_lexical_dfa, LexemeCategory};

use super::error::CompileError;

/// TINY 语言的 Token 类型。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TinyTokenKind {
    // 关键字
    If,
    Then,
    Else,
    End,
    Repeat,
    Until,
    Read,
    Write,

    // 符号
    Assign, // :=
    Lt,
    Eq,
    Plus,
    Minus,
    Times,
    Over,
    LParen,
    RParen,
    Semi,

    // 字面量
    Id(String),
    Num(i64),

    // 文件结束
    Eof,
}

/// 词法单元。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TinyToken {
    pub kind: TinyTokenKind,
    pub lexeme: String,
}

/// 对 TINY 源程序进行词法分析，返回 token 序列。
///
/// 复用实验 1 构造的 TINY 词法 DFA。
/// 由于该 DFA 仅覆盖 `' '` 字符的空白处理，额外的 `\r`、`\n`、`\t` 会被归一化为空格。
pub fn scan(source: &str) -> Result<Vec<TinyToken>, CompileError> {
    // 将 Windows / Unix 换行符和制表符统一替换为空格，
    // 避免 token 粘连。
    let normalized = source.replace(['\r', '\n', '\t'], " ");

    let dfa = create_tiny_lexical_dfa();
    let raw = dfa.long_text_search(&normalized);

    let mut out = Vec::with_capacity(raw.len() + 1);

    for t in raw {
        let token = match t.lexeme_category {
            LexemeCategory::KEYWORD => {
                let kw = t.identify.clone().unwrap_or_default();
                let kind = match kw.as_str() {
                    "if" => TinyTokenKind::If,
                    "then" => TinyTokenKind::Then,
                    "else" => TinyTokenKind::Else,
                    "end" => TinyTokenKind::End,
                    "repeat" => TinyTokenKind::Repeat,
                    "until" => TinyTokenKind::Until,
                    "read" => TinyTokenKind::Read,
                    "write" => TinyTokenKind::Write,
                    _ => return Err(CompileError::Lex(format!("未知关键字: {kw}"))),
                };
                TinyToken {
                    kind,
                    lexeme: kw,
                }
            }
            LexemeCategory::ID => {
                let name = t.identify.clone().unwrap_or_default();
                // 兜底：上游 DFA 可能把保留字错误归入 ID 类别，这里修正。
                let kind = match name.as_str() {
                    "if" => TinyTokenKind::If,
                    "then" => TinyTokenKind::Then,
                    "else" => TinyTokenKind::Else,
                    "end" => TinyTokenKind::End,
                    "repeat" => TinyTokenKind::Repeat,
                    "until" => TinyTokenKind::Until,
                    "read" => TinyTokenKind::Read,
                    "write" => TinyTokenKind::Write,
                    _ => TinyTokenKind::Id(name.clone()),
                };
                TinyToken { kind, lexeme: name }
            }
            LexemeCategory::INTEGER_CONST => {
                let v = t.value.ok_or_else(|| {
                    CompileError::Lex("整数 token 缺少 value 字段".to_string())
                })?;
                TinyToken {
                    kind: TinyTokenKind::Num(v),
                    lexeme: v.to_string(),
                }
            }
            LexemeCategory::FLOAT_CONST | LexemeCategory::SCIENTIFIC_CONST => {
                let s = t.identify.unwrap_or_else(|| "<float>".to_string());
                return Err(CompileError::Semantic(format!(
                    "当前实验四仅实现整数运算；不支持浮点常量: {s}"
                )));
            }
            LexemeCategory::ASSIGN_OPERATOR => TinyToken {
                kind: TinyTokenKind::Assign,
                lexeme: ":=".to_string(),
            },
            LexemeCategory::COMPARE_OPERATOR => {
                let op = t.identify.unwrap_or_default();
                let kind = match op.as_str() {
                    "<" => TinyTokenKind::Lt,
                    "=" => TinyTokenKind::Eq,
                    _ => return Err(CompileError::Lex(format!("未知比较运算符: {op}"))),
                };
                TinyToken { kind, lexeme: op }
            }
            LexemeCategory::NUMERIC_OPERATOR => {
                let op = t.identify.unwrap_or_default();
                let kind = match op.as_str() {
                    "+" => TinyTokenKind::Plus,
                    "-" => TinyTokenKind::Minus,
                    "*" => TinyTokenKind::Times,
                    "/" => TinyTokenKind::Over,
                    _ => return Err(CompileError::Lex(format!("未知算术运算符: {op}"))),
                };
                TinyToken { kind, lexeme: op }
            }
            LexemeCategory::LOGIC_OPERATOR => {
                let op = t.identify.unwrap_or_default();
                let kind = match op.as_str() {
                    "(" => TinyTokenKind::LParen,
                    ")" => TinyTokenKind::RParen,
                    ";" => TinyTokenKind::Semi,
                    _ => return Err(CompileError::Lex(format!("未知界符: {op}"))),
                };
                TinyToken { kind, lexeme: op }
            }
            LexemeCategory::NOTE
            | LexemeCategory::SPACE_CONST
            | LexemeCategory::STRING_CONST => {
                // 注释、空白、字符串均不是 TINY 文法的组成部分，直接丢弃。
                continue;
            }
        };

        out.push(token);
    }

    out.push(TinyToken {
        kind: TinyTokenKind::Eof,
        lexeme: "<eof>".to_string(),
    });

    Ok(out)
}