-- Enable foreign key support in SQLite
PRAGMA foreign_keys = ON;

CREATE TABLE receipts (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    store TEXT NOT NULL,
    total REAL NOT NULL,
    datetime TIMESTAMP NOT NULL
);

CREATE TABLE receipt_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    receipt_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    price REAL NOT NULL,

    FOREIGN KEY (receipt_id)
        REFERENCES receipts(id)
        ON DELETE CASCADE
);

CREATE TABLE receipt_days (
    receipt_id INTEGER NOT NULL,
    day_id INTEGER NOT NULL,
    PRIMARY KEY (receipt_id, day_id),
    FOREIGN KEY (receipt_id) REFERENCES receipts(id) ON DELETE CASCADE,
    FOREIGN KEY (day_id) REFERENCES days(id) ON DELETE CASCADE
);
