use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/*

diesel::table! {
    blpt_tbl (id) {
        id -> Integer,
        goal -> Text,
        exp_hour -> Integer,
        farm_portion -> Float,
        ctime -> Date,
        mtime -> Date,
    }
}

diesel::table! {
    ftpt_tbl (id) {
        id -> Integer,
        blpt_id -> Integer,
        cert -> Nullable<Binary>,
        ctime -> Date,
        mtime -> Date,
    }
}

diesel::table! {
    user_tbl (id) {
        id -> Integer,
        name -> Text,
        predict_death_age -> Integer,
        birth_date -> Date,
        ctime -> Date,
        mtime -> Date,
    }
}
*/

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
pub struct GetUserResponse {
    pub id: i32,
    pub name: String,
    pub predict_death_age: i32,
    pub birth_date: i64
}

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
pub struct GetBlptListResponse {
    pub blpts: Vec<GetBlptResponse>
}

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
pub struct GetBlptResponse {
    pub id: i32,
    pub goal: String,
    pub exp_hour: i32,
    pub farm_portion: f32
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