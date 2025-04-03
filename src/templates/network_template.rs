use crate::orientation::Orientation;
use serde::{Deserialize, Serialize};

use super::level_template::LevelTemplate;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkTemplate {
    pub id: u8,
    pub is_dhcp_order: bool,
    pub electric: String,
    pub level_orientation: Orientation,
    pub levels: Vec<LevelTemplate>,
}

impl NetworkTemplate {
    pub fn new(
        id: u8,
        is_dhcp_order: bool,
        electric: &str,
        level_orientation: Orientation,
        levels: Vec<LevelTemplate>,
    ) -> Self {
        Self {
            id,
            is_dhcp_order,
            electric: electric.to_owned(),
            level_orientation,
            levels,
        }
    }
}
