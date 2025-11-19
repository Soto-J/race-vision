use std::fmt;

use crate::utils::constants::size::VAR_HEADER_SIZE;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VarHeader {
    pub var_type: i32,
    pub offset: i32,
    pub count: i32,
    pub count_as_time: bool,
    pub name: [u8; 32],
    pub description: [u8; 64],
    pub unit: [u8; 32],

    padding: [u8; 3],
}

impl VarHeader {
    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < VAR_HEADER_SIZE {
            return None;
        }

        Some(Self {
            var_type: i32::from_le_bytes(buf[0..4].try_into().ok()?),
            offset: i32::from_le_bytes(buf[4..8].try_into().ok()?),
            count: i32::from_le_bytes(buf[8..12].try_into().ok()?),
            count_as_time: buf[12] != 0, // 0=false, non-zero=true
            name: buf[16..48].try_into().ok()?,
            description: buf[48..112].try_into().ok()?,
            unit: buf[112..144].try_into().ok()?,
            padding: [0; 3], // Padding bytes (13-15)
        })
    }

    pub fn name_str(&self) -> Option<&str> {
        let null_pos = self
            .name
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.name.len());

        str::from_utf8(&self.name[..null_pos]).ok()
    }

    pub fn description_str(&self) -> Option<&str> {
        let null_pos = self
            .description
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.description.len());

        std::str::from_utf8(&self.description[..null_pos]).ok()
    }

    pub fn unit_str(&self) -> Option<&str> {
        let null_pos = self
            .unit
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.unit.len());

        std::str::from_utf8(&self.unit[..null_pos]).ok()
    }
}

impl fmt::Display for VarHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VarHeader {}", self)
    }
}
