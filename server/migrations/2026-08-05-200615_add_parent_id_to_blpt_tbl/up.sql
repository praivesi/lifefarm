-- Your SQL goes here
ALTER TABLE blpt_tbl ADD COLUMN parent_id INTEGER REFERENCES blpt_tbl(id);
