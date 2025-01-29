
use axum::{response::IntoResponse, Json, http::StatusCode};

use crate::util::rest::json_from;
use super::ui_core::{self, *};
use super::dto::req::*;

pub async fn handle_get_user() -> impl IntoResponse {
    match ui_core::get_user() {
        Ok(res) => Json(res).into_response(),
        Err(result) => json_from(result).into_response()
    }
}

pub async fn handle_get_blpt() -> impl IntoResponse {
    match ui_core::get_blpt() {
        Ok(res) => Json(res).into_response(),
        Err(result) => json_from(result).into_response()
    }
}

pub async fn handle_get_ftpt() -> impl IntoResponse {
    match ui_core::get_ftpt() {
        Ok(res) => Json(res).into_response(),
        Err(result) => json_from(result).into_response()
    }
}