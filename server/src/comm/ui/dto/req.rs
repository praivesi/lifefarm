use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
pub struct PostUserRequest {
    pub name: String,
    pub predict_death_age: i32,
    pub birth_date: i64
}

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
pub struct PostBlptRequest {
    pub goal: String,
    pub desc: String,
    pub start_dt: i64,
    pub end_dt: i64
}

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
pub struct PostFtptRequest {
    pub day_dt: i64,
    pub status: i32,
    pub note: Option<String>
}