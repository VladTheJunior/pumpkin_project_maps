use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Orientation {
    #[default]
    Vertical,
    Horizontal,
}
