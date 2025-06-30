PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS request (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_email TEXT NOT NULL,
    url TEXT NOT NULL,
    method TEXT NOT NULL,
    metadata TEXT,
    payload TEXT,
    status TEXT NOT NULL,
    service TEXT,
    proto_file TEXT,
    date TEXT NOT NULL,
    hidden INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS "user" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
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
    url TEXT NOT NULL,
    method TEXT NOT NULL,
    origin TEXT,
    headers TEXT,
    body TEXT,
    status TEXT NOT NULL,
    date TEXT NOT NULL,
    hidden INTEGER NOT NULL,
    FOREIGN KEY (user_email) REFERENCES "user"(email)
);

INSERT INTO "user" (username, email, password, favorites, date, deleted)
VALUES (
    'anon',
    'anon@a.b',
    'anon',
    NULL,
    strftime('%s', 'now'),
    0
);
