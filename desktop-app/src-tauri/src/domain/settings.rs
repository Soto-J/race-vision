use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PageSettings {
    pub page: String,
    pub setting: String,
    pub value: i64, // SQLite stores booleans as INTEGER
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
