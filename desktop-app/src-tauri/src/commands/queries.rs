use std::collections::HashMap;

use crate::domain::{Database, PageSettings};

use tauri::State;
use tokio::sync::RwLock;

#[tauri::command]
pub async fn set_watched_vars(
    vars: Vec<String>,
    watched_vars: State<'_, RwLock<Vec<String>>>,
) -> Result<(), String> {
    *watched_vars.write().await = vars;

    Ok(())
}

#[tauri::command]
pub async fn get_all_page_settings(
    db: State<'_, Database>,
) -> Result<HashMap<String, HashMap<String, bool>>, String> {
    let rows = sqlx::query_as!(
        PageSettings,
        "
        SELECT 
            page, setting, value != 0 AS 'value!: bool' 
        FROM 
            page_settings  
        "
    )
    .fetch_all(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut map: HashMap<String, HashMap<String, bool>> = HashMap::new();
    for row in rows {
        map.entry(row.page)
            .or_default()
            .insert(row.setting, row.value);
    }

    Ok(map)
}

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
pub async fn set_page_active(
    page: &str,
    is_active: bool,
    db: State<'_, Database>,
) -> Result<(), String> {
    let value = if is_active { 1 } else { 0 };

    let result = sqlx::query!(
        "
        UPDATE
            page_settings
        SET
            value = ?
        WHERE
            page = ? 
        AND 
            setting = 'is_active'
        ",
        value,
        page
    )
    .execute(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    if result.rows_affected() != 1 {
        return Err("Expected to update exactly one row".into());
    }
    
    Ok(())
}

#[tauri::command]
pub async fn update_setting() -> Result<(), String> {
    Ok(())
}
