/**
 * @file    cfg_user.rs
 * @brief   This module abstracts database operations for 'cfg_user_tbl' table.
 *
 * @author  hansaem, oh (praivesi@gmail.com)
 * @date    2024/09/13 22:04 created.
 * 
**/
use crate::api::entity::{NewUser, UpdateUser, User};
use crate::config::database::get_connection;
use crate::schema::cfg_user_tbl::{self, id};
use crate::schema::cfg_user_tbl::dsl::{cfg_user_tbl as all_users};

use diesel::expression_methods::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::sqlite::SqliteConnection;

use chrono::NaiveDate;

pub fn add_user(conn: &mut SqliteConnection, name: &str, death_age: i32, birth_date: NaiveDate) -> User {
    let new_user = NewUser {
        name: name.to_string(),
        death_age,
        birth_date,
        ctime: chrono::Utc::now().naive_utc().date(),
        mtime: chrono::Utc::now().naive_utc().date()
    };

    diesel::insert_into(cfg_user_tbl::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");

    all_users
        .order(id.desc())
        .first::<User>(conn)
        .expect("Error loading the last inserted user")
}

pub fn read_user(conn: &mut SqliteConnection, read_id: i32) -> Option<User> {
    all_users
        .filter(id.eq(read_id))
        .first::<User>(conn)
        .optional()
        .expect("Error reading user")
}

pub fn update_user(conn: &mut SqliteConnection, update_id: i32, update_name: &str, update_death_age: i32, update_birth_date: NaiveDate) -> Option<User> {
    let changeset = UpdateUser {
        name: update_name.to_string(),
        death_age: update_death_age,
        birth_date: update_birth_date,
        mtime: chrono::Utc::now().naive_utc().date()
    };

    let count = diesel::update(all_users.filter(id.eq(update_id)))
                            .set(&changeset)
                            .execute(conn)
                            .expect("Error updating user");

    if 1 == count {
        read_user(conn, update_id)
    }
    else {
        None
    }
}

pub fn delete_user(conn: &mut SqliteConnection, delete_id: i32) {
    diesel::delete(all_users.filter(id.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting user");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_tbl() {
        // arrange
        let conn = &mut *get_connection();

        let new_name = "my_name";
        let new_death_age = 10;
        let new_birth_date = NaiveDate::from_ymd(0, 1, 1);

        let update_name = "my_update_name";

        // act
        let inserted_user = add_user(conn, new_name, new_death_age, new_birth_date);

        let updated_user = update_user(conn, inserted_user.id, 
                                                                update_name,
                                             inserted_user.death_age,
                                            inserted_user.birth_date);

        delete_user(conn, inserted_user.id);

        let deleted_user = read_user(conn, inserted_user.id);
        
        // assert
        assert!(new_name == inserted_user.name);
        assert!(true == updated_user.is_some());
        assert!(update_name == updated_user.unwrap().name);
        assert!(true == deleted_user.is_none());
    }
}