#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Thought {
    pub id: u32,
    pub timestamp: String,
    pub message: String,
    pub tags: Option<String>,
}