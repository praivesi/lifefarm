/**
 * @file    entity.rs
 * @brief   This module defines database entities.
 *
 * @author  hansaem, oh (praivesi@gmail.com)
 * @date    2024/09/13 22:04 created.
 * 
**/
use chrono::NaiveDate;
use crate::schema::cfg_user_tbl;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, QueryableByName, Serialize, Deserialize, Debug)]
#[diesel(table_name = cfg_user_tbl)]
pub struct User {
    pub id: i32, // Key
    pub name: String,
    pub death_age: i32,
    pub birth_date: NaiveDate,
    pub ctime: NaiveDate,
    pub mtime: NaiveDate
}

#[derive(AsChangeset)]
#[diesel(table_name = cfg_user_tbl)]
pub struct UpdateUser {
    pub name: String,
    pub death_age: i32,
    pub birth_date: NaiveDate,
    pub mtime: NaiveDate
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 0,
            name: "".to_string(),
            death_age: 0,
            birth_date: NaiveDate::from_ymd(0, 1, 1),
            ctime: NaiveDate::from_ymd(0, 1, 1),
            mtime: NaiveDate::from_ymd(0, 1, 1),
        }
    }
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = cfg_user_tbl)]
pub struct NewUser {
    pub name: String,
    pub death_age: i32,
    pub birth_date: NaiveDate,
    pub ctime: NaiveDate,
    pub mtime: NaiveDate 
}