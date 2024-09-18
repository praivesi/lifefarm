CREATE TABLE bpnt_tbl (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    goal varchar(255) NOT NULL,
    exp_hour INTEGER NOT NULL,
    farm_portion REAL NOT NULL,
    ctime DATE NOT NULL,
    mtime DATE NOT NULL
);
