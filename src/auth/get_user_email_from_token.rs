use crate::api_clients::rest_client;
use crate::api_clients::rest_client::RestApiCallType;
use crate::auth::extract_user_id::extract_user_id;
use anyhow::{bail, Error};
use serde::{Deserialize, Serialize};

pub async fn execute(
    user_token: Option<String>,
    auth_service_url: &str,
    auth_path: &str,
    service_token: &str,
) -> Result<String, Error> {
    match user_token {
        Some(user_token) => {
            let user_id: String = extract_user_id(user_token)?;
            check_token_against_auth_service(user_id, auth_service_url, auth_path, service_token)
                .await
        }
        None => bail!("Authorization header not provided!"),
    }
}

#[derive(Serialize, Deserialize)]
struct UserResponse {
    pub username: String,
}

async fn check_token_against_auth_service(
    user_id: String,
    auth_service_url: &str,
    auth_path: &str,
    service_token: &str,
) -> Result<String, Error> {
    let result = rest_client::execute_request::<UserResponse, UserResponse>(
        format!("{}{}{}", auth_service_url, auth_path, user_id),
        RestApiCallType::Get,
        Some(service_token.to_string()),
        None,
        vec![],
    );
    match result.await {
        Ok(response) => Ok(response.username),
        Err(_) => bail!("Invalid JWT token!"),
    }
}
