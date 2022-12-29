use super::error::Error;
use super::question::QuestionId;
use crate::{question::question::Question, store::Store};
use warp::hyper::StatusCode;

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
