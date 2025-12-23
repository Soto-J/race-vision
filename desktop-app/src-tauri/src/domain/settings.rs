use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageSettings {
    pub page: String,
    pub setting: String,
    pub value: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageSettingWithDisplay {
    pub page: String,
    pub setting: String,
    pub value: bool,
    pub session: Option<String>,
    pub is_visible: Option<bool>,
}

pub enum Tab {
    General,
    Header,
    Content,
    Footer,
}

pub enum WebviewMode {
    Edit,
    Visible,
    Hidden,
}
