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
    fpnt_tbl (id) {
        id -> Integer,
        bpnt_id -> Integer,
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

diesel::allow_tables_to_appear_in_same_query!(
    bpnt_tbl,
    fpnt_tbl,
    user_tbl,
);
