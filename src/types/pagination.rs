use std::collections::HashMap;

use handle_errors::Error;

/// Pagination struct which is getting extract
/// from query params
#[derive(Default, Debug, PartialEq)]
pub struct Pagination {
    /// The index of the last item which has to be returned
    // pub end: usize,
    pub limit: Option<i32>,
    /// The index of the first item which has to be returned
    // pub start: usize,
    pub offset: i32,
}

/// Extract query parameters from the `/questions` route
/// # Example query
/// GET requests to this route can have a pagination attached so we just
/// return the questions we need
/// `/questions?start=1&end=10`
/// # Example usage
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
    if params.contains_key("limit") && params.contains_key("offset") {
        // if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            // Takes the "limit" parameter in the query and
            // try to convert it to an number
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<i32>()
                    .map_err(Error::ParseError)?,
            ),
            // Takes the "offset" parameter in the query and
            // try to convert it to an number
            offset: params
                .get("offset")
                .unwrap()
                .parse::<i32>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  TEST: pagination ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
#[cfg(test)]
mod pagination_tests {

    use super::{extract_pagination, Error, HashMap, Pagination};

    #[test]
    fn valid_pagination() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("1"));
        params.insert(String::from("offset"), String::from("1"));
        let pagination_result = extract_pagination(params);
        let expected = Pagination {
            limit: Some(1),
            offset: 1,
        };
        assert_eq!(pagination_result.unwrap(), expected);
    }

    #[test]
    fn missing_offset_parameter() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("1"));

        let pagination_result = format!("{}", extract_pagination(params).unwrap_err());
        let expected = format!("{}", Error::MissingParameters);

        assert_eq!(pagination_result, expected);
    }

    #[test]
    fn missing_limit_parameter() {
        let mut params = HashMap::new();
        params.insert(String::from("offset"), String::from("1"));

        let pagination_result = format!("{}", extract_pagination(params).unwrap_err());
        let expected = format!("{}", Error::MissingParameters);

        assert_eq!(pagination_result, expected);
    }

    #[test]
    fn wrong_offset_type() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("1"));
        params.insert(String::from("offset"), String::from("NOT_A_NUMBER"));

        let pagination_result = format!("{}", extract_pagination(params).unwrap_err());
        let expected = String::from("Cannot parse parameter: invalid digit found in string");

        assert_eq!(pagination_result, expected);
    }

    #[test]
    fn wrong_limit_type() {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("NOT_A_NUMBER"));
        params.insert(String::from("offset"), String::from("1"));

        let pagination_result = format!("{}", extract_pagination(params).unwrap_err());
        let expected = String::from("Cannot parse parameter: invalid digit found in string");

        assert_eq!(pagination_result, expected);
    }
}
