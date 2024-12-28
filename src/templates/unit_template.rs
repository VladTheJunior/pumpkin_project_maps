use std::io::Write;

use crate::orientation::Orientation;

use super::network_template::NetworkTemplate;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnitTemplate {
    pub id: i32,
    pub network_orientation: Orientation,
    pub networks: Vec<NetworkTemplate>,
}

impl UnitTemplate {
    pub fn new(id: i32, network_orientation: Orientation, networks: Vec<NetworkTemplate>) -> Self {
        Self { id, network_orientation, networks }
    }

    pub fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.id.to_le_bytes())?;
        writer.write_all(&self.network_orientation.to_bytes())?;

        writer.write_all(&i32::to_le_bytes(self.networks.len().try_into()?))?;
        for network in &self.networks {
            network.serialize(writer)?;
        }

        Ok(())
    }
}
