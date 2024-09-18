/**
 * @file    bpnt.rs
 * @brief   This module abstracts database operations for 'bpnt_tbl' table.
 *
 * @author  hansaem, oh (praivesi@gmail.com)
 * @date    2024/09/13 22:04 created.
 * 
**/
use crate::api::entity::{NewBluprint, UpdateBlueprint, Blueprint};
use crate::config::database::get_connection;
use crate::schema::bpnt_tbl::{self, id};
use crate::schema::bpnt_tbl::dsl::{bpnt_tbl as all_bpnts};

use diesel::expression_methods::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::sqlite::SqliteConnection;

use chrono::NaiveDate;

pub fn add_blueprint(conn: &mut SqliteConnection, goal: &str, exp_hour: i32, farm_portion: f32) -> Blueprint {
    let new_bpnt = NewBluprint {
        goal: goal.to_string(),
        exp_hour,
        farm_portion,
        ctime: chrono::Utc::now().naive_utc().date(),
        mtime: chrono::Utc::now().naive_utc().date()
    };

    diesel::insert_into(bpnt_tbl::table)
        .values(&new_bpnt)
        .execute(conn)
        .expect("Error saving new blueprint");

    all_bpnts
        .order(id.desc())
        .first::<Blueprint>(conn)
        .expect("Error loading the last inserted blueprint")
}

pub fn read_user(conn: &mut SqliteConnection, read_id: i32) -> Option<Blueprint> {
    all_bpnts
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
        mtime: chrono::Utc::now().naive_utc().date()
    };

    let count = diesel::update(all_bpnts.filter(id.eq(update_id)))
                            .set(&changeset)
                            .execute(conn)
                            .expect("Error updating blueprint");

    if 1 == count {
        read_user(conn, update_id)
    }
    else {
        None
    }
}

pub fn delete_blueprint(conn: &mut SqliteConnection, delete_id: i32) {
    diesel::delete(all_bpnts.filter(id.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting blueprint");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bpnt_tbl() {
        // arrange
        let conn = &mut *get_connection();

        let new_goal = "my_goal";
        let new_exp_hour = 10;
        let new_farm_portion = 0.1;

        let update_goal = "my_update_goal";

        // act
        let inserted_bpnt = add_blueprint(conn, new_goal, new_exp_hour, new_farm_portion);

        let updated_bpnt = update_blueprint(conn, 
                                            inserted_bpnt.id, 
                                            update_goal,
                                            inserted_bpnt.exp_hour,
                                            inserted_bpnt.farm_portion);

        delete_blueprint(conn, inserted_bpnt.id);

        let deleted_bpnt = read_user(conn, inserted_bpnt.id);
        
        // assert
        assert!(new_goal == inserted_bpnt.goal);
        assert!(true == updated_bpnt.is_some());
        assert!(update_goal == updated_bpnt.unwrap().goal);
        assert!(true == deleted_bpnt.is_none());
    }
}