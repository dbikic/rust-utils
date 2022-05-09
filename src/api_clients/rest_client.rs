use crate::api_clients::errors::ApiCallError;
use crossterm::style::Stylize;

pub enum RestApiCallType {
    Get,
    Post,
}

pub async fn execute_request<Req: serde::Serialize, Res: serde::de::DeserializeOwned>(
    url: String,
    rest_api_call_type: RestApiCallType,
    token: Option<String>,
    request_body: Option<Req>,
    additional_headers: Vec<(String, String)>,
) -> Result<Res, ApiCallError> {
    let client = reqwest::Client::new();
    let mut response = match rest_api_call_type {
        RestApiCallType::Get => client.get(url),
        RestApiCallType::Post => client.post(url),
    };
    if let Some(token) = token {
        response = response.header("Authorization", format!("Bearer {}", token));
    }
    for additional_header in additional_headers {
        response = response.header(additional_header.0, additional_header.1);
    }
    let response = match request_body {
        Some(request_body) => response.json(&request_body).send().await,
        None => response.send().await,
    };
    match response {
        Ok(response) => map_response(response.json().await),
        Err(error) => Err(ApiCallError::RequestFailed(error.to_string())),
    }
}

pub async fn execute_request_and_log<Req: serde::Serialize>(
    url: String,
    rest_api_call_type: RestApiCallType,
    token: Option<String>,
    request_body: Option<Req>,
    additional_headers: Vec<(String, String)>,
) -> Result<(), ApiCallError> {
    let client = reqwest::Client::new();
    let mut response = match rest_api_call_type {
        RestApiCallType::Get => client.get(url),
        RestApiCallType::Post => client.post(url),
    };
    if let Some(token) = token {
        response = response.header("Authorization", format!("Bearer {}", token));
    }
    for additional_header in additional_headers {
        response = response.header(additional_header.0, additional_header.1);
    }
    match request_body {
        Some(request_body) => {
            let r = response.json(&request_body).send().await;
            match r {
                Ok(r) => {
                    println!("{}", "Status:".red());
                    println!("    {}", r.status());
                    println!("{}", "Headers:".red());
                    for x in r.headers() {
                        if let Ok(value) = x.1.to_str() {
                            println!("    {} - {}", x.0, value);
                        }
                    }
                    if let Ok(body) = r.text().await {
                        println!("{}", "Body:".red());
                        println!("    {}", body);
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        None => {
            let r = response.send().await;
            match r {
                Ok(r) => {
                    println!("{}", "Status:".red());
                    println!("    {}", r.status());
                    println!("{}", "Headers:".red());
                    for x in r.headers() {
                        if let Ok(value) = x.1.to_str() {
                            println!("    {} - {}", x.0, value);
                        }
                    }
                    if let Ok(body) = r.text().await {
                        println!("{}", "Body:".red());
                        println!("    {}", body);
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    };
    Ok(())
}

fn map_response<Res: serde::de::DeserializeOwned>(
    response: reqwest::Result<Res>,
) -> Result<Res, ApiCallError> {
    match response {
        Ok(response) => Ok(response),
        Err(error) => Err(ApiCallError::JsonParse(error.to_string())),
    }
}
