use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tags(pub Option<Vec<String>>);

/// QuestionId. Each `Question` struct must have a unique `QuestionId`.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct QuestionId(pub i32);
impl QuestionId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }
}

/// Question struct, to represent a question in the API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Tags,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewQuestion {
    pub title: String,
    pub content: String,
    pub tags: Tags,
}

impl Question {
    pub fn new(id: QuestionId, title: String, content: String, tags: Tags) -> Question {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}
