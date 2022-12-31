use tokio::sync::RwLock;

use std::{collections::HashMap, sync::Arc};

use crate::types::{
    answer::{Answer, AnswerId},
    question::{Question, QuestionId},
};

#[derive(Clone, Debug)]
pub struct Store {
    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("Can't read questions.json")
    }
    pub fn get_questions(&self) -> &Arc<RwLock<HashMap<QuestionId, Question>>> {
        &self.questions
    }
    pub fn get_anwsers(&self) -> &Arc<RwLock<HashMap<AnswerId, Answer>>> {
        &self.answers
    }
}
