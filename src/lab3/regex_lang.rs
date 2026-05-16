#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::HashMap;
use crate::lab1::{
    category::LexemeCategory,
    char_set::range,
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
pub fn parse_regex_definitions(text: &str) -> Vec<RegexDef> {
    let mut defs = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // 跳过行首的编号 (如 "1 ")
        let line = skip_leading_number(line);
        // 格式: [@]name -> regex_pattern
        if let Some(def) = parse_one_definition(line) {
            defs.push(def);
        }
    }
    defs
}

/// 跳过行首的 "1 ", "12 " 等编号
fn skip_leading_number(s: &str) -> &str {
    let s = s.trim();
    let mut chars = s.char_indices();
    while let Some((_, c)) = chars.next() {
        if c.is_ascii_digit() || c == ' ' {
            continue;
        }
        // 回退到第一个非数字非空格字符
        break;
    }
    // 简化：找到第一个非数字非空格字符
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
    CharSet(i32),               // 字符集 ID
    CharLiteral(char),          // 字符字面量
    Name(String),               // 引用其他命名正则
    Union(Box<RegexAST>, Box<RegexAST>),
    Concat(Box<RegexAST>, Box<RegexAST>),
    Star(Box<RegexAST>),
    Plus(Box<RegexAST>),
    Option_(Box<RegexAST>),
    Range(Box<RegexAST>, Box<RegexAST>),
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
            // 将 @name 映射到 LexemeCategory
            let cat = name_to_category(&def.name);
            token_map.insert(def.name.clone(), cat);
        }
    }

    // 最后一条定义（lexeme）就是最终结果
    // 合并所有带 @ 的定义对应的 NFA
    let final_nfa = if let Some(last) = defs.last() {
        env.get(&last.name).cloned().unwrap_or_else(|| {
            // 回退：合并所有 token 的 NFA
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
            '\'' => {
                // 字符字面量
                if i + 2 < chars.len() && chars[i + 2] == '\'' {
                    tokens.push(RegexTok::CharLit(chars[i + 1]));
                    i += 2;
                } else if i + 1 < chars.len() {
                    // 处理可能的多字符情况，取下一个非引号字符
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

/// 解析连接运算
fn parse_concat(tokens: &[RegexTok], pos: usize) -> (RegexAST, usize) {
    let (mut left, mut pos) = parse_unary(tokens, pos);
    while pos < tokens.len()
        && tokens[pos] != RegexTok::Pipe
        && tokens[pos] != RegexTok::RParen
        && tokens[pos] != RegexTok::EOF
    {
        // 显式的连接运算符 ·
        if tokens[pos] == RegexTok::Dot {
            pos += 1;
            let (right, new_pos) = parse_unary(tokens, pos);
            left = RegexAST::Concat(Box::new(left), Box::new(right));
            pos = new_pos;
        } else {
            // 隐式连接
            let (right, new_pos) = parse_unary(tokens, pos);
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
        return (RegexAST::CharSet(0), pos);
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
            (RegexAST::CharSet(id), pos + 1)
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
            (RegexAST::CharSet(id), pos + 1)
        }
        _ => (RegexAST::CharSet(0), pos + 1),
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
        // 给终结状态打上 category 标记
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
        RegexAST::CharSet(id) => {
            // 单字符字符集
            generateBasicNFA("CHAR", *id, None)
        }
        RegexAST::CharLiteral(c) => {
            let id = range(*c, *c);
            generateBasicNFA("CHAR", id, None)
        }
        RegexAST::Name(name) => {
            if let Some(g) = env.get(name) {
                g.clone()
            } else {
                // 未定义的名字：创建空 NFA
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
        RegexAST::Range(l, r) => {
            // 字符范围运算: 'a'~'z'
            // 两个都是 CharSet 时做范围
            let _l_nfa = eval_ast_inner(l, env);
            let _r_nfa = eval_ast_inner(r, env);
            // 简化处理：从 AST 中提取字符
            Graph::new(0)
        }
        RegexAST::Difference(l, r) => {
            let _l_nfa = eval_ast_inner(l, env);
            let _r_nfa = eval_ast_inner(r, env);
            Graph::new(0)
        }
    }
}