use handle_errors::Error;
use std::{collections::HashMap, usize};

/// Pagination struct, which is gettings extract
/// for query params
#[derive(Debug)]
pub struct Pagination {
    start: usize,
    end: usize,
}

impl Pagination {
    /// Returns the `end` of the Pagination
    pub fn get_end(&self) -> usize {
        self.end
    }
    /// Returns the `start` of the Pagination
    pub fn get_start(&self) -> usize {
        self.start
    }
    /// Sets the `end` value of the Pagination
    pub fn set_end(&mut self, end: usize) {
        self.end = end;
    }
}

/// Creates a new Pagination struct instance from query params.
/// # EXAMPLE QUERY
/// Return the questions using pagination
/// `/questions?start=0&end=10`
/// ```rust
/// use std::collections::HashMap;
///
/// let mut query = HashMap::new();
/// query.insert("start".to_string(), "1".to_string());
/// query.insert("end".to_string(), "10".to_string());
/// let p = pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.start, 1);
/// assert_eq!(p.end, 10);
/// ```
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
