use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Orientation {
    #[default]
    Vertical,
    Horizontal,
}

impl Orientation {
    pub const fn to_bytes(&self) -> [u8; 1] {
        match self {
            Orientation::Vertical => [0u8],
            Orientation::Horizontal => [1u8],
        }
    }

    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        if bytes == [0u8] {
            Self::Vertical
        } else {
            Self::Horizontal
        }
    }
}
