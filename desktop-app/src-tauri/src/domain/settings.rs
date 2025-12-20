use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {}

pub struct Page {}

pub enum Tab {
    General,
    Header,
    Content,
    Footer,
}
