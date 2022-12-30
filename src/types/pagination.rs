use handle_errors::Error;
use std::{collections::HashMap, usize};

#[derive(Debug)]
pub struct Pagination {
    start: usize,
    end: usize,
}

impl Pagination {
    pub fn get_end(&self) -> usize {
        self.end
    }
    pub fn get_start(&self) -> usize {
        self.start
    }
    pub fn set_end(&mut self, end: usize) {
        self.end = end;
    }
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
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
