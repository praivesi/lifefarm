/**
 * @file    user_repo.rs
 * @brief   This module abstracts database operations for 'user_tbl' table.
 *
 * @author  hansaem, oh (praivesi@gmail.com)
 * @date    2024/09/13 22:04 created.
 *
**/
use crate::entity::{NewUser, UpdateUser, User};
use crate::config::database::get_connection;
use crate::schema::user_tbl::{self, id};
use crate::schema::user_tbl::dsl::{user_tbl as all_users};

use diesel::expression_methods::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::sqlite::SqliteConnection;

use chrono::Utc;

pub fn add_user(name: &str, predict_death_age: i32, birth_date: i64) -> User {
    let conn = &mut *get_connection();

    let new_user = NewUser {
        name: name.to_string(),
        predict_death_age,
        birth_date,
        ctime: Utc::now().timestamp(),
        mtime: Utc::now().timestamp()
    };

    diesel::insert_into(user_tbl::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");

    all_users
        .order(id.desc())
        .first::<User>(conn)
        .expect("Error loading the last inserted user")
}

pub fn read_user(read_id: i32) -> Option<User> {
    let conn = &mut *get_connection();

    read_user_internal(conn, read_id)
}

fn read_user_internal(conn: &mut SqliteConnection, read_id: i32) -> Option<User> {
    all_users
        .filter(id.eq(read_id))
        .first::<User>(conn)
        .optional()
        .expect("Error reading user")
}

pub fn update_user(update_id: i32, update_name: &str, update_predict_death_age: i32, update_birth_date: i64) -> Option<User> {
    let conn = &mut *get_connection();

    let changeset = UpdateUser {
        name: update_name.to_string(),
        predict_death_age: update_predict_death_age,
        birth_date: update_birth_date,
        mtime: Utc::now().timestamp()
    };

    let count = diesel::update(all_users.filter(id.eq(update_id)))
                            .set(&changeset)
                            .execute(conn)
                            .expect("Error updating user");

    if 1 == count {
        read_user_internal(conn, update_id)
    }
    else {
        None
    }
}

pub fn delete_user(delete_id: i32) {
    let conn = &mut *get_connection();

    diesel::delete(all_users.filter(id.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting user");
}
