use super::Session;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayPageSettings {
    pub page: String,
    pub setting: String,
    pub session: String,
    pub is_visible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayKeys {
    page: String,
    setting: String,
    session: Session,
}

impl DisplayKeys {
    pub fn parse(page: impl Into<String>, setting: impl Into<String>, session: Session) -> Self {
        Self {
            page: page.into(),
            setting: setting.into(),
            session,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayIn {
    pub practice: bool,
    pub qualy: bool,
    pub race: bool,
}

impl DisplayIn {
    pub fn from_index(index: &HashMap<DisplayKeys, bool>, page: &str, setting: &str) -> Self {
        let mut display_in = DisplayIn {
            practice: false,
            qualy: false,
            race: false,
        };

        for session in Session::ALL {
            let visible = index
                .get(&DisplayKeys::parse(
                    page.to_string(),
                    setting.to_string(),
                    session,
                ))
                .copied()
                .unwrap_or(false);

            match session {
                Session::Practice => display_in.practice = visible,
                Session::Qualy => display_in.qualy = visible,
                Session::Race => display_in.race = visible,
            }
        }

        display_in
    }
}
