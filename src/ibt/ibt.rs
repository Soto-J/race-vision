use crate::sdk::{DiskSubHeader, Header, VarHeader};

use std::{collections::HashMap, fs::File, sync::Arc};

#[derive(Debug, Default)]
pub struct IBT {
    ibt_file: Option<File>,
    shared_mem: Arc<[u8]>,
    header: Option<Header>,
    disk_header: Option<DiskSubHeader>,
    var_headers: Option<VarHeader>,
    var_headers_dict: HashMap<String, VarHeader>,
    var_headers_names: Option<Vec<String>>,
    session_info_dict: HashMap<String, serde_yaml::Value>,
}

impl IBT {
    // pub fn new() -> Self {

    // }

    // pub fn get_item(&self, key: &str) {
    //     if let Some(disk_header) = &self.disk_header {
    //         disk_header.session_record_count - 1
    //     }
    // }
}
