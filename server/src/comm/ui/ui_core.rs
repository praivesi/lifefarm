use std::collections::HashMap;

use log::warn;

use axum::http::StatusCode;

use super::dto::req::*;
use super::dto::res::*;

use crate::enums::BlptCellType;
use crate::entity::{Blueprint, Footprint};
use crate::repository::{blpt_repo, ftpt_repo, user_repo};
use crate::util::rest::ErrorResult;
use crate::util::time;

const DEFAULT_USER_ID: i32 = 1;

pub fn get_user() -> Result<GetUserResponse, ErrorResult> {
    let user = match user_repo::read_user(DEFAULT_USER_ID) {
        Some(user) => user,
        None => user_repo::add_user("my_user", 80, time::today_midnight())
    };

    Ok(GetUserResponse {
        id: user.id,
        name: user.name,
        predict_death_age: user.predict_death_age,
        birth_date: user.birth_date
    })
}

pub fn put_user(info: PostUserRequest) -> Result<GetUserResponse, ErrorResult> {
    let user = match user_repo::update_user(DEFAULT_USER_ID, &info.name, info.predict_death_age, info.birth_date) {
        Some(user) => user,
        None => user_repo::add_user(&info.name, info.predict_death_age, info.birth_date)
    };

    Ok(GetUserResponse {
        id: user.id,
        name: user.name,
        predict_death_age: user.predict_death_age,
        birth_date: user.birth_date
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
    ftpt_repo::delete_all_by_blpt(id);
    blpt_repo::delete_blueprint(id);

    Ok(())
}

pub fn gen_blpt_cells(id: i32) -> Result<GetBlptCellListResponse, ErrorResult> {
    if let Some(entity) = blpt_repo::read_blpt(id) {
        let footprints = ftpt_repo::read_all_by_blpt(id);
        let cells: Vec<GetBlptCellResponse> = gen_cells(entity.clone(), footprints);

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

fn gen_cells(blpt: Blueprint, footprints: Vec<Footprint>) -> Vec<GetBlptCellResponse> {
    let footprints_by_day: HashMap<i64, Footprint> = footprints
        .into_iter()
        .map(|ftpt| (ftpt.day_dt, ftpt))
        .collect();

    let start_weekday = time::get_weekday(blpt.start_dt);
    let end_weekday = time::get_weekday(blpt.end_dt);

    let front_padding_cnt = (start_weekday as i32 + 1) % 7; // front padding - (weekday + 1) % 7
    let tail_padding_cnt = (5 - end_weekday as i32) % 7; // end padding - (5 - weekday) % 7

    let start_date = time::to_midnight(blpt.start_dt);
    let end_date = time::to_end_of_day(blpt.end_dt);
    let today = time::today_midnight();

    let mut cells: Vec<GetBlptCellResponse> = vec![];

    // append front padding
    for i in 0..front_padding_cnt {
        cells.push(GetBlptCellResponse{
            date: time::add_days_to_utc(start_date, (i - front_padding_cnt).into()),
            status: BlptCellType::Padding,
            is_today: false,
            note: None
        });
    }

    // append data cells
    let mut cur_date = start_date;
    while cur_date < end_date {
        let footprint = footprints_by_day.get(&cur_date);

        let status = match footprint {
            Some(ftpt) if ftpt.status == 1 => BlptCellType::Pass,
            Some(_) => BlptCellType::Fail,
            None if cur_date < today => BlptCellType::Fail,
            None => BlptCellType::Future
        };

        cells.push(GetBlptCellResponse{
            date: cur_date,
            status,
            is_today: cur_date == today,
            note: footprint.and_then(|ftpt| ftpt.note.clone())
        });

        cur_date = time::add_days_to_utc(cur_date, 1);
    }

    // append tail padding
    for i in 0..tail_padding_cnt {
        cells.push(GetBlptCellResponse{
            date: time::add_days_to_utc(end_date, (i + 1).into()),
            status: BlptCellType::Padding,
            is_today: false,
            note: None
        })
    }

    cells
}

pub fn post_ftpt_cell(blpt_id: i32, info: PostFtptRequest) -> Result<GetFtptRepsponse, ErrorResult> {
    let blpt = match blpt_repo::read_blpt(blpt_id) {
        Some(blpt) => blpt,
        None => return Err(ErrorResult{
            code: StatusCode::NOT_FOUND,
            err_msg: format!("Blueprint with id '{}' doesn't exist.", blpt_id)
        })
    };

    let day_dt = time::to_midnight(info.day_dt);

    if day_dt < time::to_midnight(blpt.start_dt) || day_dt > time::to_midnight(blpt.end_dt) {
        return Err(ErrorResult{
            code: StatusCode::BAD_REQUEST,
            err_msg: format!("day_dt '{}' is out of blueprint's range.", info.day_dt)
        });
    }

    let ftpt = ftpt_repo::upsert_footprint(blpt_id, day_dt, info.status, info.note, None);

    Ok(GetFtptRepsponse {
        id: ftpt.id,
        blpt_id: ftpt.blpt_id,
        day_dt: ftpt.day_dt,
        status: ftpt.status,
        note: ftpt.note
    })
}

pub fn get_ftpt() -> Result<GetFtptListResponse, ErrorResult> {
    let ftpts = ftpt_repo::read_all()
        .into_iter()
        .map(|ftpt| GetFtptRepsponse {
            id: ftpt.id,
            blpt_id: ftpt.blpt_id,
            day_dt: ftpt.day_dt,
            status: ftpt.status,
            note: ftpt.note
        })
        .collect();

    Ok(GetFtptListResponse { ftpts })
}

pub fn get_lifefarm() -> Result<GetLifeFarmResponse, ErrorResult> {
    let user = match user_repo::read_user(DEFAULT_USER_ID) {
        Some(user) => user,
        None => user_repo::add_user("my_user", 80, time::today_midnight())
    };

    let blpts = blpt_repo::read_all();
    let all_footprints = ftpt_repo::read_all();

    let mut pass_by_day: HashMap<i64, i32> = HashMap::new();
    for ftpt in &all_footprints {
        if ftpt.status == 1 {
            *pass_by_day.entry(ftpt.day_dt).or_insert(0) += 1;
        }
    }

    let birth_date = time::to_midnight(user.birth_date);
    let today = time::today_midnight();

    if today < birth_date {
        warn!("today ({}) is before birth_date ({})", today, birth_date);
    }

    let mut cells: Vec<LifeFarmCell> = vec![];
    let mut cur_date = birth_date;

    while cur_date <= today {
        let active_blpt_cnt = blpts.iter()
            .filter(|b| time::to_midnight(b.start_dt) <= cur_date && cur_date <= time::to_midnight(b.end_dt))
            .count();

        let (target_rate, actual_rate) = if active_blpt_cnt == 0 {
            (0.0, 0.0)
        } else {
            let pass_cnt = *pass_by_day.get(&cur_date).unwrap_or(&0);
            (1.0, pass_cnt as f32 / active_blpt_cnt as f32)
        };

        cells.push(LifeFarmCell {
            day_dt: cur_date,
            target_rate,
            actual_rate
        });

        cur_date = time::add_days_to_utc(cur_date, 1);
    }

    Ok(GetLifeFarmResponse {
        birth_date: user.birth_date,
        predict_death_age: user.predict_death_age,
        cells
    })
}
