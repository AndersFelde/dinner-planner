-- Your SQL goes here
PRAGMA foreign_keys=off;

CREATE TABLE meals_new (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    image TEXT NOT NULL,
    recipie_url TEXT
);

INSERT INTO meals_new (id, name, image, recipie_url)
SELECT id, name, image, recipie_url FROM meals WHERE image IS NOT NULL;

DROP TABLE meals;

ALTER TABLE meals_new RENAME TO meals;

PRAGMA foreign_keys=on;