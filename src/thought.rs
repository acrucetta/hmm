
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Thought {
    pub id: u32,
    pub timestamp: String,
    pub message: String,
    pub tags: String,
}

impl Thought {
    pub fn to_csv(&self) -> String {
        format!(
            "{},{},{},{}",
            self.id, self.timestamp, self.message, self.tags
        )
    }
}