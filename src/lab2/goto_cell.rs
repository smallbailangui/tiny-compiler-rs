#![allow(non_snake_case)]
#![allow(dead_code)]

#[derive(Clone, Debug)]
pub struct GotoCell {
    pub stateId: i64,
    pub nonTerminalStmbolName: String,
    pub nextStateId: i64,
}

impl GotoCell {
    pub fn new(state_id: i64, non_terminal_symbol_name: &str, next_state_id: i64) -> Self {
        Self {
            stateId: state_id,
            nonTerminalStmbolName: non_terminal_symbol_name.to_string(),
            nextStateId: next_state_id,
        }
    }
}
