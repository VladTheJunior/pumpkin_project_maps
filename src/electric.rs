use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

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
        Self { top, left, height, width, color }
    }

    pub fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.top.to_le_bytes())?;
        writer.write_all(&self.left.to_le_bytes())?;
        writer.write_all(&self.height.to_le_bytes())?;
        writer.write_all(&self.width.to_le_bytes())?;
        writer.write_all(&self.color.to_le_bytes())?;
        Ok(())
    }

    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let top = i32::from_le_bytes(buffer);

        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let left = i32::from_le_bytes(buffer);

        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let height = i32::from_le_bytes(buffer);

        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let width = i32::from_le_bytes(buffer);

        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let color = i32::from_le_bytes(buffer);

        Ok(Self { top, left, height, width, color })
    }
}
