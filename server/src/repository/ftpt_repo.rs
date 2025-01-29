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
use crate::schema::ftpt_tbl::{self, id};
use crate::schema::ftpt_tbl::dsl::{ftpt_tbl as all_ftpts};

use diesel::expression_methods::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::sqlite::SqliteConnection;

use chrono::NaiveDate;

pub fn add_footprint(conn: &mut SqliteConnection, blpt_id: i32, cert: Option<Vec<u8>>) -> Footprint {
    let new_ftpt = NewFootprint {
        blpt_id,
        cert,
        ctime: chrono::Utc::now().naive_utc().date(),
        mtime: chrono::Utc::now().naive_utc().date()
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

pub fn read_ftpt(conn: &mut SqliteConnection, read_id: i32) -> Option<Footprint> {
    all_ftpts
        .filter(id.eq(read_id))
        .first::<Footprint>(conn)
        .optional()
        .expect("Error reading footprint")
}

pub fn update_footprint(conn: &mut SqliteConnection, update_id: i32,  blpt_id: i32, cert: Option<Vec<u8>>) -> Option<Footprint> {
    let changeset = UpdateFootprint {
        id: update_id,
        blpt_id,
        cert,
        mtime: chrono::Utc::now().naive_utc().date()
    };

    let count = diesel::update(all_ftpts.filter(id.eq(update_id)))
                            .set(&changeset)
                            .execute(conn)
                            .expect("Error updating footprint");

    if 1 == count {
        read_ftpt(conn, update_id)
    }
    else {
        None
    }
}

pub fn delete_footprint(conn: &mut SqliteConnection, delete_id: i32) -> usize {
    diesel::delete(all_ftpts.filter(id.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting footprint")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ftpt_tbl() {
        // arrange
        let conn = &mut *get_connection();

        let new_blpt_id = 10;
        let new_cert = Some(vec![1,2,3,4,5]);

        let updated_blpt_id = 20;
        let updated_cert = Some(vec![6,7,8,9,10]);

        // act
        let inserted_ftpt = add_footprint(conn, new_blpt_id, new_cert);

        let updated_ftpt = update_footprint(conn, 
                                            inserted_ftpt.id,
                                            updated_blpt_id,
                                            updated_cert);

        let del_count = delete_footprint(conn, inserted_ftpt.id);

        // assert
        assert!(new_blpt_id == inserted_ftpt.blpt_id);
        assert!(true == updated_ftpt.is_some());
        assert!(updated_blpt_id == updated_ftpt.unwrap().blpt_id);
        assert!(1 == del_count);
    }
}