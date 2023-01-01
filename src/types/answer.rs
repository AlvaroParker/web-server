use serde::{Deserialize, Serialize};

use super::question::QuestionId;

/// AnswerId `struct` to individualize each anwser for each Question.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct AnswerId(i32);

/// Answer `struct` used to save answer of each questions.
/// Each `Answer` struct has a unique `AnswerId`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Answer {
    id: AnswerId,
    content: String,
    question_id: QuestionId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAnswer {
    pub content: String,
    pub question_id: QuestionId,
}

impl AnswerId {
    pub fn new(id: i32) -> AnswerId {
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
}
