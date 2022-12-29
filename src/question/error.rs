use warp::reject::Reject;

use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            Error::MissingParameters => {
                write!(f, "Missing parameter")
            }
            Error::QuestionNotFound => {
                write!(f, "Question not found")
            }
        }
    }
}
impl Reject for Error {}
