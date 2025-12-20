-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS webview_layout (
        name TEXT PRIMARY KEY,
        x REAL NOT NULL DEFAULT 0,
        y REAL NOT NULL DEFAULT 0,
        width REAL NOT NULL DEFAULT 200,
        height REAL NOT NULL DEFAULT 200
    );

INSERT INTO
    webview_layout (name, x, y, width, height)
VALUES
    ('inputs', 50.0, 50.0, 400.0, 150.0),
    ('standings', 200.0, 200.0, 400.0, 600.0),
    ('track-map', 600.0, 200.0, 300.0, 300.0),
    ('relative', 100.0, 500.0, 400.0, 200.0) ON CONFLICT (name) DO NOTHING;