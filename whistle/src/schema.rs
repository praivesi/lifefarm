// @generated automatically by Diesel CLI.

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
