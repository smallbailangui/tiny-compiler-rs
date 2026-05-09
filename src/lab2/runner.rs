use std::fs;

use crate::display::*;
use crate::first::compute_first_sets;
use crate::follow::compute_follow_sets;
use crate::ll1::*;
use crate::lr0::build_lr0_dfa;
use crate::report::*;
use crate::slr::*;
use crate::types::*;

pub fn run_ll_test(name: &str, grammar: &mut Grammar, html: &mut String) {
    println!("\n========================================");
    println!("LL(1) 测试: {}", name);
    println!("========================================");
    print_grammar(grammar);

    compute_first_sets(grammar);
    compute_follow_sets(grammar);
    print_first_follow(grammar);

    let table = build_ll1_parse_table(grammar);
    let (ok, conflicts) = check_ll1(&table);
    println!("\nLL(1) 分析表:");
    print_ll1_parse_table(grammar, &table);
    if ok {
        println!("\n✔ 该文法是 LL(1) 文法");
    } else {
        println!("\n✘ 该文法不是 LL(1) 文法，冲突如下:");
        for c in &conflicts {
            println!("  - {}", c);
        }
    }

    html.push_str(&format!(
        "<div class=\"panel\"><div class=\"panel-head\"><span class=\"ico\">🔍</span>{}</div>\n<div class=\"panel-body\">\n",
        html_escape(name)
    ));
    html.push_str(&html_grammar_section("产生式表", grammar));
    html.push_str(&html_first_follow(grammar));
    html.push_str(&html_ll1_parse_table(grammar, &table));
    html.push_str(&html_conflicts(&conflicts, ok, "LL(1) 文法"));
    html.push_str("</div></div>\n");
}

pub fn run_lr_test(name: &str, grammar: &mut Grammar, html: &mut String) {
    println!("\n========================================");
    println!("LR/SLR(1) 测试: {}", name);
    println!("========================================");
    print_grammar(grammar);

    compute_first_sets(grammar);
    compute_follow_sets(grammar);
    print_first_follow(grammar);

    let dfa = build_lr0_dfa(grammar);
    print_lr0_dfa(&dfa, grammar);

    let (slr_ok, slr_conflicts) = check_slr1(&dfa, grammar);
    let table = build_lr_parse_table(&dfa, grammar);

    println!("\nLR 分析表:");
    print_lr_parse_table(&dfa, grammar, &table);
    if slr_ok {
        println!("\n✔ 该文法是 SLR(1) 文法");
    } else {
        println!("\n✘ 该文法不是 SLR(1) 文法，冲突如下:");
        for c in &slr_conflicts {
            println!("  - {}", c);
        }
    }

    html.push_str(&format!(
        "<div class=\"panel\"><div class=\"panel-head\"><span class=\"ico\">🔍</span>{}</div>\n<div class=\"panel-body\">\n",
        html_escape(name)
    ));
    html.push_str(&html_grammar_section("产生式表", grammar));
    html.push_str(&html_first_follow(grammar));
    html.push_str(&html_dfa(&dfa, grammar));
    html.push_str(&html_lr_parse_table(&dfa, grammar, &table));
    html.push_str(&html_conflicts(&slr_conflicts, slr_ok, "SLR(1) 文法"));
    if !table.conflicts.is_empty() {
        html.push_str("<h4>分析表冲突</h4><ul>");
        for c in &table.conflicts {
            html.push_str(&format!("<li><code>{}</code></li>", html_escape(c)));
        }
        html.push_str("</ul>");
    }
    html.push_str("</div></div>\n");
}

pub fn run_lab2() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║ 实验二：上下文无关文法的 DFA 构建            ║");
    println!("╚══════════════════════════════════════════════╝");

    fs::create_dir_all("./results").expect("无法创建 results 目录");
    let mut html = String::from(HTML_HEADER);
    html.push_str(
        r##"<div class="sec-divider"><span class="dot" id="s-ll"></span><h2>LL(1) 分析表构造</h2></div>
"##,
    );
    {
        let mut g = crate::tests::make_ll1_test1();
        run_ll_test(
            "LL(1) Test 1: S → +SS | *SS | a",
            &mut g,
            &mut html,
        );
    }
    {
        let mut g = crate::tests::make_ll1_test2();
        run_ll_test(
            "LL(1) Test 2: T → S, S → ( T ) T S | ε",
            &mut g,
            &mut html,
        );
    }

    html.push_str(
        r##"<div class="sec-divider"><span class="dot" id="s-arith"></span><h2>算术表达式文法（SLR(1) 验证）</h2></div>
"##,
    );
    {
        let mut g = crate::tests::make_arith_grammar();
        run_lr_test(
            "算术表达式（基础）：E → E+T | T,  T → T*F | F,  F → (E) | id",
            &mut g,
            &mut html,
        );
    }
    {
        let mut g = crate::tests::make_arith_extended_grammar();
        run_lr_test("算术表达式（扩展 -, /）", &mut g, &mut html);
    }

    html.push_str(
        r##"<div class="sec-divider"><span class="dot" id="s-tiny"></span><h2>TINY 语言文法（SLR(1) 验证）</h2></div>
"##,
    );
    {
        let mut g = crate::tests::make_tiny_grammar();
        run_lr_test("TINY 语言文法", &mut g, &mut html);
    }

    html.push_str("<div class=\"foot\">实验二 · 上下文无关文法的 DFA 构建 · 可视化报告</div></div></div></body></html>");

    fs::write("./results/lab2_report.html", &html).expect("写入 lab2_report.html 失败");

  
    println!("测试完成");
    
}
