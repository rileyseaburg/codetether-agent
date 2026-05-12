use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TabInfo {
    pub index: usize,
    pub url: String,
    pub title: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TabList {
    pub current: Option<usize>,
    pub tabs: Vec<TabInfo>,
}
