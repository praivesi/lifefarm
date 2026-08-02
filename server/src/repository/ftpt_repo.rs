/**
 * @file    ftpt_repo.rs
 * @brief   This module abstracts database operations for 'ftpt_tbl' table.
 *
 * @author  hansaem, oh (praivesi@gmail.com)
 * @date    2025/01/12 15:28 created.
 *
**/
use crate::entity::{NewFootprint, UpdateFootprint, Footprint};
use crate::config::database::get_connection;
use crate::schema::ftpt_tbl::{self, id, blpt_id, day_dt};
use crate::schema::ftpt_tbl::dsl::{ftpt_tbl as all_ftpts};

use diesel::expression_methods::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::sqlite::SqliteConnection;

use chrono::Utc;

pub fn upsert_footprint(update_blpt_id: i32, update_day_dt: i64, update_status: i32, update_note: Option<String>, update_photo: Option<Vec<u8>>) -> Footprint {
    match read_by_blpt_and_day(update_blpt_id, update_day_dt) {
        Some(existing) => {
            update_footprint(existing.id, update_status, update_note, update_photo)
                .expect("Error updating existing footprint")
        }
        None => add_footprint(update_blpt_id, update_day_dt, update_status, update_note, update_photo)
    }
}

pub fn add_footprint(new_blpt_id: i32, new_day_dt: i64, new_status: i32, new_note: Option<String>, new_photo: Option<Vec<u8>>) -> Footprint {
    let conn = &mut *get_connection();

    let new_ftpt = NewFootprint {
        blpt_id: new_blpt_id,
        day_dt: new_day_dt,
        status: new_status,
        note: new_note,
        photo: new_photo,
        ctime: Utc::now().timestamp(),
        mtime: Utc::now().timestamp()
    };

    diesel::insert_into(ftpt_tbl::table)
        .values(&new_ftpt)
        .execute(conn)
        .expect("Error saving new footprint");

    all_ftpts
        .order(id.desc())
        .first::<Footprint>(conn)
        .expect("Error loading the last inserted footprint")
}

pub fn read_ftpt(read_id: i32) -> Option<Footprint> {
    let conn = &mut *get_connection();

    read_ftpt_internal(conn, read_id)
}

fn read_ftpt_internal(conn: &mut SqliteConnection, read_id: i32) -> Option<Footprint> {
    all_ftpts
        .filter(id.eq(read_id))
        .first::<Footprint>(conn)
        .optional()
        .expect("Error reading footprint")
}

pub fn read_by_blpt_and_day(read_blpt_id: i32, read_day_dt: i64) -> Option<Footprint> {
    let conn = &mut *get_connection();

    all_ftpts
        .filter(blpt_id.eq(read_blpt_id))
        .filter(day_dt.eq(read_day_dt))
        .first::<Footprint>(conn)
        .optional()
        .expect("Error reading footprint by blueprint and day")
}

pub fn read_all_by_blpt(read_blpt_id: i32) -> Vec<Footprint> {
    let conn = &mut *get_connection();

    all_ftpts
        .filter(blpt_id.eq(read_blpt_id))
        .load::<Footprint>(conn)
        .expect("Error loading footprints for blueprint")
}

pub fn read_all() -> Vec<Footprint> {
    let conn = &mut *get_connection();

    all_ftpts.load::<Footprint>(conn).expect("Error loading footprints")
}

pub fn update_footprint(update_id: i32, update_status: i32, update_note: Option<String>, update_photo: Option<Vec<u8>>) -> Option<Footprint> {
    let conn = &mut *get_connection();

    let changeset = UpdateFootprint {
        status: update_status,
        note: update_note,
        photo: update_photo,
        mtime: Utc::now().timestamp()
    };

    let count = diesel::update(all_ftpts.filter(id.eq(update_id)))
                            .set(&changeset)
                            .execute(conn)
                            .expect("Error updating footprint");

    if 1 == count {
        read_ftpt_internal(conn, update_id)
    }
    else {
        None
    }
}

pub fn delete_footprint(delete_id: i32) -> usize {
    let conn = &mut *get_connection();

    diesel::delete(all_ftpts.filter(id.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting footprint")
}

pub fn delete_all_by_blpt(delete_blpt_id: i32) -> usize {
    let conn = &mut *get_connection();

    diesel::delete(all_ftpts.filter(blpt_id.eq(delete_blpt_id)))
        .execute(conn)
        .expect("Error deleting footprints for blueprint")
}
