use chrono::TimeZone;
use chrono::Utc;
use log::warn;

use axum::http::StatusCode;

use super::dto::req::*;
use super::dto::res::*;

use crate::entity::Blueprint;
use crate::util::rest::ErrorResult;
use crate::repository::blpt_repo;

pub fn get_user() -> Result<GetUserResponse, ErrorResult> {
    Ok(GetUserResponse {
        id: 1,
        name: "my_user".to_string(),
        predict_death_age: 80,
        birth_date: 725456933, // Sun Dec 27 1992 11:48:53 GMT+0000
    })
}

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

pub fn post_blpt(info: PostBlptRequest) -> Result<Blueprint, ErrorResult> {
    let entity = blpt_repo::add_blueprint(&info.goal, &info.desc, info.start_dt, info.end_dt);

    Ok(entity)
}

pub fn put_blpt(id: i32, info: PostBlptRequest) -> Result<Blueprint, ErrorResult> {
    if let Some(entity) = blpt_repo::update_blueprint(id, &info.goal, &info.desc, info.start_dt, info.end_dt) {
        Ok(entity)
    } else {
        Err(ErrorResult{
            code: StatusCode::INTERNAL_SERVER_ERROR,
            err_msg: format!("failed to update blueprint (id: {})", id)
        })
    }
}

pub fn delete_blpt(id: i32) -> Result<(), ErrorResult> {
    blpt_repo::delete_blueprint(id);

    Ok(())
}

pub fn get_ftpt() -> Result<GetFtptListResponse, ErrorResult> {
    Ok(GetFtptListResponse {
        ftpts: vec![
            GetFtptRepsponse {
                id: 10,
                blpt_id: 0,
                cert: None
            }
        ]
    })
}