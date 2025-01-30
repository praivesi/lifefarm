use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub enum BlptCellType {
    Fail = 0,
    Pass = 1,
    Padding = 2
}