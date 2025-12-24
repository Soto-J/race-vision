use crate::domain::display::DisplayIn;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

impl PageSettingValue {
    pub fn as_section_mut(&mut self) -> Option<&mut HashMap<String, NestedSetting>> {
        match self {
            PageSettingValue::Section(map) => Some(map),
            _ => None,
        }
    }
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

impl PageConfig {
    pub fn section_mut(&mut self, name: &str) -> &mut HashMap<String, NestedSetting> {
        self.settings
            .entry(name.to_string())
            .or_insert_with(|| PageSettingValue::Section(HashMap::new()))
            .as_section_mut()
            .expect("Section was overwritten with Bool")
    }
}
