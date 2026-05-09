use crate::types::*;

pub fn fmt_body(body: &[String]) -> String {
    if body.is_empty() {
        EPSILON.to_string()
    } else {
        body.join(" ")
    }
}

pub fn fmt_production(prod: &Production) -> String {
    format!("{} → {}", prod.head, fmt_body(&prod.body))
}

pub fn fmt_item(item: &LR0Item, grammar: &Grammar) -> String {
    let prod = &grammar.productions[item.production_id];
    if prod.body.is_empty() {
        format!("{} → {}", prod.head, DOT)
    } else {
        let mut parts: Vec<String> = prod.body.clone();
        parts.insert(item.dot_position, DOT.to_string());
        format!("{} → {}", prod.head, parts.join(" "))
    }
}

pub fn print_grammar(grammar: &Grammar) {
    println!("ProductionTable");
    println!("---------------------------------");
    println!("Id\tContent");
    for prod in &grammar.productions {
        println!("{}\t{}", prod.production_id, fmt_production(prod));
    }
    println!("---------------------------------");
}

pub fn print_first_follow(grammar: &Grammar) {
    println!("\nFIRST / FOLLOW 集合");
    println!("---------------------------------");
    println!("{:<10}{:<40}{}", "NT", "FIRST", "FOLLOW");
    for nt in &grammar.non_terminals {
        let sym = &grammar.symbols[nt];
        let fs: Vec<String> = sym.first_set.iter().cloned().collect();
        let fl: Vec<String> = sym.follow_set.iter().cloned().collect();
        println!(
            "{:<10}{{{:<37}}}{{{}}}",
            nt,
            fs.join(", "),
            fl.join(", ")
        );
    }
    println!("---------------------------------");
}

pub fn print_lr0_dfa(dfa: &LR0DFA, grammar: &Grammar) {
    for is in &dfa.item_sets {
        println!("StateId: {}", is.state_id);
        println!("---------------------------------");
        println!("Type\tProduction");
        let mut items: Vec<LR0Item> = is.items.iter().cloned().collect();
        items.sort_by_key(|it| {
            (
                if is.core_items.contains(it) { 0 } else { 1 },
                it.production_id,
                it.dot_position,
            )
        });
        for item in &items {
            let cat = if is.core_items.contains(item) {
                "CORE"
            } else {
                "NONCORE"
            };
            println!("{}\t{}", cat, fmt_item(item, grammar));
        }
        println!("---------------------------------");
    }
    if !dfa.edges.is_empty() {
        println!("Transitions:");
        for e in &dfa.edges {
            println!("  I{}  --{}-->  I{}", e.from_state, e.driver_symbol, e.to_state);
        }
    }
}

pub fn print_lr_parse_table(dfa: &LR0DFA, grammar: &Grammar, table: &LRParseTable) {
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

    print!("State");
    print!(" |");
    for t in &term_cols {
        print!(" {:^6}", t);
    }
    print!(" ||");
    for n in &nt_cols {
        print!(" {:^6}", n);
    }
    println!();
    let total_w = 6 + 1 + 8 * term_cols.len() + 2 + 8 * nt_cols.len();
    println!("{}", "-".repeat(total_w));

    for is in &dfa.item_sets {
        print!("{:>5}", is.state_id);
        print!(" |");
        for t in &term_cols {
            let cell = table.action.get(&(is.state_id, t.clone()));
            let s = match cell {
                Some(c) => match c.action_type {
                    ActionCategory::Shift => format!("s{}", c.id),
                    ActionCategory::Reduce => format!("r{}", c.id),
                    ActionCategory::Accept => "acc".into(),
                },
                None => "".into(),
            };
            print!(" {:^6}", s);
        }
        print!(" ||");
        for n in &nt_cols {
            let s = match table.goto.get(&(is.state_id, n.clone())) {
                Some(g) => g.next_state.to_string(),
                None => "".into(),
            };
            print!(" {:^6}", s);
        }
        println!();
    }
}

pub fn print_ll1_parse_table(grammar: &Grammar, table: &LL1ParseTable) {
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

    print!("{:^6}", "");
    for t in &term_cols {
        print!(" {:^10}", t);
    }
    println!();

    for nt in &nt_cols {
        print!("{:^6}", nt);
        for t in &term_cols {
            let cell = table.cells.get(&(nt.clone(), t.clone()));
            let s = match cell {
                Some(prods) if prods.len() == 1 => format!("r{}", prods[0]),
                Some(prods) => format!("CONFLICT{:?}", prods),
                None => "".into(),
            };
            print!(" {:^10}", s);
        }
        println!();
    }
}
