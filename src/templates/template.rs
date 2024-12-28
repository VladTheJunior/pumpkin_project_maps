use std::io::{BufWriter, Write};

use super::object_template::ObjectTemplate;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Template {
    pub objects: Vec<ObjectTemplate>,
}

impl Template {
    /// used in BitScan
    pub fn serialize_legacy(&self) -> Result<Vec<u8>> {
        let mut writer = BufWriter::new(Vec::new());

        writer.write_all(&i32::to_le_bytes(self.objects.len().try_into()?))?;
        for object in &self.objects {
            object.serialize(&mut writer)?;
        }
        writer.flush()?;
        Ok(writer.into_inner()?)
    }
}
