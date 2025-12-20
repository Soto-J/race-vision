-- Add down migration script here
CREATE TABLE
    IF NOT EXISTS webview_layout (
        name TEXT PRIMARY KEY,
        x REAL NOT NULL DEFAULT 0,
        y REAL NOT NULL DEFAULT 0,
        width REAL NOT NULL DEFAULT 200,
        height REAL NOT NULL DEFAULT 200
    );