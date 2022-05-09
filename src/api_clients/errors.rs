use graphql_client::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiCallError {
    #[error("Request failed: {0}")]
    RequestFailed(String),
    #[error("Json parse fail: {0}")]
    JsonParse(String),
    #[error("Server returned: {0}")]
    DataError(String),
}

pub fn extract_error_message(errors_maybe: Option<Vec<Error>>) -> String {
    match errors_maybe {
        None => "Unknown error".to_string(),
        Some(errors) => {
            let errors: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
            errors.join(" | ")
        }
    }
}
