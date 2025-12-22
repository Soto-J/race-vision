# RaceVision Database Documentation

## Database Location

- **Path**: `desktop-app/src-tauri/db/app.db`
- **Type**: SQLite 3
- **Migrations**: Managed by SQLx

## Quick Access

```bash
# View database in CLI
cd desktop-app/src-tauri/db
sqlite3 app.db

# Common queries
sqlite3 app.db "SELECT * FROM app_settings;"
sqlite3 app.db "SELECT * FROM webview_layout;"
sqlite3 app.db "SELECT * FROM page_settings;"
sqlite3 app.db ".tables"
sqlite3 app.db ".schema table_name"
```

---

## Database Schema

### 1. `webview_layout`

Stores position and size for each overlay window.

| Column   | Type | Constraints           | Description                                                     |
| -------- | ---- | --------------------- | --------------------------------------------------------------- |
| `name`   | TEXT | PRIMARY KEY           | Unique identifier for the overlay (e.g., 'inputs', 'standings') |
| `x`      | REAL | NOT NULL, DEFAULT 0   | X coordinate of window position                                 |
| `y`      | REAL | NOT NULL, DEFAULT 0   | Y coordinate of window position                                 |
| `width`  | REAL | NOT NULL, DEFAULT 200 | Window width in pixels                                          |
| `height` | REAL | NOT NULL, DEFAULT 200 | Window height in pixels                                         |

**Example Data**:

```sql
SELECT * FROM webview_layout;
-- inputs|50.0|50.0|400.0|150.0
-- standings|200.0|200.0|400.0|600.0
-- track-map|600.0|200.0|300.0|300.0
-- relative|100.0|500.0|400.0|200.0
```

---

### 2. `app_settings`

Global application settings (single row table).

| Column                         | Type    | Constraints                  | Description                         |
| ------------------------------ | ------- | ---------------------------- | ----------------------------------- |
| `id`                           | INTEGER | PRIMARY KEY, CHECK (id = 1)  | Always 1 (ensures single row)       |
| `use_metric_system`            | INTEGER | NOT NULL, DEFAULT 0, BOOLEAN | 0 = Imperial, 1 = Metric            |
| `startup_overlay_minimized`    | INTEGER | NOT NULL, DEFAULT 0, BOOLEAN | Start overlays minimized            |
| `minimize_to_system_tray`      | INTEGER | NOT NULL, DEFAULT 0, BOOLEAN | Minimize to tray instead of taskbar |
| `show_race_control_at_startup` | INTEGER | NOT NULL, DEFAULT 0, BOOLEAN | Auto-show race control window       |
| `use_ctrl_f6_instead_of_f6`    | INTEGER | NOT NULL, DEFAULT 0, BOOLEAN | Alternative hotkey for F6           |
| `use_hardware_acceleration`    | INTEGER | NOT NULL, DEFAULT 0, BOOLEAN | Enable GPU acceleration             |
| `lower_fps`                    | INTEGER | NOT NULL, DEFAULT 0, BOOLEAN | Reduce frame rate for performance   |
| `show_overlays_in_taskbar`     | INTEGER | NOT NULL, DEFAULT 0, BOOLEAN | Show overlay windows in taskbar     |
| `vr_compatibility`             | INTEGER | NOT NULL, DEFAULT 0, BOOLEAN | Enable VR compatibility mode        |

**Note**: All values are boolean (0 or 1). Only one row exists (id must equal 1).

**Example Query**:

```sql
SELECT * FROM app_settings;
-- 1|0|0|0|0|0|0|0|0|0
```

---

### 3. `page_settings`

Individual settings for each page/overlay.

| Column    | Type    | Constraints                  | Description                                   |
| --------- | ------- | ---------------------------- | --------------------------------------------- |
| `page`    | TEXT    | PRIMARY KEY (composite)      | Page identifier (e.g., 'inputs', 'standings') |
| `setting` | TEXT    | PRIMARY KEY (composite)      | Setting name (e.g., 'is_active', 'show_flag') |
| `value`   | INTEGER | NOT NULL, DEFAULT 0, BOOLEAN | Setting value (0 or 1)                        |

**Composite Primary Key**: `(page, setting)` - Ensures unique setting per page.

**Example Pages**:

- `inputs` - Input display overlay
- `standings` - Race standings overlay
- `track-map` - Track map overlay
- `relative` - Relative timing overlay
- `damage` - Damage indicator overlay
- `fuel` - Fuel calculator overlay
- `weather` - Weather information overlay

**Example Settings**:

