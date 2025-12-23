use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageSettings {
    pub page: String,
    pub setting: String,
    pub value: bool,
}

// API response types
#[derive(Serialize)]
#[serde(untagged)]
pub enum PageSettingValue {
    Bool(bool),
    Section(HashMap<String, NestedSetting>),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NestedSetting {
    pub is_active: bool,
    pub display_in: DisplayIn,
}

#[derive(Serialize)]
pub struct DisplayIn {
    pub practice: bool,
    pub qualy: bool,
    pub race: bool,
}

#[derive(Serialize)]
pub struct PageConfig {
    #[serde(flatten)]
    pub settings: HashMap<String, PageSettingValue>,
}

pub enum WebviewMode {
    Edit,
    Visible,
    Hidden,
}
