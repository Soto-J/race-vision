use crate::domain::{Database, PageSettings};

use std::collections::HashMap;
use tauri::State;

#[tauri::command]
pub async fn get_page_settings(
    page: &str,
    db: State<'_, Database>,
) -> Result<HashMap<String, bool>, String> {
    let rows = sqlx::query_as!(
        PageSettings,
        "
        SELECT 
            page, setting, value != 0 AS 'value!: bool'
        FROM 
            page_settings
        WHERE
            page = ?    
        ",
        page
    )
    .fetch_all(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut map: HashMap<String, bool> = HashMap::new();

    for row in rows {
        map.entry(row.setting).or_insert(row.value);
    }

    Ok(map)
}

#[tauri::command]
pub async fn update_setting() -> Result<(), String> {
    Ok(())
}
