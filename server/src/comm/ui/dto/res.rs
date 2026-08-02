use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::entity::Blueprint;
use crate::enums::BlptCellType;

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
pub struct GetUserResponse {
    pub id: i32,
    pub name: String,
    pub predict_death_age: i32,
    pub birth_date: i64
}

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
pub struct GetBlptListResponse {
    pub blpts: Vec<Blueprint>
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct GetBlptCellListResponse {
    pub blpt: Blueprint,
    pub cell_start_dt: i64,
    pub cell_end_dt: i64,
    pub cells: Vec<GetBlptCellResponse>
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct GetBlptCellResponse {
    pub date: i64,
    pub status: BlptCellType,
    pub is_today: bool,
    pub note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
pub struct GetFtptListResponse {
    pub ftpts: Vec<GetFtptRepsponse>
}

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
pub struct GetFtptRepsponse {
    pub id: i32,
    pub blpt_id: i32,
    pub day_dt: i64,
    pub status: i32,
    pub note: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
pub struct GetLifeFarmResponse {
    pub birth_date: i64,
    pub predict_death_age: i32,
    pub cells: Vec<LifeFarmCell>
}

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
pub struct LifeFarmCell {
    pub day_dt: i64,
    pub target_rate: f32,
    pub actual_rate: f32
}