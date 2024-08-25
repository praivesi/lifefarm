CREATE TABLE create_bp_period_tbl (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    parent_period_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    desc TEXT,
    start_date DATE NOT NULL,
    max_use_hour INTEGER NOT NULL,
    d_strategy VARCHAR(128) NOT NULL,
    h_strategy INTEGER NOT NULL,
    used_day INTEGER NOT NULL,
    used_hour INTEGER NOT NULL,
    ctime DATE NOT NULL,
    mtime DATE NOT NULL
);