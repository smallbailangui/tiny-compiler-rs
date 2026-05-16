#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::HashMap;
use crate::lab1::{
    category::LexemeCategory,
    char_set::{range, difference_charset_char, difference_charsets, segments_of},
    graph::{generateBasicNFA, union, product, closure, plusClosure, zeroOrOne, Graph},
};

/// 解析出的正则定义
#[derive(Clone, Debug)]
pub struct RegexDef {
    pub name: String,
    pub pattern: String,
    pub is_token: bool,  // 是否带 @ 前缀（即对应的词类名）
}

/// 将正则语言描述的文本解析为定义列表
/// 支持多行定义：以 `|` 开头的行视为前一个定义的续行
pub fn parse_regex_definitions(text: &str) -> Vec<RegexDef> {
    let mut defs = Vec::new();
    let mut current: Option<RegexDef> = None;

    for line in text.lines() {
        let original = line.trim();
        if original.is_empty() {
            continue;
        }
        let line = skip_leading_number(original);

        // 续行：以 `|` 开头，追加到前一个定义的 pattern
        if line.starts_with('|') {
            if let Some(ref mut def) = current {
                let continuation = line[1..].trim();
                def.pattern.push(' ');
                def.pattern.push('|');
                def.pattern.push(' ');
                def.pattern.push_str(continuation);
            }
            continue;
        }

        // 先将前一个定义保存
        if let Some(def) = current.take() {
            defs.push(def);
        }

        // 格式: [@]name -> regex_pattern
        if let Some(def) = parse_one_definition(line) {
            current = Some(def);
        }
    }

    // 保存最后一个定义
    if let Some(def) = current.take() {
        defs.push(def);
    }

    defs
}

/// 跳过行首的 "1 ", "12 " 等编号
fn skip_leading_number(s: &str) -> &str {
    let s = s.trim();
    let start = s.find(|c: char| !c.is_ascii_digit() && c != ' ').unwrap_or(0);
    &s[start..]
}

/// 解析一行定义
fn parse_one_definition(line: &str) -> Option<RegexDef> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    let is_token = line.starts_with('@');
    let working = if is_token { &line[1..] } else { line };

    // 查找 "->" 分隔符
    let arrow_pos = working.find("->")?;
    let name = working[..arrow_pos].trim().to_string();
    let pattern = working[arrow_pos + 2..].trim().to_string();

    if name.is_empty() || pattern.is_empty() {
        return None;
    }

    Some(RegexDef {
        name,
        pattern,
        is_token,
    })
}

/// 正则表达式 AST 节点
#[derive(Clone, Debug)]
enum RegexAST {
    CharSet(i32, Option<char>),  // 字符集 ID 及可选关联字符（单字符时用于范围提取）
    CharLiteral(char),           // 字符字面量
    Name(String),                // 引用其他命名正则
    Union(Box<RegexAST>, Box<RegexAST>),
    Concat(Box<RegexAST>, Box<RegexAST>),
    Star(Box<RegexAST>),
    Plus(Box<RegexAST>),
    Option_(Box<RegexAST>),
    Range(char, char),           // 字符范围运算: 'a'~'z' 或 'a'..'z'
    Difference(Box<RegexAST>, Box<RegexAST>),
}

/// 根据正则定义列表构建最终的 NFA（合并所有带 @ 的定义）
pub fn build_nfa_from_defs(defs: &[RegexDef]) -> (Graph, HashMap<String, LexemeCategory>) {
    let mut env: HashMap<String, Graph> = HashMap::new();
    let mut token_map: HashMap<String, LexemeCategory> = HashMap::new();

    for def in defs {
        let ast = parse_regex_pattern(&def.pattern);
        let nfa = eval_ast(&ast, &env, def.is_token, &def.name);
        env.insert(def.name.clone(), nfa);

        if def.is_token {
            let cat = name_to_category(&def.name);
            token_map.insert(def.name.clone(), cat);
        }
    }

    // 最后一条定义（lexeme）就是最终结果
    // 合并所有带 @ 的定义对应的 NFA
    let final_nfa = if let Some(last) = defs.last() {
        env.get(&last.name).cloned().unwrap_or_else(|| {
            merge_all_token_nfas(&env, &defs, &token_map)
        })
    } else {
        Graph::new(0)
    };

    (final_nfa, token_map)
}

