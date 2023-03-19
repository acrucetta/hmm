
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Thought {
    pub id: String,
    pub timestamp: String,
    pub message: String,
    pub tags: String,
}