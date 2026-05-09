use crate::display::{fmt_item, fmt_production};
use crate::types::*;

pub fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn html_grammar_section(name: &str, grammar: &Grammar) -> String {
    let mut h = String::new();
    h.push_str(&format!(
        "<div class=\"panel\"><div class=\"panel-head\"><span class=\"ico\">\u{1f4cb}</span>\u{6587}\u{6cd5} \u{2014} {}</div>\n",
        html_escape(name)
    ));
    h.push_str(
        "<div class=\"panel-body compact\"><table><thead><tr><th>ID</th><th>\u{4ea7}\u{751f}\u{5f0f}</th></tr></thead><tbody>\n",
    );
    for prod in &grammar.productions {
        h.push_str(&format!(
            "<tr><td>{}</td><td><code>{}</code></td></tr>\n",
            prod.production_id,
            html_escape(&fmt_production(prod))
        ));
    }
    h.push_str("</tbody></table></div></div>\n");
    h
}

pub fn html_first_follow(grammar: &Grammar) -> String {
    let mut h = String::from(
        "<div class=\"panel\"><div class=\"panel-head\"><span class=\"ico\">\u{1f524}</span>FIRST / FOLLOW \u{96c6}</div>\n<div class=\"panel-body compact\"><table><thead><tr><th>NT</th><th>FIRST</th><th>FOLLOW</th></tr></thead><tbody>\n",
    );
    for nt in &grammar.non_terminals {
        let sym = &grammar.symbols[nt];
        let fs: Vec<String> = sym.first_set.iter().cloned().collect();
        let fl: Vec<String> = sym.follow_set.iter().cloned().collect();
        h.push_str(&format!(
            "<tr><td>{}</td><td><code>{{{}}}</code></td><td><code>{{{}}}</code></td></tr>\n",
            html_escape(nt),
            html_escape(&fs.join(", ")),
            html_escape(&fl.join(", "))
        ));
    }
    h.push_str("</tbody></table></div></div>\n");
    h
}

pub fn html_dfa(dfa: &LR0DFA, grammar: &Grammar) -> String {
    let mut h = String::from("<div class=\"panel\"><div class=\"panel-head\"><span class=\"ico\">\u{1f500}</span>LR(0) DFA \u{72b6}\u{6001}");
    h.push_str(&format!(
        " &nbsp;<span style=\"font-weight:400;color:var(--c5);font-size:11px\">{} \u{72b6}\u{6001} \u{b7} {} \u{53d8}\u{8fc1}</span></div>\n<div class=\"panel-body\">",
        dfa.item_sets.len(),
        dfa.edges.len()
    ));
    h.push_str("<div class=\"state-grid\">\n");
    for is in &dfa.item_sets {
        h.push_str(&format!(
            "<div class=\"state-card\"><div class=\"sid\">I{}</div><table><tbody>\n",
            is.state_id
        ));
        let mut items: Vec<LR0Item> = is.items.iter().cloned().collect();
        items.sort_by_key(|it| {
            (
                if is.core_items.contains(it) { 0 } else { 1 },
                it.production_id,
                it.dot_position,
            )
        });
        for item in &items {
            let (tag, cls) = if is.core_items.contains(item) {
                ("CORE", "core-tag")
            } else {
                ("NC", "ncore-tag")
            };
            h.push_str(&format!(
                "<tr><td class=\"{}\">{}</td><td><code>{}</code></td></tr>\n",
                cls,
                tag,
                html_escape(&fmt_item(item, grammar))
            ));
        }
        h.push_str("</tbody></table></div>\n");
    }
    h.push_str("</div>\n");

    if !dfa.edges.is_empty() {
        h.push_str("<div class=\"trans-bar\">");
        for e in &dfa.edges {
            h.push_str(&format!(
                "<span class=\"trans-chip\">I{} <span class=\"arr\">\u{2192}</span><span class=\"sym\">{}</span> <span class=\"arr\">\u{2192}</span> I{}</span>\n",
                e.from_state,
                html_escape(&e.driver_symbol),
                e.to_state
            ));
        }
        h.push_str("</div>\n");
    }

    if dfa.item_sets.len() <= 30 {
        let mut merm = String::from(
            "<div class=\"mermaid-wrap\"><pre class=\"mermaid\">\nflowchart LR\n",
        );
        for is in &dfa.item_sets {
            merm.push_str(&format!("    I{}((\"I{}\"))\n", is.state_id, is.state_id));
        }
        merm.push_str(&format!("    I{}:::start\n", dfa.start_state));
        for e in &dfa.edges {
            let label = e
                .driver_symbol
                .replace('"', "#quot;")
                .replace('<', "#lt;")
                .replace('>', "#gt;");
            merm.push_str(&format!(
                "    I{} -->|\"{}\"| I{}\n",
                e.from_state, label, e.to_state
            ));
        }
        merm.push_str(
            "    classDef start fill:#1f6feb,stroke:#58a6ff,stroke-width:3px,color:#fff\n",
        );
        merm.push_str("</pre></div>\n");
        h.push_str(&merm);
    } else {
        h.push_str(&format!(
            "<p style=\"color:var(--c5);font-size:12px;margin-top:6px\">\u{72b6}\u{6001}\u{8f83}\u{591a} ({}), \u{7701}\u{7565} Mermaid \u{56fe}</p>\n",
            dfa.item_sets.len()
        ));
    }
    h.push_str("</div></div>\n");
    h
}

