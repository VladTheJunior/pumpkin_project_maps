use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::orientation::Orientation;

use super::unit_template::UnitTemplate;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectTemplate {
    pub id: u16,
    pub name: String,
    pub unit_orientation: Orientation,
    pub units: Vec<UnitTemplate>,
}

impl ObjectTemplate {
    pub fn new(id: u16, name: &str, unit_orientation: Orientation, units: Vec<UnitTemplate>) -> Self {
        Self {
            id,
            name: name.to_owned(),
            unit_orientation,
            units,
        }
    }

    pub fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.id.to_le_bytes())?;
        writer.write_all(&i32::to_le_bytes(self.name.len().try_into()?))?;
        writer.write_all(self.name.as_bytes())?;
        writer.write_all(&self.unit_orientation.to_bytes())?;

        writer.write_all(&i32::to_le_bytes(self.units.len().try_into()?))?;
        for unit in &self.units {
            unit.serialize(writer)?;
        }

        Ok(())
    }
}