/// 合并所有 token 定义的 NFA
fn merge_all_token_nfas(
    env: &HashMap<String, Graph>,
    defs: &[RegexDef],
    _token_map: &HashMap<String, LexemeCategory>,
) -> Graph {
    let mut token_nfas: Vec<Graph> = Vec::new();
    for def in defs {
        if def.is_token {
            if let Some(g) = env.get(&def.name) {
                token_nfas.push(g.clone());
            }
        }
    }
    if token_nfas.is_empty() {
        return Graph::new(0);
    }
    let mut iter = token_nfas.into_iter();
    let mut acc = iter.next().unwrap();
    for g in iter {
        acc = union(acc, g);
    }
    acc
}

/// 将名字转换为 LexemeCategory
fn name_to_category(name: &str) -> LexemeCategory {
    match name {
        "reserved" | "reserved_word" => LexemeCategory::LOGIC_OPERATOR,
        "cc" | "character_constant" => LexemeCategory::STRING_CONST,
        "space" | "blank" => LexemeCategory::SPACE_CONST,
        "crlf" | "newline" => LexemeCategory::SPACE_CONST,
        "keyword" => LexemeCategory::KEYWORD,
        "numeric_op" => LexemeCategory::NUMERIC_OPERATOR,
        "compare_op" => LexemeCategory::COMPARE_OPERATOR,
        "logic_op" => LexemeCategory::LOGIC_OPERATOR,
        "assign" => LexemeCategory::ASSIGN_OPERATOR,
        "id" => LexemeCategory::ID,
        "integer_const" => LexemeCategory::INTEGER_CONST,
        "note" => LexemeCategory::NOTE,
        _ => LexemeCategory::ID,
    }
}

/// 简单的递归下降解析器：解析正则表达式模式字符串
fn parse_regex_pattern(pattern: &str) -> RegexAST {
    let tokens = tokenize_pattern(pattern);
    let (ast, _) = parse_union(&tokens, 0);
    ast
}

/// 正则模式词法分析：将模式串切分为 token
#[derive(Clone, Debug, PartialEq)]
enum RegexTok {
    LParen,
    RParen,
    Pipe,
    Star,
    Plus,
    Question,
    Tilde,
    Minus,
    Dot,       // 连接运算符 (·)
    RangeOp,   // 范围运算符 (..)
    CharLit(char),
    Name(String),
    EOF,
}

fn tokenize_pattern(pattern: &str) -> Vec<RegexTok> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        match c {
            '(' => tokens.push(RegexTok::LParen),
            ')' => tokens.push(RegexTok::RParen),
            '|' => tokens.push(RegexTok::Pipe),
            '*' => tokens.push(RegexTok::Star),
            '+' => tokens.push(RegexTok::Plus),
            '?' => tokens.push(RegexTok::Question),
            '~' => tokens.push(RegexTok::Tilde),
            '-' => tokens.push(RegexTok::Minus),
            '·' => tokens.push(RegexTok::Dot),
            '.' => {
                // 检查 ".." 范围运算符
                if i + 1 < chars.len() && chars[i + 1] == '.' {
                    tokens.push(RegexTok::RangeOp);
                    i += 1; // 跳过第二个点
                } else {
                    tokens.push(RegexTok::CharLit('.'));
                }
            }
            '\'' => {
                // 字符字面量
                if i + 2 < chars.len() && chars[i + 2] == '\'' {
                    tokens.push(RegexTok::CharLit(chars[i + 1]));
                    i += 2;
                } else if i + 1 < chars.len() {
                    tokens.push(RegexTok::CharLit(chars[i + 1]));
                    i += 1;
                    if i + 1 < chars.len() && chars[i + 1] == '\'' {
                        i += 1;
                    }
                }
            }
            ' ' | '\t' | '\r' | '\n' => {
                // 跳过空白
            }
            _ if c.is_alphanumeric() || c == '_' => {
                // 名字：字母、数字、下划线，以及中文字符
                let start = i;
                while i < chars.len()
                    && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] as u32 > 127)
                {
                    i += 1;
                }
                let name: String = chars[start..i].iter().collect();
                tokens.push(RegexTok::Name(name));
                continue; // 跳过 i += 1
            }
            _ => {
                // 其他字符作为字面量
                tokens.push(RegexTok::CharLit(c));
            }
        }
        i += 1;
    }

    tokens.push(RegexTok::EOF);
    tokens
}

