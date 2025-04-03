use crate::electric::Electric;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LevelTemplate {
    pub id: i32,
    pub unit: String,
    pub rows: i32,
    pub columns: i32,
    pub addresses: Vec<u8>,
    pub electrics: Vec<Electric>,
}

impl LevelTemplate {
    pub fn new(
        id: i32,
        unit: &str,
        rows: i32,
        columns: i32,
        addresses: Vec<u8>,
        electrics: Vec<Electric>,
    ) -> Self {
        Self {
            id,
            unit: unit.to_owned(),
            rows,
            columns,
            addresses,
            electrics,
        }
    }
}
