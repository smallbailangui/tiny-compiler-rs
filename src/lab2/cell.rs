#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::lab2::non_terminal_symbol::NonTerminalSymbol;
use crate::lab2::production::Production;
use crate::lab2::terminal_symbol::TerminalSymbol;

#[derive(Clone, Debug)]
pub struct Cell {
    pub nonTerminalSymbol: NonTerminalSymbol,
    pub terminalSymbol: TerminalSymbol,
    pub production: Production,
}

impl Cell {
    pub fn new(
        non_terminal_symbol: NonTerminalSymbol,
        terminal_symbol: TerminalSymbol,
        production: Production,
    ) -> Self {
        Self {
            nonTerminalSymbol: non_terminal_symbol,
            terminalSymbol: terminal_symbol,
            production,
        }
    }
}
