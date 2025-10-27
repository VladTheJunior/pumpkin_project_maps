use crate::electric::Electric;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LevelTemplate {
    pub name: String,
    pub rows: i32,
    pub columns: i32,
    pub addresses: Vec<(u32, bool)>,
    pub electrics: Vec<Electric>,
}

impl LevelTemplate {
    pub fn new(
        name: &str,
        rows: i32,
        columns: i32,
        addresses: Vec<(u32, bool)>,
        electrics: Vec<Electric>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            rows,
            columns,
            addresses,
            electrics,
        }
    }
}
