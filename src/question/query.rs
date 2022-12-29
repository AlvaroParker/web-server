use super::{error::Error, question::Question};
use crate::store::Store;
use std::collections::HashMap;
use warp::Rejection;

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParameters)
}

pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !params.is_empty() {
        let mut pagination = extract_pagination(params)?;
        let res: Vec<Question> = store
            .get_questions()
            .read()
            .await
            .values()
            .cloned()
            .collect();
        if pagination.end > res.len() {
            pagination.end = res.len();
        };
        if pagination.end < pagination.start || pagination.start > res.len() {
            println!("Error here");
            return Err(Rejection::from(Error::MissingParameters));
        }
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
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
