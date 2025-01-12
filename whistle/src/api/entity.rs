/**
 * @file    entity.rs
 * @brief   This module defines database entities.
 *
 * @author  hansaem, oh (praivesi@gmail.com)
 * @date    2024/09/13 22:04 created.
 * 
**/
use chrono::NaiveDate;
use crate::schema::{user_tbl, bpnt_tbl, fpnt_tbl};

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, QueryableByName, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_tbl)]
pub struct User {
    pub id: i32, // Key
    pub name: String,
    pub predict_death_age: i32,
    pub birth_date: NaiveDate,
    pub ctime: NaiveDate,
    pub mtime: NaiveDate
}

#[derive(AsChangeset)]
#[diesel(table_name = user_tbl)]
pub struct UpdateUser {
    pub name: String,
    pub predict_death_age: i32,
    pub birth_date: NaiveDate,
    pub mtime: NaiveDate
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 0,
            name: "".to_string(),
            predict_death_age: 0,
            birth_date: NaiveDate::from_ymd(0, 1, 1),
            ctime: NaiveDate::from_ymd(0, 1, 1),
            mtime: NaiveDate::from_ymd(0, 1, 1),
        }
    }
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_tbl)]
pub struct NewUser {
    pub name: String,
    pub predict_death_age: i32,
    pub birth_date: NaiveDate,
    pub ctime: NaiveDate,
    pub mtime: NaiveDate 
}

#[derive(Queryable, Selectable, QueryableByName, Serialize, Deserialize, Debug)]
#[diesel(table_name = bpnt_tbl)]
pub struct Blueprint {
    pub id: i32, // Key
    pub goal: String,
    pub exp_hour: i32,
    pub farm_portion: f32,
    pub ctime: NaiveDate,
    pub mtime: NaiveDate
}


#[derive(AsChangeset)]
#[diesel(table_name = bpnt_tbl)]
pub struct UpdateBlueprint {
    pub goal: String,
    pub exp_hour: i32,
    pub farm_portion: f32,
    pub mtime: NaiveDate
}

impl Default for Blueprint {
    fn default() -> Self {
        Blueprint {
            id: 0,
            goal: "".to_string(),
            exp_hour: 0,
            farm_portion: 0.0,
            ctime: NaiveDate::from_ymd(0, 1, 1),
            mtime: NaiveDate::from_ymd(0, 1, 1),
        }
    }
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = bpnt_tbl)]
pub struct NewBluprint {
    pub goal: String,
    pub exp_hour: i32,
    pub farm_portion: f32,
    pub ctime: NaiveDate,
    pub mtime: NaiveDate 
}

#[derive(Queryable, Selectable, QueryableByName, Serialize, Deserialize, Debug)]
#[diesel(table_name = fpnt_tbl)]
pub struct Footprint {
    pub id: i32, // Key
    pub bpnt_id: i32,
    pub cert: Option<Vec<u8>>,
    pub ctime: NaiveDate,
    pub mtime: NaiveDate
}

#[derive(AsChangeset)]
#[diesel(table_name = fpnt_tbl)]
pub struct UpdateFootprint {
    pub id: i32, // Key
    pub bpnt_id: i32,
    pub cert: Option<Vec<u8>>,
    pub mtime: NaiveDate
}

impl Default for Footprint {
    fn default() -> Self {
        Footprint {
            id: 0,
            bpnt_id: 0,
            cert: None,
            ctime: NaiveDate::from_ymd(0, 1, 1),
            mtime: NaiveDate::from_ymd(0, 1, 1),
        }
    }
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = fpnt_tbl)]
pub struct NewFootprint {
    pub bpnt_id: i32,
    pub cert: Option<Vec<u8>>,
    pub ctime: NaiveDate,
    pub mtime: NaiveDate 
}