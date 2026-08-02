CREATE TABLE ftpt_tbl_old (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    blpt_id INTEGER NOT NULL,
    cert BLOB,
    ctime BIGINT NOT NULL,
    mtime BIGINT NOT NULL
);

INSERT INTO ftpt_tbl_old (id, blpt_id, cert, ctime, mtime)
    SELECT id, blpt_id, photo, ctime, mtime FROM ftpt_tbl;

DROP INDEX IF EXISTS idx_ftpt_blpt_day;

DROP TABLE ftpt_tbl;

ALTER TABLE ftpt_tbl_old RENAME TO ftpt_tbl;
