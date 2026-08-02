use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum BlptCellType {
    Fail = 0,
    Pass = 1,
    Padding = 2,
    // day is within the blueprint's range but has no record yet (today or a future day)
    Future = 3
}