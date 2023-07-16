use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tips {
    title: String,
    url: String,
    description: String,
    created_at: String,
    tags: Vec<String>
}