pub fn html_lr_parse_table(dfa: &LR0DFA, grammar: &Grammar, table: &LRParseTable) -> String {
    let mut term_cols: Vec<String> = grammar
        .terminals
        .iter()
        .filter(|t| t.as_str() != EPSILON)
        .cloned()
        .collect();
    term_cols.sort_by_key(|t| if t.as_str() == END_MARKER { 1 } else { 0 });
    let nt_cols: Vec<String> = grammar
        .non_terminals
        .iter()
        .filter(|n| **n != grammar.aug_root)
        .cloned()
        .collect();

    let mut h = String::from(
        "<div class=\"panel\"><div class=\"panel-head\"><span class=\"ico\">\u{1f4ca}</span>LR \u{5206}\u{6790}\u{8868}</div>\n<div class=\"panel-body compact\"><div class=\"tbl-scroll\"><table><thead>\n",
    );
    h.push_str("<tr><th rowspan=\"2\">\u{72b6}\u{6001}</th>");
    h.push_str(&format!("<th colspan=\"{}\">ACTION</th>", term_cols.len()));
    h.push_str(&format!(
        "<th colspan=\"{}\">GOTO</th></tr>\n<tr>",
        nt_cols.len()
    ));
    for t in &term_cols {
        h.push_str(&format!("<th>{}</th>", html_escape(t)));
    }
    for n in &nt_cols {
        h.push_str(&format!("<th>{}</th>", html_escape(n)));
    }
    h.push_str("</tr></thead><tbody>\n");

    for is in &dfa.item_sets {
        h.push_str(&format!("<tr><td>{}</td>", is.state_id));
        for t in &term_cols {
            let (txt, cls) = match table.action.get(&(is.state_id, t.clone())) {
                Some(c) => match c.action_type {
                    ActionCategory::Shift => (format!("s{}", c.id), "act-shift"),
                    ActionCategory::Reduce => (format!("r{}", c.id), "act-reduce"),
                    ActionCategory::Accept => ("acc".into(), "act-accept"),
                },
                None => ("\u{b7}".into(), ""),
            };
            h.push_str(&format!("<td class=\"{}\">{}</td>", cls, txt));
        }
        for n in &nt_cols {
            let txt = match table.goto.get(&(is.state_id, n.clone())) {
                Some(g) => g.next_state.to_string(),
                None => "\u{b7}".into(),
            };
            h.push_str(&format!("<td class=\"act-goto\">{}</td>", txt));
        }
        h.push_str("</tr>\n");
    }
    h.push_str("</tbody></table></div></div></div>\n");
    h
}

pub fn html_ll1_parse_table(grammar: &Grammar, table: &LL1ParseTable) -> String {
    let term_cols: Vec<String> = grammar
        .terminals
        .iter()
        .filter(|t| t.as_str() != EPSILON)
        .cloned()
        .collect();
    let nt_cols: Vec<String> = grammar
        .non_terminals
        .iter()
        .filter(|n| **n != grammar.aug_root)
        .cloned()
        .collect();

    let mut h = String::from(
        "<div class=\"panel\"><div class=\"panel-head\"><span class=\"ico\">\u{1f4ca}</span>LL(1) \u{5206}\u{6790}\u{8868}</div>\n<div class=\"panel-body compact\"><div class=\"tbl-scroll\"><table><thead><tr><th></th>",
    );
    for t in &term_cols {
        h.push_str(&format!("<th>{}</th>", html_escape(t)));
    }
    h.push_str("</tr></thead><tbody>\n");
    for nt in &nt_cols {
        h.push_str(&format!("<tr><td>{}</td>", html_escape(nt)));
        for t in &term_cols {
            let (txt, cls) = match table.cells.get(&(nt.clone(), t.clone())) {
                Some(prods) if prods.len() == 1 => {
                    (
                        format!("r{}", prods[0]),
                        "act-reduce",
                    )
                }
                Some(_prods) => ("\u{26a0}".into(), "act-conflict"),
                None => ("\u{b7}".into(), ""),
            };
            h.push_str(&format!("<td class=\"{}\">{}</td>", cls, txt));
        }
        h.push_str("</tr>\n");
    }
    h.push_str("</tbody></table></div></div></div>\n");
    h
}

