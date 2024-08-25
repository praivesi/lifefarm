// @generated automatically by Diesel CLI.

diesel::table! {
    cfg_user_tbl (id) {
        id -> Nullable<Integer>,
        name -> Text,
        death_age -> Integer,
        birth_date -> Date,
        ctime -> Date,
        mtime -> Date,
    }
}

diesel::table! {
    create_bp_period_tbl (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        parent_period_id -> Integer,
        name -> Text,
        desc -> Nullable<Text>,
        start_date -> Date,
        max_use_hour -> Integer,
        d_strategy -> Text,
        h_strategy -> Integer,
        used_day -> Integer,
        used_hour -> Integer,
        ctime -> Date,
        mtime -> Date,
    }
}

diesel::table! {
    create_bp_prnp_tbl (id) {
        id -> Nullable<Integer>,
        life_id -> Integer,
        principle -> Text,
        desc -> Nullable<Text>,
        ctime -> Date,
        mtime -> Date,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cfg_user_tbl,
    create_bp_period_tbl,
    create_bp_prnp_tbl,
);
