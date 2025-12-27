use crate::domain::display::DisplayIn;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PageSettings {
    pub page: String,
    pub setting: String,
    pub value: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PageSettingValue {
    Bool(bool),
    Section(HashMap<String, NestedSetting>),
    GeneralSection(HashMap<String, GeneralSetting>),
}

impl PageSettingValue {
    pub fn as_section_mut(&mut self) -> &mut HashMap<String, NestedSetting> {
        match self {
            Self::Section(map) => map,
            _ => panic!("PageSettingValue was not a Section"),
        }
    }

    pub fn as_general_section_mut(&mut self) -> &mut HashMap<String, GeneralSetting> {
        match self {
            Self::GeneralSection(map) => map,
            _ => panic!("PageSettingValue was not a GeneralSection"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NestedSetting {
    pub is_active: bool,
    pub display_in: DisplayIn,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralSetting {
    pub is_active: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageConfig {
    #[serde(flatten)]
    pub settings: HashMap<String, PageSettingValue>,
}

impl PageConfig {
    pub fn section_mut(&mut self, name: impl Into<String>) -> &mut HashMap<String, NestedSetting> {
        self.settings
            .entry(name.into())
            .or_insert_with(|| PageSettingValue::Section(HashMap::new()))
            .as_section_mut()
    }

    pub fn general_section_mut(&mut self, name: &str) -> &mut HashMap<String, GeneralSetting> {
        self.settings
            .entry(name.to_string())
            .or_insert_with(|| PageSettingValue::GeneralSection(HashMap::new()))
            .as_general_section_mut()
    }
}
