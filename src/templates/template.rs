use std::collections::HashMap;

use super::object_template::ObjectTemplate;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Template {
    pub objects: Vec<ObjectTemplate>,
    pub content: HashMap<u32, MapContent>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MapContent {
    pub object_id: u32,
    pub unit_id: u32,
    pub netowrk_id: u32,
    pub level_id: u8,
    pub row: u8,
    pub column: u8,
    pub is_dhcp: bool,
}
