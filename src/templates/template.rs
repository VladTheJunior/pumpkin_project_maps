use std::{
    collections::HashSet,
    io::{BufWriter, Write},
};

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

    pub fn fetch_known_ips(&self) -> HashSet<u32> {
        let mut ips = HashSet::new();
        for object in &self.objects {
            let a = object.id.to_be_bytes()[0];
            let b = object.id.to_be_bytes()[1];
            for unit in &object.units {
                for network in &unit.networks {
                    for level in &network.levels {
                        ips.extend(level.addresses.iter().filter_map(|d| {
                            if *d == 0 {
                                None
                            } else {
                                Some(u32::from_be_bytes([a, b, network.id, *d]))
                            }
                        }));
                    }
                }
            }
        }
        return ips;
    }
}
