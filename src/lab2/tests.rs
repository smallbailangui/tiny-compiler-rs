use crate::types::Grammar;
use crate::types::EPSILON;

pub fn make_ll1_test1() -> Grammar {
    Grammar::new(
        &["S"],
        &["+", "*", "a"],
        &[
            ("S", &["+", "S", "S"]),
            ("S", &["*", "S", "S"]),
            ("S", &["a"]),
        ],
        "S",
    )
}

pub fn make_ll1_test2() -> Grammar {
    Grammar::new(
        &["T", "S"],
        &["(", ")"],
        &[
            ("T", &["S"]),
            ("S", &["(", "T", ")", "T", "S"]),
            ("S", &[EPSILON]),
        ],
        "T",
    )
}

pub fn make_arith_grammar() -> Grammar {
    Grammar::new(
        &["E", "T", "F"],
        &["+", "*", "(", ")", "id"],
        &[
            ("E", &["E", "+", "T"]),
            ("E", &["T"]),
            ("T", &["T", "*", "F"]),
            ("T", &["F"]),
            ("F", &["(", "E", ")"]),
            ("F", &["id"]),
        ],
        "E",
    )
}

pub fn make_arith_extended_grammar() -> Grammar {
    Grammar::new(
        &["E", "T", "F"],
        &["+", "-", "*", "/", "(", ")", "id"],
        &[
            ("E", &["E", "+", "T"]),
            ("E", &["E", "-", "T"]),
            ("E", &["T"]),
            ("T", &["T", "*", "F"]),
            ("T", &["T", "/", "F"]),
            ("T", &["F"]),
            ("F", &["(", "E", ")"]),
            ("F", &["id"]),
        ],
        "E",
    )
}

pub fn make_tiny_grammar() -> Grammar {
    Grammar::new(
        &["P", "S", "T", "E", "C", "A", "AO", "M", "MO", "F"],
        &[
            "if", "then", "else", "end", "repeat", "until", "read", "write", ";", ":=", "<",
            "=", "+", "-", "*", "/", "(", ")", "id", "num",
        ],
        &[
            ("P", &["S"]),
            ("S", &["S", ";", "T"]),
            ("S", &["T"]),
            ("T", &["if", "E", "then", "S", "end"]),
            ("T", &["if", "E", "then", "S", "else", "S", "end"]),
            ("T", &["repeat", "S", "until", "E"]),
            ("T", &["id", ":=", "E"]),
            ("T", &["read", "id"]),
            ("T", &["write", "E"]),
            ("E", &["A", "C", "A"]),
            ("E", &["A"]),
            ("C", &["<"]),
            ("C", &["="]),
            ("A", &["A", "AO", "M"]),
            ("A", &["M"]),
            ("AO", &["+"]),
            ("AO", &["-"]),
            ("M", &["M", "MO", "F"]),
            ("M", &["F"]),
            ("MO", &["*"]),
            ("MO", &["/"]),
            ("F", &["(", "E", ")"]),
            ("F", &["num"]),
            ("F", &["id"]),
        ],
        "P",
    )
}
