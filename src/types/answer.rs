use serde::{Deserialize, Serialize};

use super::question::QuestionId;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct AnswerId(String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Answer {
    id: AnswerId,
    content: String,
    question_id: QuestionId,
}

impl AnswerId {
    pub fn new(id: String) -> AnswerId {
        AnswerId(id)
    }
}
impl Answer {
    pub fn new(id: AnswerId, content: String, question_id: QuestionId) -> Answer {
        Answer {
            id,
            content,
            question_id,
        }
    }
    pub fn get_id(&self) -> &AnswerId {
        &self.id
    }
}
