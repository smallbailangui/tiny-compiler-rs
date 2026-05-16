#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod regex_lang;
pub mod scanner;

/// 实验 3：词法分析器构造工具的实现
/// 任务 1：用正则语言描述 TINY 语言的词法，得到完整的词法分析器
/// 任务 2：以 sample.tny 作为输入，输出词序列

/// TINY 语言的词法定义（正则语言程序）
/// 每行格式：[@]name -> regex_pattern
/// 
/// 说明：这是用正则语言描述 TINY 词法的完整程序。
/// 带 @ 前缀的名字表示该正则表达式对应一个词类（token category），
/// 其 NFA 结束状态的 category 属性为该名字。
/// 
/// 最后一行的 lexeme 将前面所有词类合并，构成 TINY 语言词法的完整 DFA。
pub const TINY_LEX_DEF: &str = r#"
character -> '\0'..'\127'
letter -> 'a'..'z' | 'A'..'Z'
@reserved -> '(' | ')' | '|' | '·' | '*' | '+' | '?' | '→' | '@' | '$' | '-' | '~'
@id -> letter+
@cc -> ''' character '''
@space -> ' '+
@crlf -> ('\r' · '\n')+
lexeme -> reserved | id | cc | space | crlf
"#;

/// TINY 语言完整词法定义（覆盖所有实际 TINY 语言 token）
/// 包括：关键字、运算符、界符、标识符、整数、注释、空白
pub const TINY_LEX_FULL_DEF: &str = r#"
digit           -> '0'..'9'
letter          -> 'a'..'z' | 'A'..'Z'
letter_or_digit -> letter | digit

@keyword    -> 'i' 'f' | 't' 'h' 'e' 'n' | 'e' 'l' 's' 'e' | 'e' 'n' 'd'
             | 'r' 'e' 'p' 'e' 'a' 't' | 'u' 'n' 't' 'i' 'l'
             | 'r' 'e' 'a' 'd' | 'w' 'r' 'i' 't' 'e'

@numeric_op    -> '+' | '-' | '*' | '/'
@compare_op    -> '<' | '='
@logic_op      -> '(' | ')' | ';'
@assign        -> ':' '='

@id            -> letter letter_or_digit*

@integer_const -> digit+
@note          -> '{' character* '}'
@space         -> ' ' | '\t' | '\r' | '\n'

lexeme -> keyword | numeric_op | compare_op | logic_op | assign
        | id | integer_const | note | space
"#;

/// 使用动态正则解析器（regex_lang.rs）构建 TINY 词法 DFA。
/// 该函数不再直接调用 lab1 的静态 create_tiny_lexical_dfa()，
/// 而是通过 parse_regex_definitions + build_nfa_from_defs 动态生成 NFA，
/// 再经 NFA_to_DFA 得到最终的 DFA。
pub fn build_tiny_lexer_dfa() -> crate::lab1::graph::Graph {
    use crate::lab1::reset_global_tables;
    use crate::lab3::regex_lang::{parse_regex_definitions, build_nfa_from_defs};

    reset_global_tables();

    // 1. 解析全量 TINY 词法定义
    let defs = parse_regex_definitions(TINY_LEX_FULL_DEF);

    // 2. 动态构建 NFA
    let (nfa, _token_map) = build_nfa_from_defs(&defs);

    // 3. 将生成的 NFA 转换为 DFA
    let dfa = nfa.NFA_to_DFA();

    println!("TINY 词法 DFA 构建完成 (基于动态正则解析)");
    println!("  DFA 状态数: {}", dfa.numOfStates);
    dfa
}

/// 实验 3 主入口：展示正则语言描述 → 构建 DFA → 扫描 sample.tny
pub fn run_lab3_test() {
    println!("========== 实验 3：词法分析器构造工具的实现 ==========\n");

    // 第一步：展示正则语言词法描述
    println!("--- 任务 1：用正则语言描述 TINY 词法 ---");
    println!("以下是描述 TINY 语言词法的正则语言程序:");
    println!("{}", TINY_LEX_DEF);
    println!("（注：上面展示的是正则语言自身的词法描述格式；");
    println!(" 实际构建的词法 DFA 覆盖完整的 TINY 语言 token。）\n");

    // 第二步：构建 DFA
    println!("--- 构建 TINY 词法 DFA ---");
    let dfa = build_tiny_lexer_dfa();
    println!();

    // 第三步：扫描 sample.tny
    println!("--- 任务 2：扫描 sample.tny ---");
    let sample = r#"
        { Sample program in TINY language - computes factorial }
        read x; { input an integer }
        if 0 < x then { don't compute if x <= 0 }
            fact := 1;
            repeat
                fact := fact * x;
                x := x - 1
            until x = 0;
            write fact { output factorial of x }
        end
    "#;

    println!("源程序:\n{}", sample);
    println!("词法分析结果:");

    let tokens = dfa.long_text_search(sample);
    for token in &tokens {
        // 跳过空白和注释
        if token.lexeme_category == crate::lab1::LexemeCategory::SPACE_CONST
            || token.lexeme_category == crate::lab1::LexemeCategory::NOTE
        {
            continue;
        }
        let content = if let Some(val) = token.value {
            val.to_string()
        } else if let Some(ref ident) = token.identify {
            ident.clone()
        } else {
            "-".to_string()
        };
        println!("  {:<22} | {}", format!("{:?}", token.lexeme_category), content);
    }

    println!("\n========== 实验 3 完成 ==========");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lab1::reset_global_tables;

    #[test]
    fn test_lab3_sample_tiny_scan() {
        reset_global_tables();
        let dfa = build_tiny_lexer_dfa();

        let sample = r#"
            { Sample program in TINY language - computes factorial }
            read x; { input an integer }
            if 0 < x then { don't compute if x <= 0 }
                fact := 1;
                repeat
                    fact := fact * x;
                    x := x - 1
                until x = 0;
                write fact { output factorial of x }
            end
        "#;

        let tokens = dfa.long_text_search(sample);
        // 过滤空白和注释后的 token 列表
        let meaningful: Vec<_> = tokens
            .iter()
            .filter(|t| {
                t.lexeme_category != crate::lab1::LexemeCategory::SPACE_CONST
                    && t.lexeme_category != crate::lab1::LexemeCategory::NOTE
            })
            .collect();

        assert!(!meaningful.is_empty(), "应该能扫描出 token");

        // 验证关键 token 存在
        let has_read = meaningful.iter().any(|t| t.identify.as_deref() == Some("read"));
        let has_if = meaningful.iter().any(|t| t.identify.as_deref() == Some("if"));
        let has_end = meaningful.iter().any(|t| t.identify.as_deref() == Some("end"));
        let has_write = meaningful.iter().any(|t| t.identify.as_deref() == Some("write"));

        assert!(has_read, "应该包含 read 关键字");
        assert!(has_if, "应该包含 if 关键字");
        assert!(has_end, "应该包含 end 关键字");
        assert!(has_write, "应该包含 write 关键字");
    }
}