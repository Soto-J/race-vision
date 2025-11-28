use crate::{
    domain::iracing_errors::{ClientError, ResolverError},
    iracing_client::telemetry::VarKind,
    utils::constants::size::VAR_HEADER_SIZE,
};

use std::fmt;

macro_rules! read_slice {
    ($buf:expr, $start:expr, $end:expr, $field:expr) => {
        $buf[$start..$end]
            .try_into()
            .map_err(|_| ResolverError::InvalidVarHeader(format!("{} field corrupt", $field)))?
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct VarHeader {
    pub var_type: i32,
    pub offset: i32,
    pub count: i32,
    pub count_as_time: bool,
    pub name: [u8; 32],
    pub description: [u8; 64],
    pub unit: [u8; 32],
}

impl VarHeader {
    pub fn from_bytes(buf: &[u8]) -> Result<Self, ClientError> {
        if buf.len() < VAR_HEADER_SIZE {
            return Err(ResolverError::VarHeaderNotFound.into());
        }

        let var_type = i32::from_le_bytes(read_slice!(buf, 0, 4, "var_type"));

        if VarKind::try_from(var_type).is_err() {
            return Err(ResolverError::InvalidVarKind(var_type).into());
        }

        let offset = i32::from_le_bytes(read_slice!(buf, 4, 8, "offset"));
        let count = i32::from_le_bytes(read_slice!(buf, 8, 12, "count"));

        let count_as_time = buf[12] != 0;

        if offset < 0 {
            return Err(ResolverError::InvalidVarHeader(format!(
                "offset must be positive {}",
                offset
            ))
            .into());
        }

        if count <= 0 {
            return Err(ResolverError::InvalidVarHeader(format!(
                "count must be positive: {}",
                count
            ))
            .into());
        }

        Ok(Self {
            var_type,
            offset,
            count,
            count_as_time,
            name: read_slice!(buf, 16, 48, "name"),
            description: read_slice!(buf, 48, 112, "description"),
            unit: read_slice!(buf, 112, 144, "unit"),
        })
    }

    pub fn name_str(&self) -> Result<&str, ClientError> {
        let null_pos = self
            .name
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.name.len());

        let slice = str::from_utf8(&self.name[..null_pos]).map_err(|e| {
            ResolverError::InvalidHeaderField {
                field: "name_str".into(),
                reason: e.to_string(),
            }
        })?;

        Ok(slice)
    }

    pub fn description_str(&self) -> Result<&str, ClientError> {
        let null_pos = self
            .description
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.description.len());

        let slice = std::str::from_utf8(&self.description[..null_pos]).map_err(|e| {
            {
                ResolverError::InvalidHeaderField {
                    field: "description_str".into(),
                    reason: e.to_string(),
                }
            }
        })?;

        Ok(slice)
    }

    pub fn unit_str(&self) -> Result<&str, ClientError> {
        let null_pos = self
            .unit
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.unit.len());

        let slice = std::str::from_utf8(&self.unit[..null_pos]).map_err(|e| {
            ResolverError::InvalidHeaderField {
                field: "unit_str".into(),
                reason: e.to_string(),
            }
        })?;

        Ok(slice)
    }
}

impl fmt::Display for VarHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VarHeader(name=\"{}\", type={}, offset={}, count={})",
            self.name_str().unwrap_or("<invalid>"),
            self.var_type,
            self.offset,
            self.count,
        )
    }
}
