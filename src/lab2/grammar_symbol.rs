#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::lab2::terminal_symbol::TerminalSymbol;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GrammarSymbol {
    Terminal(TerminalSymbol),
    NonTerminal(String),
    Null,
}

impl GrammarSymbol {
    pub fn name(&self) -> &str {
        match self {
            GrammarSymbol::Terminal(ts) => &ts.name,
            GrammarSymbol::NonTerminal(name) => name,
            GrammarSymbol::Null => "null",
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, GrammarSymbol::Terminal(_))
    }

    pub fn is_non_terminal(&self) -> bool {
        matches!(self, GrammarSymbol::NonTerminal(_))
    }
}