/// 解析并运算 (最低优先级)
fn parse_union(tokens: &[RegexTok], pos: usize) -> (RegexAST, usize) {
    let (mut left, mut pos) = parse_concat(tokens, pos);
    while pos < tokens.len() && tokens[pos] == RegexTok::Pipe {
        pos += 1;
        let (right, new_pos) = parse_concat(tokens, pos);
        left = RegexAST::Union(Box::new(left), Box::new(right));
        pos = new_pos;
    }
    (left, pos)
}

/// 解析范围 / 差运算 (.., ~, -)
/// 这些二元运算作用于字符级原子，优先级高于连接运算
fn parse_range_diff(tokens: &[RegexTok], pos: usize) -> (RegexAST, usize) {
    let (mut left, mut pos) = parse_unary(tokens, pos);
    while pos < tokens.len()
        && (tokens[pos] == RegexTok::RangeOp
            || tokens[pos] == RegexTok::Tilde
            || tokens[pos] == RegexTok::Minus)
    {
        let op_token = tokens[pos].clone();
        pos += 1;
        let (right, new_pos) = parse_unary(tokens, pos);
        left = match op_token {
            RegexTok::RangeOp | RegexTok::Tilde => {
                // 从左右原子中提取字符
                let from_c = extract_char(&left);
                let to_c = extract_char(&right);
                match (from_c, to_c) {
                    (Some(fc), Some(tc)) => RegexAST::Range(fc, tc),
                    _ => RegexAST::Range('\0', '\0'),
                }
            }
            RegexTok::Minus => {
                RegexAST::Difference(Box::new(left), Box::new(right))
            }
            _ => unreachable!(),
        };
        pos = new_pos;
    }
    (left, pos)
}

