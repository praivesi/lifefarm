use chrono::TimeZone;
use chrono::Utc;
use log::warn;

use axum::http::StatusCode;

use super::dto::req::*;
use super::dto::res::*;

use crate::util::rest::ErrorResult;

pub fn get_blpt() -> Result<GetBlptListResponse, ErrorResult> {
    Ok(GetBlptListResponse {
        blpts: vec![
            GetBlptResponse {
                id: 0,
                goal: "test".to_string(),
                exp_hour: 10,
                farm_portion: 10.0
            }
        ]
    })
}