use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tags(Option<Vec<String>>);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct QuestionId(String);
impl QuestionId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

/// Question struct, to represent a question in the API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Tags,
}

impl Question {
    pub fn get_id(&self) -> QuestionId {
        self.id.clone()
    }
}
