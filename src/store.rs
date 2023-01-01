use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use handle_errors::Error;

use crate::types::answer::{AnswerId, NewAnswer};
use crate::types::question::{NewQuestion, Tags};
use crate::types::{
    answer::Answer,
    question::{Question, QuestionId},
};

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {}", e),
        };
        Store {
            connection: db_pool,
        }
    }
    pub async fn get_questions(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Question>, sqlx::Error> {
        match sqlx::query("SELECT * FROM questions LIMIT $1 OFFSET $2;")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| {
                Question::new(
                    QuestionId::new(row.get("id")),
                    row.get("title"),
                    row.get("content"),
                    Tags(row.get("tags")),
                )
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }
    pub async fn add_question(&self, new_question: NewQuestion) -> Result<Question, sqlx::Error> {
        match sqlx::query(
            "INSERT INTO questions (title, content, tags)
                          VALUES ($1, $2, $3)
                          RETURNING id, title, content, tags",
        )
        .bind(new_question.title)
        .bind(new_question.content)
        .bind(new_question.tags.0)
        .map(|row: PgRow| {
            Question::new(
                QuestionId::new(row.get("id")),
                row.get("title"),
                row.get("content"),
                Tags(row.get("tags")),
            )
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    pub async fn update_question(
        &self,
        question: Question,
        question_id: i32,
    ) -> Result<Question, sqlx::Error> {
        match sqlx::query(
            "UPDATE question SET title = $1, content = $2, tags = $3
             WHERE id = $4
             RETURNING id, title, content, tags",
        )
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags.0)
        .bind(question_id)
        .map(|row: PgRow| {
            Question::new(
                QuestionId::new(row.get("id")),
                row.get("title"),
                row.get("content"),
                Tags(row.get("tags")),
            )
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_question(&self, question_id: i32) -> Result<bool, sqlx::Error> {
        match sqlx::query("DELETE FROM questions WHERE id=$1 RETURNING id")
            .bind(question_id)
            .fetch_one(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }

    pub async fn add_answer(&self, new_answer: NewAnswer) -> Result<Answer, Error> {
        match sqlx::query("INSERT INTO answers (content,question_id) VALUES ($1, $2) RETURNING id, content,question_id")
            .bind(new_answer.content)
            .bind(new_answer.question_id.0)
            .map(|row: PgRow| {
                Answer::new(
                    AnswerId::new(row.get("id")),
                    row.get("content"),
                    QuestionId::new(row.get("question_id")),
                )
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(answer) => Ok(answer),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn get_question(&self, question_id: i32) -> Result<Question, sqlx::Error> {
        match sqlx::query("SELECT id,title,content,tags FROM questions WHERE ID=$1")
            .bind(question_id)
            .map(|row: PgRow| {
                Question::new(
                    QuestionId(row.get("id")),
                    row.get("title"),
                    row.get("content"),
                    Tags(row.get("tags")),
                )
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(question) => Ok(question),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }
}
