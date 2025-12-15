use crate::{
    domain::iracing_errors::{ClientError, ResolverError},
    iracing_client::telemetry::TelemetryType,
    utils::constants::size::{
        COUNT_AS_TIME_BYTE, COUNT_END, COUNT_START, DESCRIPTION_END, DESCRIPTION_START, KIND_END,
        KIND_START, NAME_END, NAME_START, OFFSET_END, OFFSET_START, UNIT_START, VAR_HEADER_SIZE,
    },
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

        let var_type = i32::from_le_bytes(read_slice!(buf, KIND_START, KIND_END, "var_type"));
        if TelemetryType::try_from(var_type).is_err() {
            return Err(ResolverError::InvalidVarKind(var_type).into());
        }

        let offset = i32::from_le_bytes(read_slice!(buf, OFFSET_START, OFFSET_END, "offset"));
        if offset < 0 {
            return Err(ResolverError::InvalidVarHeader(format!(
                "offset must be positive {}",
                offset
            ))
            .into());
        }

        let count = i32::from_le_bytes(read_slice!(buf, COUNT_START, COUNT_END, "count"));
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
            count_as_time: buf[COUNT_AS_TIME_BYTE] != 0,
            name: read_slice!(buf, NAME_START, NAME_END, "name"),
            description: read_slice!(buf, DESCRIPTION_START, DESCRIPTION_END, "description"),
            unit: read_slice!(buf, UNIT_START, VAR_HEADER_SIZE, "unit"),
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
