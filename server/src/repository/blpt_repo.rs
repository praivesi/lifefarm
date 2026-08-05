/**
 * @file    blpt_repo.rs
 * @brief   This module abstracts database operations for 'blpt_tbl' table.
 *
 * @author  hansaem, oh (praivesi@gmail.com)
 * @date    2024/09/18 18:57 created.
 * 
**/
use crate::entity::{NewBluprint, UpdateBlueprint, Blueprint};
use crate::config::database::get_connection;
use crate::schema::blpt_tbl::{self, id};
use crate::schema::blpt_tbl::dsl::{blpt_tbl as all_blpts};

use diesel::expression_methods::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::sqlite::SqliteConnection;

use chrono::Utc;


// TODO: error handling in repository module

pub fn add_blueprint(goal: &str, desc: &str, start_dt: i64, end_dt: i64, parent_id: Option<i32>) -> Blueprint {
    let conn = &mut *get_connection();

    let new_blpt = NewBluprint {
        goal: goal.to_string(),
        desc: desc.to_string(),
        start_dt,
        end_dt,
        ctime: Utc::now().timestamp(),
        mtime: Utc::now().timestamp(),
        parent_id
    };

    diesel::insert_into(blpt_tbl::table)
        .values(&new_blpt)
        .execute(conn)
        .expect("Error saving new blueprint");

    all_blpts
        .order(id.desc())
        .first::<Blueprint>(conn)
        .expect("Error loading the last inserted blueprint")
}

pub fn read_all() -> Vec<Blueprint> {
    let conn = &mut *get_connection();

    all_blpts.load::<Blueprint>(conn).expect("Error loading blueprints")
}

pub fn read_blpt(read_id: i32) -> Option<Blueprint> {
    let conn = &mut *get_connection();

    read_blpt_internal(conn, read_id)
}

pub fn find_by_goal_and_parent(find_goal: &str, find_parent_id: Option<i32>) -> Option<Blueprint> {
    use crate::schema::blpt_tbl::{goal, parent_id};

    let conn = &mut *get_connection();

    let query = all_blpts.filter(goal.eq(find_goal));

    match find_parent_id {
        Some(pid) => query.filter(parent_id.eq(pid)).first::<Blueprint>(conn),
        None => query.filter(parent_id.is_null()).first::<Blueprint>(conn),
    }
    .optional()
    .expect("Error finding blueprint by goal and parent_id")
}

pub fn read_children(of_parent_id: i32) -> Vec<Blueprint> {
    use crate::schema::blpt_tbl::parent_id;

    let conn = &mut *get_connection();

    all_blpts
        .filter(parent_id.eq(of_parent_id))
        .load::<Blueprint>(conn)
        .expect("Error loading child blueprints")
}

pub fn update_blueprint(update_id: i32, update_goal: &str, update_desc: &str, update_start_dt: i64, update_end_dt: i64, update_parent_id: Option<i32>) -> Option<Blueprint> {
    let conn = &mut *get_connection();

    let changeset = UpdateBlueprint {
        goal: update_goal.to_string(),
        desc: update_desc.to_string(),
        start_dt: update_start_dt,
        end_dt: update_end_dt,
        mtime: Utc::now().timestamp(),
        parent_id: update_parent_id
    };

    let count = diesel::update(all_blpts.filter(id.eq(update_id)))
                            .set(&changeset)
                            .execute(conn)
                            .expect("Error updating blueprint");

    if 1 == count {
        read_blpt_internal(conn, update_id)
    }
    else {
        None
    }
}

pub fn delete_blueprint(delete_id: i32) {
    let conn = &mut *get_connection();

    diesel::delete(all_blpts.filter(id.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting blueprint");
}

fn read_blpt_internal(conn: &mut SqliteConnection, read_id: i32) -> Option<Blueprint> {
    all_blpts
        .filter(id.eq(read_id))
        .first::<Blueprint>(conn)
        .optional()
        .expect("Error reading blueprint")
}