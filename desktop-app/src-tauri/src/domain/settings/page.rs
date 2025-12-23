use crate::domain::display::DisplayIn;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PageSettings {
    pub page: String,
    pub setting: String,
    pub value: bool,
}

// API response types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PageSettingValue {
    Bool(bool),
    Section(HashMap<String, NestedSetting>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NestedSetting {
    pub is_active: bool,
    pub display_in: DisplayIn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageConfig {
    #[serde(flatten)]
    pub settings: HashMap<String, PageSettingValue>,
}
