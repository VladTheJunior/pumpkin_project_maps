use crate::orientation::Orientation;
use serde::{Deserialize, Serialize};

use super::unit_template::UnitTemplate;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectTemplate {
    pub name: String,
    pub unit_orientation: Orientation,
    pub units: Vec<UnitTemplate>,
}

impl ObjectTemplate {
    pub fn new(
        name: &str,
        unit_orientation: Orientation,
        units: Vec<UnitTemplate>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            unit_orientation,
            units,
        }
    }
}
