use serde::{Deserialize, Serialize};

// Session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Session {
    Practice,
    Qualy,
    Race,
}

impl Session {
    pub const ALL: [Session; 3] = [Session::Practice, Session::Qualy, Session::Race];

    pub fn as_str(self) -> &'static str {
        match self {
            Session::Practice => "practice",
            Session::Qualy => "qualy",
            Session::Race => "race",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "practice" => Some(Session::Practice),
            "qualy" => Some(Session::Qualy),
            "race" => Some(Session::Race),
            _ => None,
        }
    }
}
