use tokio::sync::RwLock;

use super::question::question::{Question, QuestionId};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct Store {
    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            questions: Arc::new(RwLock::new(Self::init())),
        }
    }
    pub fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("Can't read questions.json")
    }
    pub fn get_questions(&self) -> &Arc<RwLock<HashMap<QuestionId, Question>>> {
        &self.questions
    }
}
