use std::collections::HashMap;

use warp::hyper::StatusCode;

use crate::store::Store;
use crate::types::answer::{Answer, AnswerId};
use crate::types::question::QuestionId;

pub async fn add_answer(
    store: Store,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let answer = Answer::new(
        AnswerId::new("1".to_string()),
        params.get("content").unwrap().to_string(),
        QuestionId::new(params.get("questionId").unwrap().to_string()),
    );

    store
        .get_anwsers()
        .write()
        .await
        .insert(answer.get_id().clone(), answer);
    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}