- `is_active` - Whether the page is enabled
- `show_flag` - Display flag status
- `show_position` - Display position number
- `show_intervals` - Display time intervals
- etc.

**Example Query**:

```sql
SELECT * FROM page_settings WHERE page = 'inputs';
-- inputs|is_active|0
-- inputs|show_throttle|1
-- inputs|show_brake|1
```

---

### 4. `page_setting_display`

Controls visibility of specific settings per session type.

| Column       | Type    | Constraints                    | Description                                  |
| ------------ | ------- | ------------------------------ | -------------------------------------------- |
| `page`       | TEXT    | PRIMARY KEY (composite)        | Page identifier                              |
| `setting`    | TEXT    | PRIMARY KEY (composite)        | Setting name                                 |
| `session`    | TEXT    | PRIMARY KEY (composite), CHECK | Session type: 'practice', 'qualy', or 'race' |
| `is_visible` | INTEGER | NOT NULL, DEFAULT 1, BOOLEAN   | Whether setting is visible in this session   |

**Composite Primary Key**: `(page, setting, session)`

**Foreign Key**: References `page_settings(page, setting)` with CASCADE delete

**Session Types**:

- `practice` - Practice session
- `qualy` - Qualifying session
- `race` - Race session

**Example Query**:

```sql
SELECT * FROM page_setting_display WHERE page = 'standings';
-- standings|show_position|practice|1
-- standings|show_position|qualy|1
-- standings|show_position|race|0
```

---

### 5. `_sqlx_migrations`

SQLx internal migration tracking (managed automatically).

| Column           | Type      | Constraints                         | Description                 |
| ---------------- | --------- | ----------------------------------- | --------------------------- |
| `version`        | BIGINT    | PRIMARY KEY                         | Migration version timestamp |
| `description`    | TEXT      | NOT NULL                            | Migration description       |
| `installed_on`   | TIMESTAMP | NOT NULL, DEFAULT CURRENT_TIMESTAMP | When migration was applied  |
| `success`        | BOOLEAN   | NOT NULL                            | Whether migration succeeded |
| `checksum`       | BLOB      | NOT NULL                            | Migration file checksum     |
| `execution_time` | BIGINT    | NOT NULL                            | Time taken to execute (ms)  |

**Note**: Do not modify this table manually. It's managed by SQLx migrations.

---

## Relationships

```
page_settings
    ↓ (FOREIGN KEY CASCADE)
page_setting_display
```

When a row is deleted from `page_settings`, all related rows in `page_setting_display` are automatically deleted.

---

## Common Operations

### View all settings for a specific page

```sql
SELECT * FROM page_settings WHERE page = 'inputs';
```

### Get overlay positions

```sql
SELECT name, x, y, width, height FROM webview_layout ORDER BY name;
```

### Check which settings are visible in race sessions

```sql
SELECT page, setting
FROM page_setting_display
WHERE session = 'race' AND is_visible = 1;
```

### Update app setting

```sql
UPDATE app_settings SET use_metric_system = 1 WHERE id = 1;
```

### Add new page setting

```sql
INSERT INTO page_settings (page, setting, value)
VALUES ('standings', 'show_lap_times', 1);
```

### Update overlay position

```sql
UPDATE webview_layout
SET x = 100, y = 100, width = 500, height = 300
WHERE name = 'inputs';
```

---

## Migration Files

Location: `desktop-app/src-tauri/migrations/`

- `20251220214634_initial_schema.up.sql` - Initial database structure
- Additional migrations as numbered SQL files

To create a new migration:

```bash
sqlx migrate add migration_description
```

To apply migrations:

```bash
sqlx migrate run --database-url sqlite:./db/app.db
```

---

## Backup & Restore

### Backup database

```bash
sqlite3 app.db ".backup backup.db"
# or
cp app.db app_backup_$(date +%Y%m%d).db
```

### Export to SQL

```bash
sqlite3 app.db .dump > backup.sql
```

### Restore from SQL

```bash
sqlite3 new_app.db < backup.sql
```

---

## Troubleshooting

### Database is locked

- Close all connections to the database
- Check if the application is running
- Delete `app.db-wal` and `app.db-shm` files (Write-Ahead Log files)

### Reset database

```bash
# Delete database and reapply migrations
rm app.db
sqlx migrate run --database-url sqlite:./db/app.db
```

### Check database integrity

```bash
sqlite3 app.db "PRAGMA integrity_check;"
```

### View all constraints

```bash
sqlite3 app.db "SELECT sql FROM sqlite_master WHERE type='table';"
```
