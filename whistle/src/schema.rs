// @generated automatically by Diesel CLI.

diesel::table! {
    bpnt_tbl (id) {
        id -> Integer,
        goal -> Text,
        exp_hour -> Integer,
        farm_portion -> Float,
        ctime -> Date,
        mtime -> Date,
    }
}

diesel::table! {
    cfg_user_tbl (id) {
        id -> Integer,
        name -> Text,
        death_age -> Integer,
        birth_date -> Date,
        ctime -> Date,
        mtime -> Date,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bpnt_tbl,
    cfg_user_tbl,
);
