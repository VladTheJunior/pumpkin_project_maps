use crate::orientation::Orientation;
use serde::{Deserialize, Serialize};

use super::unit_template::UnitTemplate;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectTemplate {
    pub id: u16,
    pub name: String,
    pub unit_orientation: Orientation,
    pub units: Vec<UnitTemplate>,
}

impl ObjectTemplate {
    pub fn new(
        id: u16,
        name: &str,
        unit_orientation: Orientation,
        units: Vec<UnitTemplate>,
    ) -> Self {
        Self {
            id,
            name: name.to_owned(),
            unit_orientation,
            units,
        }
    }
}
