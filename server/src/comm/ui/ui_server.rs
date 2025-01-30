
use axum::{response::IntoResponse, Json, http::StatusCode, extract::Path};

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

pub async fn handle_post_blpt(Json(info): Json<PostBlptRequest>) -> impl IntoResponse {
    match ui_core::post_blpt(info) {
        Ok(entity) => Json(entity).into_response(),
        Err(result) => json_from(result).into_response()
    }
}

pub async fn handle_put_blpt(Path(blpt_id): Path<i32>, Json(info): Json<PostBlptRequest>) -> impl IntoResponse {
    match ui_core::put_blpt(blpt_id, info) {
        Ok(entity) => Json(entity).into_response(),
        Err(result) => json_from(result).into_response()
    }
}

pub async fn handle_delete_blpt(Path(blpt_id): Path<i32>) -> impl IntoResponse {
    match ui_core::delete_blpt(blpt_id) {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(result) => json_from(result).into_response()
    }
}

pub async fn handle_get_ftpt() -> impl IntoResponse {
    match ui_core::get_ftpt() {
        Ok(res) => Json(res).into_response(),
        Err(result) => json_from(result).into_response()
    }
}