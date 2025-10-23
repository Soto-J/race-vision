#![allow(unused)]

#[repr(C)]
#[derive(Debug, Clone)]
pub struct DiskSubHeader {
    pub session_start_date: u64,
    pub session_start_time: f64,
    pub session_end_time: f64,
    pub session_lap_count: i32,
    pub session_record_count: i32,
}

impl DiskSubHeader {
    pub async fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 32 {
            return None;
        }

        Some(Self {
            session_start_date: u64::from_le_bytes(buf[0..8].try_into().ok()?),
            session_start_time: f64::from_le_bytes(buf[8..16].try_into().ok()?),
            session_end_time: f64::from_le_bytes(buf[16..24].try_into().ok()?),
            session_lap_count: i32::from_le_bytes(buf[24..28].try_into().ok()?),
            session_record_count: i32::from_le_bytes(buf[28..32].try_into().ok()?),
        })
    }
}
