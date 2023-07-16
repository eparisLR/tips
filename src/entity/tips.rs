use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tips {
    pub title: String,
    pub url: String,
    pub description: String,
    pub created_at: String,
    pub tags: Vec<String>
}