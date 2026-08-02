/**
 * @file    entity.rs
 * @brief   This module defines database entities.
 *
 * @author  hansaem, oh (praivesi@gmail.com)
 * @date    2024/09/13 22:04 created.
 * 
**/
use crate::schema::{user_tbl, blpt_tbl, ftpt_tbl};

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Selectable, QueryableByName, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_tbl)]
pub struct User {
    pub id: i32, // Key
    pub name: String,
    pub predict_death_age: i32,
    pub birth_date: i64,
    pub ctime: i64,
    pub mtime: i64
}

#[derive(AsChangeset)]
#[diesel(table_name = user_tbl)]
pub struct UpdateUser {
    pub name: String,
    pub predict_death_age: i32,
    pub birth_date: i64,
    pub mtime: i64
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 0,
            name: "".to_string(),
            predict_death_age: 0,
            birth_date: 0,
            ctime: 0,
            mtime: 0,
        }
    }
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_tbl)]
pub struct NewUser {
    pub name: String,
    pub predict_death_age: i32,
    pub birth_date: i64,
    pub ctime: i64,
    pub mtime: i64 
}

#[derive(Queryable, Selectable, QueryableByName, Serialize, Deserialize, Debug, ToSchema, Clone)]
#[diesel(table_name = blpt_tbl)]
pub struct Blueprint {
    pub id: i32, // Key
    pub goal: String,
    pub desc: String,
    pub start_dt: i64,
    pub end_dt: i64,
    #[serde(skip)] pub ctime: i64,
    #[serde(skip)] pub mtime: i64
}


#[derive(AsChangeset)]
#[diesel(table_name = blpt_tbl)]
pub struct UpdateBlueprint {
    pub goal: String,
    pub desc: String,
    pub start_dt: i64,
    pub end_dt: i64,
    pub mtime: i64
}

impl Default for Blueprint {
    fn default() -> Self {
        Blueprint {
            id: 0,
            goal: "".to_string(),
            desc: "".to_string(),
            start_dt: 0,
            end_dt: 0,
            ctime: 0,
            mtime: 0,
        }
    }
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = blpt_tbl)]
pub struct NewBluprint {
    pub goal: String,
    pub desc: String,
    pub start_dt: i64,
    pub end_dt: i64,
    pub ctime: i64,
    pub mtime: i64 
}

#[derive(Queryable, Selectable, QueryableByName, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = ftpt_tbl)]
pub struct Footprint {
    pub id: i32, // Key
    pub blpt_id: i32,
    pub day_dt: i64,
    pub status: i32,
    pub note: Option<String>,
    pub photo: Option<Vec<u8>>,
    pub ctime: i64,
    pub mtime: i64
}

#[derive(AsChangeset)]
#[diesel(table_name = ftpt_tbl)]
pub struct UpdateFootprint {
    pub status: i32,
    pub note: Option<String>,
    pub photo: Option<Vec<u8>>,
    pub mtime: i64
}

impl Default for Footprint {
    fn default() -> Self {
        Footprint {
            id: 0,
            blpt_id: 0,
            day_dt: 0,
            status: 0,
            note: None,
            photo: None,
            ctime: 0,
            mtime: 0,
        }
    }
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = ftpt_tbl)]
pub struct NewFootprint {
    pub blpt_id: i32,
    pub day_dt: i64,
    pub status: i32,
    pub note: Option<String>,
    pub photo: Option<Vec<u8>>,
    pub ctime: i64,
    pub mtime: i64
}