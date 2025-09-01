-- This file should undo anything in `up.sql`
PRAGMA foreign_keys=off;

CREATE TABLE meals_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    image TEXT,
    recipie_url TEXT
);

INSERT INTO meals_new (id, name, image, recipie_url)
SELECT id, name, image, recipie_url FROM meals;

DROP TABLE meals;

ALTER TABLE meals_new RENAME TO meals;

PRAGMA foreign_keys=on;