use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Electric {
    pub top: i32,
    pub left: i32,
    pub height: i32,
    pub width: i32,
    pub color: i32,
}

impl Electric {
    pub fn new(top: i32, left: i32, height: i32, width: i32, color: i32) -> Self {
        Self {
            top,
            left,
            height,
            width,
            color,
        }
    }
}
