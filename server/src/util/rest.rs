use axum::{Json, http::StatusCode};
use axum_server::tls_rustls::RustlsConfig;
use hyper::HeaderMap;
use serde::{Deserialize, Serialize};

const SERVER_CRT: &str = include_str!("../../resources/ssl/server.crt");
const SERVER_KEY: &str = include_str!("../../resources/ssl/server.key");

pub struct RestResult {
    pub code: StatusCode,
    pub err_msg: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerErrorResponse {
    pub err_msg: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestResponse {
    pub err_msg: String
}

// TODO: should return "err_msg" field instead of "msg" in '/front/' API error case
impl RestResponse {
    pub fn error(err_msg: String) -> Self {
        RestResponse {
            err_msg
        }
    }
}

pub fn json_from(result: RestResult) -> (StatusCode, axum::Json<RestResponse>) {
    (result.code, Json(RestResponse::error(result.err_msg)))
}

pub fn json_from_500(err_msg: &str) -> (StatusCode, axum::Json<ServerErrorResponse>) {
    (StatusCode::INTERNAL_SERVER_ERROR,
        Json(ServerErrorResponse {
            err_msg: err_msg.to_string()
    }))
}

pub fn get_bearer_token(headers: HeaderMap) -> String {
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_value) = auth_header.to_str() {
            if auth_value.starts_with("Bearer ") {
                return auth_value.trim_start_matches("Bearer ").to_string();
            }
        }
    }

    "".to_string()
}

pub async fn gen_tls_config() -> RustlsConfig {
    RustlsConfig::from_pem(
        SERVER_CRT.as_bytes().to_vec(),
        SERVER_KEY.as_bytes().to_vec())
        .await
        .expect("Failed to load SSL configuration")
}