use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::electric::Electric;

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
    pub fn new(id: i32, unit: &str, rows: i32, columns: i32, addresses: Vec<u8>, electrics: Vec<Electric>) -> Self {
        Self {
            id,
            unit: unit.to_owned(),
            rows,
            columns,
            addresses,
            electrics,
        }
    }

    pub fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.id.to_le_bytes())?;
        writer.write_all(&i32::to_le_bytes(self.unit.len().try_into()?))?;
        writer.write_all(self.unit.as_bytes())?;
        writer.write_all(&self.rows.to_le_bytes())?;
        writer.write_all(&self.columns.to_le_bytes())?;
        writer.write_all(&i32::to_le_bytes(self.addresses.len().try_into()?))?;
        writer.write_all(&self.addresses)?;

        writer.write_all(&i32::to_le_bytes(self.electrics.len().try_into()?))?;
        for electric in &self.electrics {
            electric.serialize(writer)?;
        }

        Ok(())
    }
}
