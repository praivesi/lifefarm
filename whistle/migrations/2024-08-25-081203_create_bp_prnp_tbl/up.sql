CREATE TABLE create_bp_prnp_tbl (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    life_id INTEGER NOT NULL,
    principle TEXT NOT NULL,
    desc TEXT,
    ctime DATE NOT NULL,
    mtime DATE NOT NULL
);