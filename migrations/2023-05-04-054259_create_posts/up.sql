-- Your SQL goes here
CREATE TABLE posts (
  id Integer PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE mtg_sets (
  code TEXT PRIMARY KEY
);

CREATE TABLE mtg_formats (
 format_name TEXT PRIMARY KEY
);

CREATE TABLE card (
    card_name TEXT PRIMARY KEY,
    amount Integer,
    card_set TEXT NOT NULL,
    card_formats TEXT NOT NULL,
    FOREIGN KEY (card_set) REFERENCES mtg_sets (code),
    FOREIGN KEY (card_formats) REFERENCES mtg_formats (format_name)
);