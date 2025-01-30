use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::entity::Blueprint;

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

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
pub struct GetFtptListResponse {
    pub ftpts: Vec<GetFtptRepsponse>
}

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
pub struct GetFtptRepsponse {
    pub id: i32,
    pub blpt_id: i32,
    pub cert: Option<String>
}