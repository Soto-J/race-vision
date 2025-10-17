use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct VarHeader {
    pub var_type: i32,
    pub offset: i32,
    pub count: i32,
    pub count_as_time: bool,

    padding: [u8; 3],

    pub description: [u8; 64],
    pub unit: [u8; 32],
}
