use crate::api_clients::errors::{extract_error_message, ApiCallError};

pub async fn execute_graphql_request<Req: serde::Serialize, Res: serde::de::DeserializeOwned>(
    url: String,
    token: &str,
    request_body: Req,
) -> Result<Res, ApiCallError> {
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&request_body)
        .send()
        .await;
    match response {
        Ok(response) => map_response(response.json().await),
        Err(error) => Err(ApiCallError::RequestFailed(error.to_string())),
    }
}

fn map_response<Res: serde::de::DeserializeOwned>(
    response: reqwest::Result<graphql_client::Response<Res>>,
) -> Result<Res, ApiCallError> {
    match response {
        Ok(response) => match response.data {
            None => Err(ApiCallError::DataError(extract_error_message(
                response.errors,
            ))),
            Some(response) => Ok(response),
        },
        Err(error) => Err(ApiCallError::JsonParse(error.to_string())),
    }
}
