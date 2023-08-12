-- Add migration script here
-- CREATE TABLE IF NOT EXISTS cardbase
-- (
--     name TEXT NOT NULL PRIMARY KEY,
--     set_code NOT NULL,
--     price TEXT
-- );

-- CREATE TABLE IF NOT EXISTS cardcol
-- (
--     name TEXT NOT NULL PRIMARY KEY,
--     set_code NOT NULL,
--     price TEXT,
--     date BLOB,
--     FOREIGN KEY (name, set_code, price) REFERENCES cardbase(name, set_code, price)
-- );

CREATE TABLE IF NOT EXISTS carduser
(
    name TEXT NOT NULL PRIMARY KEY,
    set_code NOT NULL,
    price TEXT,
    amount INTEGER
    -- FOREIGN KEY (name, set_code, price) REFERENCES cardbase(name, set_code, price)
);