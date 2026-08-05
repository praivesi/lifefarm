// @generated automatically by Diesel CLI.

diesel::table! {
    blpt_tbl (id) {
        id -> Integer,
        goal -> Text,
        desc -> Text,
        start_dt -> BigInt,
        end_dt -> BigInt,
        ctime -> BigInt,
        mtime -> BigInt,
        parent_id -> Nullable<Integer>,
    }
}

diesel::table! {
    ftpt_tbl (id) {
        id -> Integer,
        blpt_id -> Integer,
        day_dt -> BigInt,
        status -> Integer,
        note -> Nullable<Text>,
        photo -> Nullable<Binary>,
        ctime -> BigInt,
        mtime -> BigInt,
    }
}

diesel::table! {
    user_tbl (id) {
        id -> Integer,
        name -> Text,
        predict_death_age -> Integer,
        birth_date -> BigInt,
        ctime -> BigInt,
        mtime -> BigInt,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    blpt_tbl,
    ftpt_tbl,
    user_tbl,
);