pub fn html_conflicts(conflicts: &[String], is_ok: bool, kind: &str) -> String {
    let mut h = String::new();
    if is_ok {
        h.push_str(&format!(
            "<div class=\"badge badge-ok\">\u{2714} \u{8be5}\u{6587}\u{6cd5}\u{662f} {}</div>\n",
            kind
        ));
    } else {
        h.push_str(&format!(
            "<div class=\"badge badge-ng\">\u{2718} \u{8be5}\u{6587}\u{6cd5}\u{4e0d}\u{662f} {}\u{ff0c}\u{68c0}\u{6d4b}\u{5230} {} \u{4e2a}\u{51b2}\u{7a81}</div>\n",
            kind,
            conflicts.len()
        ));
        h.push_str("<ul class=\"conflict-list\">\n");
        for c in conflicts {
            h.push_str(&format!("<li>{}</li>\n", html_escape(c)));
        }
        h.push_str("</ul>\n");
    }
    h
}

pub const HTML_HEADER: &str = r##"<!DOCTYPE html><html lang="zh-CN"><head><meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>实验二 · DFA 构建与分析表</title>
<script src="https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.min.js"></script>
<script>mermaid.initialize({startOnLoad:true,theme:'default',flowchart:{curve:'basis'},securityLevel:'loose'});</script>
<style>
:root {
  --c0: #0d1117; --c1: #161b22; --c2: #21262d; --c3: #30363d;
  --c4: #484f58; --c5: #8b949e; --c6: #c9d1d9; --c7: #f0f6fc;
  --ac: #58a6ff; --gr: #3fb950; --ye: #d29922; --re: #f85149;
  --pu: #bc8cff; --or: #f0883e;
  --mono: 'SF Mono','Fira Code','Cascadia Code',Consolas,monospace;
  --sans: -apple-system,BlinkMacSystemFont,'Segoe UI',system-ui,sans-serif;
  --radius: 6px;
}
*,*::before,*::after{box-sizing:border-box;margin:0;padding:0}
html{scroll-behavior:smooth;scroll-padding-top:60px}
body{font-family:var(--sans);background:var(--c0);color:var(--c6);line-height:1.45;font-size:14px}
a{color:var(--ac);text-decoration:none}
a:hover{text-decoration:underline}

.topbar{position:sticky;top:0;z-index:100;background:var(--c1);border-bottom:1px solid var(--c3);
  display:flex;align-items:center;justify-content:space-between;padding:0 24px;height:48px}
.topbar h1{font-size:15px;font-weight:600;color:var(--c7);letter-spacing:.3px}
.topbar .ver{font-size:11px;color:var(--c5)}

.layout{display:flex;min-height:calc(100vh - 48px)}
.sidebar{width:220px;flex-shrink:0;background:var(--c1);border-right:1px solid var(--c3);
  padding:20px 0;position:sticky;top:48px;height:calc(100vh - 48px);overflow-y:auto}
.sidebar .sec-title{padding:6px 20px;font-size:10px;text-transform:uppercase;
  letter-spacing:.8px;color:var(--c4);margin-top:12px}
.sidebar a{display:block;padding:6px 20px;font-size:13px;color:var(--c5);border-left:2px solid transparent;
  transition:all .15s}
.sidebar a:hover,.sidebar a.act{color:var(--ac);border-left-color:var(--ac);background:var(--c2);
  text-decoration:none}

.main{flex:1;padding:24px 32px;max-width:1100px}

.sec-divider{display:flex;align-items:center;gap:10px;margin:32px 0 18px;padding-bottom:8px;
  border-bottom:1px dashed var(--c3)}
.sec-divider .dot{width:8px;height:8px;border-radius:50%;background:var(--ac)}
.sec-divider h2{font-size:17px;font-weight:600;color:var(--c7);letter-spacing:.2px}

