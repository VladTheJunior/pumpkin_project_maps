use crate::orientation::Orientation;

use super::network_template::NetworkTemplate;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnitTemplate {
    pub id: i32,
    pub network_orientation: Orientation,
    pub networks: Vec<NetworkTemplate>,
}

impl UnitTemplate {
    pub fn new(id: i32, network_orientation: Orientation, networks: Vec<NetworkTemplate>) -> Self {
        Self {
            id,
            network_orientation,
            networks,
        }
    }
}
