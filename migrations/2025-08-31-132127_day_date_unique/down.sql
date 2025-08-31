-- This file should undo anything in `up.sql`
PRAGMA foreign_keys=off;

CREATE TABLE days_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date DATE NOT NULL,
    meal_id INTEGER,
    week INTEGER NOT NULL,
    year INTEGER NOT NULL
);

INSERT INTO days_new (id, date, meal_id, week, year)
SELECT id, date, meal_id, week, year FROM days;

DROP TABLE days;

ALTER TABLE days_new RENAME TO days;

PRAGMA foreign_keys=on;