
use axum::{response::IntoResponse, Json, http::StatusCode, extract::Path};

use crate::util::rest::json_from;
use super::ui_core;
use super::dto::req::*;

pub async fn handle_get_user() -> impl IntoResponse {
    match ui_core::get_user() {
        Ok(res) => Json(res).into_response(),
        Err(result) => json_from(result).into_response()
    }
}

pub async fn handle_put_user(Json(info): Json<PostUserRequest>) -> impl IntoResponse {
    match ui_core::put_user(info) {
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

pub async fn handle_post_blpt_sync() -> impl IntoResponse {
    match ui_core::sync_blpt_from_notion().await {
        Ok(res) => Json(res).into_response(),
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

pub async fn handle_get_blpt_cell(Path(blpt_id): Path<i32>) -> impl IntoResponse {
    match ui_core::gen_blpt_cells(blpt_id) {
        Ok(res) => Json(res).into_response(),
        Err(result) => json_from(result).into_response()
    }
}

pub async fn handle_post_blpt_cell(Path(blpt_id): Path<i32>, Json(info): Json<PostFtptRequest>) -> impl IntoResponse {
    match ui_core::post_ftpt_cell(blpt_id, info) {
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

pub async fn handle_get_lifefarm() -> impl IntoResponse {
    match ui_core::get_lifefarm() {
        Ok(res) => Json(res).into_response(),
        Err(result) => json_from(result).into_response()
    }
}