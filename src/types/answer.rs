use serde::{Deserialize, Serialize};

use super::question::QuestionId;

/// AnswerId `struct` to individualize each anwser for each Question.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct AnswerId(String);

/// Answer `struct` used to save answer of each questions.
/// Each `Answer` struct has a unique `AnswerId`
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