/// 从 AST 节点中提取字符值（用于范围运算）
fn extract_char(ast: &RegexAST) -> Option<char> {
    match ast {
        RegexAST::CharSet(_, Some(c)) => Some(*c),
        RegexAST::CharSet(id, None) => {
            let segs = segments_of(*id);
            if segs.len() == 1 && segs[0].fromChar == segs[0].toChar {
                Some(segs[0].fromChar)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// 从 AST 节点中提取字符集 ID（用于差运算）
fn extract_charset_id(ast: &RegexAST, _env: &HashMap<String, Graph>) -> Option<i32> {
    match ast {
        RegexAST::CharSet(id, _) => Some(*id),
        RegexAST::Name(_name) => {
            // 名字引用时无法直接拿到 charset id，暂不支持
            // 会在 eval_ast_inner 中做处理
            None
        }
        _ => None,
    }
}

/// 解析连接运算
fn parse_concat(tokens: &[RegexTok], pos: usize) -> (RegexAST, usize) {
    let (mut left, mut pos) = parse_range_diff(tokens, pos);
    while pos < tokens.len()
        && tokens[pos] != RegexTok::Pipe
        && tokens[pos] != RegexTok::RParen
        && tokens[pos] != RegexTok::EOF
        && tokens[pos] != RegexTok::RangeOp
        && tokens[pos] != RegexTok::Tilde
        && tokens[pos] != RegexTok::Minus
    {
        // 显式的连接运算符 ·
        if tokens[pos] == RegexTok::Dot {
            pos += 1;
            let (right, new_pos) = parse_range_diff(tokens, pos);
            left = RegexAST::Concat(Box::new(left), Box::new(right));
            pos = new_pos;
        } else {
            // 隐式连接
            let (right, new_pos) = parse_range_diff(tokens, pos);
            left = RegexAST::Concat(Box::new(left), Box::new(right));
            pos = new_pos;
        }
    }
    (left, pos)
}

/// 解析一元后缀运算 (*, +, ?)
fn parse_unary(tokens: &[RegexTok], pos: usize) -> (RegexAST, usize) {
    let (mut ast, mut pos) = parse_atom(tokens, pos);
    while pos < tokens.len() {
        match tokens[pos] {
            RegexTok::Star => {
                ast = RegexAST::Star(Box::new(ast));
                pos += 1;
            }
            RegexTok::Plus => {
                ast = RegexAST::Plus(Box::new(ast));
                pos += 1;
            }
            RegexTok::Question => {
                ast = RegexAST::Option_(Box::new(ast));
                pos += 1;
            }
            _ => break,
        }
    }
    (ast, pos)
}

/// 解析原子 (括号、字符、名字)
fn parse_atom(tokens: &[RegexTok], pos: usize) -> (RegexAST, usize) {
    if pos >= tokens.len() {
        return (RegexAST::CharSet(0, None), pos);
    }
    match &tokens[pos] {
        RegexTok::LParen => {
            let (inner, pos) = parse_union(tokens, pos + 1);
            // 期望 RParen
            let pos = if pos < tokens.len() && tokens[pos] == RegexTok::RParen {
                pos + 1
            } else {
                pos
            };
            (inner, pos)
        }
        RegexTok::CharLit(c) => {
            let id = range(*c, *c);
            (RegexAST::CharSet(id, Some(*c)), pos + 1)
        }
        RegexTok::Name(name) => {
            // 检查是否是 "空格字符"、"回车字符"、"换行字符" 等特殊名字
            let id = match name.as_str() {
                "空格字符" => range(' ', ' '),
                "回车字符" => range('\r', '\r'),
                "换行字符" => range('\n', '\n'),
                _ => {
                    // 作为名字引用
                    return (RegexAST::Name(name.clone()), pos + 1);
                }
            };
            (RegexAST::CharSet(id, None), pos + 1)
        }
        _ => (RegexAST::CharSet(0, None), pos + 1),
    }
}

/// 求值 AST，构建 NFA
fn eval_ast(
    ast: &RegexAST,
    env: &HashMap<String, Graph>,
    is_token: bool,
    token_name: &str,
) -> Graph {
    let mut nfa = eval_ast_inner(ast, env);

    if is_token {
        let cat = name_to_category(token_name);
        for state in &mut nfa.pStateTable {
            if state.StateType == "MATCH" {
                state.LexemeCategory = Some(cat.clone());
            }
        }
    }

    nfa
}

fn eval_ast_inner(ast: &RegexAST, env: &HashMap<String, Graph>) -> Graph {
    match ast {
        RegexAST::CharSet(id, _) => {
            // 检查是单字符还是字符集区间
            let segs = segments_of(*id);
            if segs.len() == 1 && segs[0].fromChar == segs[0].toChar {
                generateBasicNFA("CHAR", *id, None)
            } else {
                generateBasicNFA("CHARSET", *id, None)
            }
        }
        RegexAST::CharLiteral(c) => {
            let id = range(*c, *c);
            generateBasicNFA("CHAR", id, None)
        }
        RegexAST::Name(name) => {
            if let Some(g) = env.get(name) {
                g.clone()
            } else {
                Graph::new(0)
            }
        }
        RegexAST::Union(l, r) => {
            let l_nfa = eval_ast_inner(l, env);
            let r_nfa = eval_ast_inner(r, env);
            union(l_nfa, r_nfa)
        }
        RegexAST::Concat(l, r) => {
            let l_nfa = eval_ast_inner(l, env);
            let r_nfa = eval_ast_inner(r, env);
            product(l_nfa, r_nfa)
        }
        RegexAST::Star(inner) => {
            let nfa = eval_ast_inner(inner, env);
            closure(nfa)
        }
        RegexAST::Plus(inner) => {
            let nfa = eval_ast_inner(inner, env);
            plusClosure(nfa)
        }
        RegexAST::Option_(inner) => {
            let nfa = eval_ast_inner(inner, env);
            zeroOrOne(nfa)
        }
        RegexAST::Range(from_c, to_c) => {
            // 字符范围运算: 'a'~'z' 或 'a'..'z'
            let id = range(*from_c, *to_c);
            generateBasicNFA("CHARSET", id, None)
        }
        RegexAST::Difference(l, r) => {
            // 差运算: letter - 'i'（字符集与字符的差）或 letter - digit（字符集与字符集的差）
            let l_id = extract_charset_id(l, env);
            let r_id = extract_charset_id(r, env);
            let r_char = extract_char(r);

            let result_id = match (l_id, r_id, r_char) {
                (Some(lid), _, Some(c)) => {
                    // 字符集 - 字符
                    difference_charset_char(lid, c)
                }
                (Some(lid), Some(rid), _) => {
                    // 字符集 - 字符集
                    difference_charsets(lid, rid)
                }
                _ => 0,
            };

            if result_id != 0 {
                generateBasicNFA("CHARSET", result_id, None)
            } else {
                Graph::new(0)
            }
        }
    }
}