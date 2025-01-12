
use axum::{response::IntoResponse, Json, http::StatusCode};

use crate::util::rest::{json_from, json_from_500, RestResult};
use crate::enums::AsstIdStatus;

use super::dto::res::{GetAsstIdErrorResponse, GetAsstIdResponse};
use super::ui_core::{self, *};
use super::dto::req::*;

pub async fn handle_get_asst_id() -> impl IntoResponse {
    let (status, asst_id, reg_time) = ui_core::get_asst_id();

    if AsstIdStatus::None == status {
        json_from_500("invalid ASST_ID registration state detected.").into_response()
    } else if AsstIdStatus::NotRegistered == status {
        StatusCode::NOT_FOUND.into_response()
    } else if AsstIdStatus::Registered == status {
        (
            StatusCode::BAD_REQUEST,
            Json(
                GetAsstIdErrorResponse {
                    register_time: reg_time
                })
        ).into_response()
    } else {
        Json(
            GetAsstIdResponse {
                asset_id: asst_id,
                register_time: reg_time
            }).into_response()
    }
}

pub async fn handle_get_system_list() -> impl IntoResponse {
    Json(ui_core::get_system_name_list()).into_response()
}

pub async fn handle_get_asst() -> impl IntoResponse {
    match ui_core::get_asst() {
        Ok(response) => Json(response).into_response(),
        Err(result) => json_from(result).into_response()
    }
}

pub async fn handle_put_user(Json(info): Json<PutUserRequest>) -> impl IntoResponse {
    match ui_core::update_user(info) {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(result) => json_from(result).into_response()
    }
}

pub async fn handle_put_asst_mgmt(Json(info): Json<PutAsstMgmtRequest>) -> impl IntoResponse {
    match ui_core::update_asst_mgmt(info) {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(result) => json_from(result).into_response()
    }
}

pub async fn handle_get_agent() -> impl IntoResponse {
    Json(ui_core::get_agent()).into_response()
}

pub async fn handle_post_asst(Json(info): Json<PostAsstRequest>) -> impl IntoResponse {
    match ui_core::post_asset(info) {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(result ) => json_from(result).into_response()
    }
}

pub async fn handle_post_validate(Json(info): Json<PostValidateRequest>) -> impl IntoResponse {
    match ui_core::post_validate(info) {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(result ) => json_from(result).into_response()
    }
}

pub async fn handle_post_unlock(Json(info): Json<PostUnlockRequest>) -> impl IntoResponse {
    match ui_core::post_unlock(info) {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(result ) => json_from(result).into_response()
    }
}

pub async fn handle_post_intgr_log(Json(info): Json<PostLogListRequest>) -> impl IntoResponse {
    // match ui_core::get_intgr_log(info) {
    //     Ok(res) => Json(res).into_response(),
    //     Err(result ) => json_from(result).into_response()
    // }

    Json(ui_core::get_intgr_log(info)).into_response()
}

pub async fn handle_get_setting() -> impl IntoResponse {
    match ui_core::get_setting() {
        Ok(res) => Json(res).into_response(),
        Err(result ) => json_from(result).into_response()
    }
}

pub async fn handle_get_delete() -> impl IntoResponse {
    match ui_core::get_delete() {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(result ) => json_from(result).into_response()
    }
}