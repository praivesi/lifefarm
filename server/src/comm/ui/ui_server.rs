
use axum::{response::IntoResponse, Json, http::StatusCode};

use crate::util::rest::{json_from, json_from_500, RestResult};
use super::ui_core::{self, *};
use super::dto::req::*;

pub async fn handle_get_bpnt() -> impl IntoResponse {
    StatusCode::NO_CONTENT.into_response()
}