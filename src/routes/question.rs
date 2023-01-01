use handle_errors::Error;
use std::collections::HashMap;
use tracing::{event, instrument, Level};

use crate::store::Store;
use crate::types::pagination::{extract_pagination, Pagination};
use crate::types::question::{NewQuestion, Question};
use warp::hyper::StatusCode;

fn to_optu32(n: Option<u32>) -> Option<i32> {
    if let Some(x) = n {
        return Some(x as i32);
    }
    None
}

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "server1", Level::INFO, "querying questions");
    let mut pagination = Pagination::default();
    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }
    let res: Vec<Question> = match store
        .get_questions(to_optu32(pagination.limit), pagination.offset as i32)
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    };
    Ok(warp::reply::json(&res))
}

#[instrument]
pub async fn get_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let res: Question = match store.get_question(id).await {
        Ok(question) => question,
        Err(e) => return Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    };
    Ok(warp::reply::json(&res))
}

// Called inside routes a filter with a body argument which is deserialized to a struct using Serde
pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.add_question(new_question).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError(e)));
    }
    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.update_question(question, id).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    };
    Ok(warp::reply::json(&res))
}

pub async fn delete_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.delete_question(id).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError(e)));
    }
    Ok(warp::reply::with_status(
        format!("Question {} deleted", id),
        StatusCode::OK,
    ))
}
