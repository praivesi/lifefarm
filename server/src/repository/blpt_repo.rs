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

pub fn add_blueprint(conn: &mut SqliteConnection, goal: &str, exp_hour: i32, farm_portion: f32) -> Blueprint {
    let new_blpt = NewBluprint {
        goal: goal.to_string(),
        exp_hour,
        farm_portion,
        ctime: Utc::now().timestamp(),
        mtime: Utc::now().timestamp()
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

pub fn read_blpt(conn: &mut SqliteConnection, read_id: i32) -> Option<Blueprint> {
    all_blpts
        .filter(id.eq(read_id))
        .first::<Blueprint>(conn)
        .optional()
        .expect("Error reading blueprint")
}

pub fn update_blueprint(conn: &mut SqliteConnection, update_id: i32, update_goal: &str, update_exp_hour: i32, update_farm_portion: f32) -> Option<Blueprint> {
    let changeset = UpdateBlueprint {
        goal: update_goal.to_string(),
        exp_hour: update_exp_hour,
        farm_portion: update_farm_portion,
        mtime: Utc::now().timestamp()
    };

    let count = diesel::update(all_blpts.filter(id.eq(update_id)))
                            .set(&changeset)
                            .execute(conn)
                            .expect("Error updating blueprint");

    if 1 == count {
        read_blpt(conn, update_id)
    }
    else {
        None
    }
}

pub fn delete_blueprint(conn: &mut SqliteConnection, delete_id: i32) {
    diesel::delete(all_blpts.filter(id.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting blueprint");
}