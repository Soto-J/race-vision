use crate::domain::iracing_errors::ParseError;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum VarKind {
    Char8 = 0,
    Bool = 1,
    I32 = 2,
    Bitfield = 3,
    F32 = 4,
    F64 = 5,
}

impl VarKind {
    pub fn parse_to_value(&self, bytes: &[u8]) -> Result<TelemetryValue, ParseError> {
        match self {
            VarKind::Char8 => Ok(TelemetryValue::Chars8(bytes.to_vec())),
            VarKind::Bool => Ok(TelemetryValue::Bool(
                bytes.iter().map(|&b| b != 0).collect(),
            )),
            VarKind::I32 => Ok(TelemetryValue::I32(parse_le_chunks::<i32, 4>(bytes)?)),
            VarKind::Bitfield => Ok(TelemetryValue::Bitfield(parse_le_chunks::<u32, 4>(bytes)?)),
            VarKind::F32 => Ok(TelemetryValue::F32(parse_le_chunks::<f32, 4>(bytes)?)),
            VarKind::F64 => Ok(TelemetryValue::F64(parse_le_chunks::<f64, 8>(bytes)?)),
        }
    }
}

fn parse_le_chunks<T, const N: usize>(bytes: &[u8]) -> Result<Vec<T>, ParseError>
where
    T: FromLeBytes<N>,
{
    bytes
        .chunks_exact(N)
        .map(|chunk| {
            let array: [u8; N] = chunk
                .try_into()
                .map_err(|_| ParseError::InvalidChunk(format!("size={}", N)))?;

            Ok(T::from_le_bytes(array))
        })
        .collect()
}

impl TryFrom<i32> for VarKind {
    type Error = String;

    fn try_from(var_type: i32) -> Result<Self, Self::Error> {
        match var_type {
            0 => Ok(Self::Char8),
            1 => Ok(Self::Bool),
            2 => Ok(Self::I32),
            3 => Ok(Self::Bitfield),
            4 => Ok(Self::F32),
            5 => Ok(Self::F64),
            _ => Err(format!("Unknown variable type: {}", var_type)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TelemetryValue {
    Chars8(Vec<u8>),
    Bool(Vec<bool>),
    I32(Vec<i32>),
    Bitfield(Vec<u32>),
    F32(Vec<f32>),
    F64(Vec<f64>),
}

impl TelemetryValue {
    pub fn format(value: &TelemetryValue) -> String {
        match value {
            TelemetryValue::F32(vals) => {
                if vals.len() == 1 {
                    format!("{:.2}", vals[0])
                } else {
                    format!("{:?}", vals)
                }
            }
            TelemetryValue::F64(vals) => {
                if vals.len() == 1 {
                    format!("{:.2}", vals[0])
                } else {
                    format!("{:?}", vals)
                }
            }
            TelemetryValue::I32(vals) => {
                if vals.len() == 1 {
                    format!("{}", vals[0])
                } else {
                    format!("{:?}", vals)
                }
            }
            TelemetryValue::Bool(vals) => {
                if vals.len() == 1 {
                    format!("{}", vals[0])
                } else {
                    format!("{:?}", vals)
                }
            }
            TelemetryValue::Bitfield(vals) => format!("{:?}", vals),
            TelemetryValue::Chars8(vals) => String::from_utf8_lossy(vals).to_string(),
        }
    }
}

trait FromLeBytes<const N: usize>: Sized {
    fn from_le_bytes(bytes: [u8; N]) -> Self;
}

impl FromLeBytes<4> for i32 {
    fn from_le_bytes(bytes: [u8; 4]) -> Self {
        i32::from_le_bytes(bytes)
    }
}
impl FromLeBytes<4> for u32 {
    fn from_le_bytes(bytes: [u8; 4]) -> Self {
        u32::from_le_bytes(bytes)
    }
}

impl FromLeBytes<4> for f32 {
    fn from_le_bytes(bytes: [u8; 4]) -> Self {
        f32::from_le_bytes(bytes)
    }
}

impl FromLeBytes<8> for f64 {
    fn from_le_bytes(bytes: [u8; 8]) -> Self {
        f64::from_le_bytes(bytes)
    }
}