.panel{background:var(--c1);border:1px solid var(--c3);border-radius:var(--radius);
  margin-bottom:16px;overflow:hidden}
.panel-head{padding:10px 16px;background:var(--c2);border-bottom:1px solid var(--c3);
  font-size:13px;font-weight:600;color:var(--c7);display:flex;align-items:center;gap:8px}
.panel-head .ico{font-size:14px}
.panel-body{padding:14px 16px}
.panel-body.compact{padding:0}

table{width:100%;border-collapse:collapse;font-size:12px}
thead th{position:sticky;top:0;background:var(--c2);color:var(--c5);font-weight:600;font-size:11px;
  text-transform:uppercase;letter-spacing:.5px;padding:7px 10px;text-align:center;
  border-bottom:2px solid var(--c3);white-space:nowrap}
td{padding:6px 10px;text-align:center;border-bottom:1px solid var(--c3);color:var(--c6)}
tr:last-child td{border-bottom:none}
tbody tr:hover td{background:var(--c2)}
td code{font-family:var(--mono);font-size:11px;color:var(--c6);background:var(--c2);
  padding:2px 6px;border-radius:3px}
td:first-child{font-weight:700;color:var(--c7);font-family:var(--mono)}
td.act-shift{color:var(--ac);font-weight:700;font-family:var(--mono)}
td.act-reduce{color:var(--ye);font-weight:700;font-family:var(--mono)}
td.act-accept{color:var(--gr);font-weight:700;font-family:var(--mono)}
td.act-goto{color:var(--pu);font-weight:700;font-family:var(--mono)}
td.act-conflict{color:var(--re);font-weight:700;font-family:var(--mono)}

.tbl-scroll{max-height:480px;overflow:auto;border:1px solid var(--c3);border-radius:var(--radius)}
.tbl-scroll table{margin:0;border:none}

.state-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(320px,1fr));gap:10px}
.state-card{background:var(--c2);border:1px solid var(--c3);border-radius:var(--radius);
  padding:10px 12px}
.state-card .sid{font-size:13px;font-weight:700;color:var(--ac);margin-bottom:6px}
.state-card table{font-size:11px}
.state-card thead th{font-size:10px;padding:4px 8px;background:transparent}
.state-card td{padding:3px 8px;font-size:11px}
.state-card td.core-tag{color:var(--ye);font-weight:700;width:50px;font-size:10px}
.state-card td.ncore-tag{color:var(--c4);font-size:10px}

.trans-bar{display:flex;flex-wrap:wrap;gap:4px;align-items:center;
  padding:8px 0;font-size:11px;font-family:var(--mono)}
.trans-chip{display:inline-flex;align-items:center;gap:3px;
  background:var(--c2);border:1px solid var(--c3);border-radius:4px;padding:3px 8px;font-size:11px}
.trans-chip .sym{color:var(--ac);font-weight:600}
.trans-chip .arr{color:var(--c4)}

.badge{display:inline-flex;align-items:center;gap:5px;padding:5px 14px;
  border-radius:4px;font-size:12px;font-weight:600}
.badge-ok{background:#122d1e;color:var(--gr);border:1px solid #1a472a}
.badge-ng{background:#2d1215;color:var(--re);border:1px solid #471a1a}
.conflict-list{list-style:none;display:flex;flex-direction:column;gap:3px;margin-top:8px}
.conflict-list li{font-family:var(--mono);font-size:11px;color:var(--or);
  background:var(--c2);padding:5px 10px;border-radius:3px;border-left:3px solid var(--or)}

.mermaid-wrap{background:var(--c2);border:1px solid var(--c3);border-radius:var(--radius);
  padding:12px;margin:8px 0;overflow-x:auto}

.foot{text-align:center;padding:40px 0 24px;font-size:11px;color:var(--c4);
  border-top:1px solid var(--c3);margin-top:32px}

@media(max-width:800px){
  .sidebar{display:none}
  .layout{display:block}
  .main{padding:16px}
  .state-grid{grid-template-columns:1fr}
}
</style></head><body>
<div class="topbar">
  <h1>📐 实验二 · DFA 构建与语法分析表</h1>
  <span class="ver">FIRST / FOLLOW · LL(1) · LR(0) · SLR(1)</span>
</div>
<div class="layout">
<div class="sidebar">
  <div class="sec-title">目录</div>
  <a href="#s-ll">LL(1) 分析表构造</a>
  <a href="#s-arith">算术表达式 (SLR)</a>
  <a href="#s-tiny">TINY 文法 (SLR)</a>
</div>
<div class="main">
"##;
