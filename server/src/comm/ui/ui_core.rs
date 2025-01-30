use chrono::TimeZone;
use chrono::Utc;
use log::warn;

use axum::http::StatusCode;

use super::dto::req::*;
use super::dto::res::*;

use crate::enums::BlptCellType;
use crate::entity::Blueprint;
use crate::repository::blpt_repo;
use crate::util::rest::ErrorResult;
use crate::util::time;

pub fn get_user() -> Result<GetUserResponse, ErrorResult> {
    Ok(GetUserResponse {
        id: 1,
        name: "my_user".to_string(),
        predict_death_age: 80,
        birth_date: 725456933, // Sun Dec 27 1992 11:48:53 GMT+0000
    })
}

pub fn get_blpt() -> Result<GetBlptListResponse, ErrorResult> {
    let blpts = blpt_repo::read_all();

    Ok(GetBlptListResponse { blpts })
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

pub fn gen_blpt_cells(id: i32) -> Result<GetBlptCellListResponse, ErrorResult> {
    if let Some(entity) = blpt_repo::read_blpt(id) {
        let cells: Vec<GetBlptCellResponse> = gen_cells(entity.clone());

        // TODO: Ensure must have at least 1 cell in the vector
        Ok(GetBlptCellListResponse{
            blpt: entity,
            cell_start_dt: cells.first().unwrap().date,
            cell_end_dt: cells.last().unwrap().date,
            cells
        })

    } else {
        Err(ErrorResult{
            code: StatusCode::NOT_FOUND,
            err_msg: format!("Blueprint with id '{}' doesn't exist.", id)
        })
    }
}

fn gen_cells(blpt: Blueprint) -> Vec<GetBlptCellResponse> {
    let start_weekday = time::get_weekday(blpt.start_dt);
    let end_weekday = time::get_weekday(blpt.end_dt);

    let front_padding_cnt = (start_weekday as i32 + 1) % 7; // front padding - (weekday + 1) % 7
    let tail_padding_cnt = (5 - end_weekday as i32) % 7; // end padding - (5 - weekday) % 7

    let start_date = time::to_midnight(blpt.start_dt);
    let end_date = time::to_end_of_day(blpt.end_dt);

    let mut cells: Vec<GetBlptCellResponse> = vec![];

    // append front padding
    for i in 0..front_padding_cnt {
        cells.push(GetBlptCellResponse{
            date: time::add_days_to_utc(start_date, (i - front_padding_cnt).into()),
            status: BlptCellType::Padding
        });
    }

    // append data cells
    let mut cur_date = start_date;
    while cur_date < end_date {
        cells.push(GetBlptCellResponse{
            date: cur_date,
            status: BlptCellType::Pass,
        });

        cur_date = time::add_days_to_utc(cur_date, 1);
    }

    // append tail padding
    for i in 0..tail_padding_cnt {
        cells.push(GetBlptCellResponse{
            date: time::add_days_to_utc(end_date, (i + 1).into()),
            status: BlptCellType::Padding
        })
    }

    cells
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