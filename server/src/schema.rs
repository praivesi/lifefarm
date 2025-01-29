// @generated automatically by Diesel CLI.

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

diesel::allow_tables_to_appear_in_same_query!(
    blpt_tbl,
    ftpt_tbl,
    user_tbl,
);
