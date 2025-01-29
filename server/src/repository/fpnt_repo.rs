/**
 * @file    fpnt_repo.rs
 * @brief   This module abstracts database operations for 'fpnt_tbl' table.
 *
 * @author  hansaem, oh (praivesi@gmail.com)
 * @date    2025/01/12 15:28 created.
 * 
**/
use crate::entity::{NewFootprint, UpdateFootprint, Footprint};
use crate::config::database::get_connection;
use crate::schema::fpnt_tbl::{self, id};
use crate::schema::fpnt_tbl::dsl::{fpnt_tbl as all_fpnts};

use diesel::expression_methods::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::sqlite::SqliteConnection;

use chrono::NaiveDate;

pub fn add_footprint(conn: &mut SqliteConnection, bpnt_id: i32, cert: Option<Vec<u8>>) -> Footprint {
    let new_fpnt = NewFootprint {
        bpnt_id,
        cert,
        ctime: chrono::Utc::now().naive_utc().date(),
        mtime: chrono::Utc::now().naive_utc().date()
    };

    diesel::insert_into(fpnt_tbl::table)
        .values(&new_fpnt)
        .execute(conn)
        .expect("Error saving new footprint");

    all_fpnts 
        .order(id.desc())
        .first::<Footprint>(conn)
        .expect("Error loading the last inserted footprint")
}

pub fn read_fpnt(conn: &mut SqliteConnection, read_id: i32) -> Option<Footprint> {
    all_fpnts
        .filter(id.eq(read_id))
        .first::<Footprint>(conn)
        .optional()
        .expect("Error reading footprint")
}

pub fn update_footprint(conn: &mut SqliteConnection, update_id: i32,  bpnt_id: i32, cert: Option<Vec<u8>>) -> Option<Footprint> {
    let changeset = UpdateFootprint {
        id: update_id,
        bpnt_id,
        cert,
        mtime: chrono::Utc::now().naive_utc().date()
    };

    let count = diesel::update(all_fpnts.filter(id.eq(update_id)))
                            .set(&changeset)
                            .execute(conn)
                            .expect("Error updating footprint");

    if 1 == count {
        read_fpnt(conn, update_id)
    }
    else {
        None
    }
}

pub fn delete_footprint(conn: &mut SqliteConnection, delete_id: i32) -> usize {
    diesel::delete(all_fpnts.filter(id.eq(delete_id)))
        .execute(conn)
        .expect("Error deleting footprint")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fpnt_tbl() {
        // arrange
        let conn = &mut *get_connection();

        let new_bpnt_id = 10;
        let new_cert = Some(vec![1,2,3,4,5]);

        let updated_bpnt_id = 20;
        let updated_cert = Some(vec![6,7,8,9,10]);

        // act
        let inserted_fpnt = add_footprint(conn, new_bpnt_id, new_cert);

        let updated_fpnt = update_footprint(conn, 
                                            inserted_fpnt.id,
                                            updated_bpnt_id,
                                            updated_cert);

        let del_count = delete_footprint(conn, inserted_fpnt.id);

        // assert
        assert!(new_bpnt_id == inserted_fpnt.bpnt_id);
        assert!(true == updated_fpnt.is_some());
        assert!(updated_bpnt_id == updated_fpnt.unwrap().bpnt_id);
        assert!(1 == del_count);
    }
}