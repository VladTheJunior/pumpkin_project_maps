use crate::orientation::Orientation;
use serde::{Deserialize, Serialize};

use super::level_template::LevelTemplate;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkTemplate {
    pub name: String,
    pub electric: String,
    pub level_orientation: Orientation,
    pub levels: Vec<LevelTemplate>,
}

impl NetworkTemplate {
    pub fn new(
        name: &str,
        electric: &str,
        level_orientation: Orientation,
        levels: Vec<LevelTemplate>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            electric: electric.to_owned(),
            level_orientation,
            levels,
        }
    }
}
