#![allow(non_snake_case)]
#![allow(dead_code)]

#[derive(Clone, Debug)]
pub struct ProductionInfo {
    pub indexId: i64,
    pub headName: String,
    pub bodySize: i64,
}

impl ProductionInfo {
    pub fn new(index_id: i64, head_name: &str, body_size: i64) -> Self {
        Self {
            indexId: index_id,
            headName: head_name.to_string(),
            bodySize: body_size,
        }
    }
}
