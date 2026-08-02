CREATE TABLE ftpt_tbl_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    blpt_id INTEGER NOT NULL,
    day_dt BIGINT NOT NULL,
    status INTEGER NOT NULL DEFAULT 1,
    note TEXT,
    photo BLOB,
    ctime BIGINT NOT NULL,
    mtime BIGINT NOT NULL
);

INSERT INTO ftpt_tbl_new (id, blpt_id, day_dt, status, note, photo, ctime, mtime)
    SELECT id, blpt_id, ctime, 1, NULL, cert, ctime, mtime FROM ftpt_tbl;

DROP TABLE ftpt_tbl;

ALTER TABLE ftpt_tbl_new RENAME TO ftpt_tbl;

CREATE UNIQUE INDEX idx_ftpt_blpt_day ON ftpt_tbl (blpt_id, day_dt);
