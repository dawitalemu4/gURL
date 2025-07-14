PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS request (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_email TEXT NOT NULL,
    command TEXT NOT NULL,
    status TEXT NOT NULL,
    method TEXT NOT NULL,
    date TEXT NOT NULL,
    hidden INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS "user" (
    username TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    favorites TEXT,
    date TEXT NOT NULL,
    deleted INTEGER NOT NULL
);

DROP TABLE IF EXISTS request;

CREATE TABLE IF NOT EXISTS request (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_email TEXT NOT NULL,
    command TEXT NOT NULL,
    status TEXT NOT NULL,
    method TEXT NOT NULL,
    date TEXT NOT NULL,
    hidden INTEGER NOT NULL,
    FOREIGN KEY (user_email) REFERENCES "user"(email)
);

INSERT INTO "user" (username, email, password, favorites, date, deleted)
VALUES (
    'anon',
    'anon',
    'anon',
    NULL,
    strftime('%s', 'now'),
    0
);
