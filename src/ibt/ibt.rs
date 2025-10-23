use crate::sdk::{DiskSubHeader, Header, VarHeader};

use std::{default, sync::Arc};

#[derive(Debug, Default)]
pub struct IBT {
    //  ibt_file : Option<>,
    shared_mem: Arc<[u8]>,
    header: Option<Header>,
    disk_header: Option<DiskSubHeader>,
    var_headers: Option<VarHeader>,
    // var_headers_dict : Option<>,
    // var_headers_names : Option<>,
    // session_info_dict : Option<>,
}

impl IBT {
    // pub fn new() -> Self {

    // }
}
