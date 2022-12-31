use handle_errors::Error;
use std::collections::HashMap;
use tracing::{info, instrument};

use crate::store::Store;
use crate::types::pagination::extract_pagination;
use crate::types::question::{Question, QuestionId};
use warp::hyper::StatusCode;
use warp::Rejection;

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("querying questions");
    if !params.is_empty() {
        let mut pagination = extract_pagination(params)?;
        info!(pagination = true);
        let res: Vec<Question> = store
            .get_questions()
            .read()
            .await
            .values()
            .cloned()
            .collect();
        if pagination.get_end() > res.len() {
            pagination.set_end(res.len());
        };
        if pagination.get_end() < pagination.get_start() || pagination.get_start() > res.len() {
            println!("Error here");
            return Err(Rejection::from(Error::MissingParameters));
        }
        let res = &res[pagination.get_start()..pagination.get_end()];
        Ok(warp::reply::json(&res))
    } else {
        info!(pagination = false);
        let res: Vec<Question> = store
            .get_questions()
            .read()
            .await
            .values()
            .cloned()
            .collect();
        Ok(warp::reply::json(&res))
    }
}

pub async fn get_question(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_questions().read().await.get(&QuestionId::new(id)) {
        Some(q) => Ok(warp::reply::json(q)),
        None => Err(Rejection::from(Error::QuestionNotFound)),
    }
}

pub async fn add_question(
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .get_questions()
        .write()
        .await
        .insert(question.get_id(), question);

    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

pub async fn update_question(
    id: String,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store
        .get_questions()
        .write()
        .await
        .get_mut(&QuestionId::new(id))
    {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }
    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

pub async fn delete_question(
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store
        .get_questions()
        .write()
        .await
        .remove(&QuestionId::new(id))
    {
        Some(_) => Ok(warp::reply::with_status("Question removed", StatusCode::OK)),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}
