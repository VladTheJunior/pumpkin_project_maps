use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::orientation::Orientation;

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
    pub fn new(id: u8, is_dhcp_order: bool, electric: &str, level_orientation: Orientation, levels: Vec<LevelTemplate>) -> Self {
        Self {
            id,
            is_dhcp_order,
            electric: electric.to_owned(),
            level_orientation,
            levels,
        }
    }

    pub fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.id.to_le_bytes())?;
        writer.write_all(if self.is_dhcp_order { &[1u8] } else { &[0u8] })?;
        writer.write_all(&i32::to_le_bytes(self.electric.len().try_into()?))?;
        writer.write_all(self.electric.as_bytes())?;
        writer.write_all(&self.level_orientation.to_bytes())?;

        writer.write_all(&i32::to_le_bytes(self.levels.len().try_into()?))?;
        for level in &self.levels {
            level.serialize(writer)?;
        }

        Ok(())
    }
}
