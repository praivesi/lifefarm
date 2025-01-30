CREATE TABLE user_tbl (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name varchar(255) NOT NULL,
    predict_death_age INTEGER NOT NULL,
    birth_date BIGINT NOT NULL,
    ctime BIGINT NOT NULL,
    mtime BIGINT NOT NULL
);
