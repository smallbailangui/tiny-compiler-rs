#![allow(non_snake_case)]
#![allow(dead_code)]

#[derive(Clone, Debug)]
pub struct ActionCell {
    pub stateId: i64,
    pub terminalSymbolName: String,
    pub actionType: String,
    pub actionId: i64,
}

impl ActionCell {
    pub fn new(state_id: i64, terminal_symbol_name: &str, action_type: &str, action_id: i64) -> Self {
        Self {
            stateId: state_id,
            terminalSymbolName: terminal_symbol_name.to_string(),
            actionType: action_type.to_string(),
            actionId: action_id,
        }
    }
}
