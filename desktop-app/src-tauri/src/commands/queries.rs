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
pub async fn page_settings(
    page: Option<&str>,
    db: State<'_, Database>,
) -> Result<Vec<PageSettings>, String> {
    match page {
        Some(p) => {
            sqlx::query_as!(
                PageSettings,
                "
                SELECT 
                    page, setting, value 
                FROM 
                    page_settings
                WHERE
                    page == ?    
                ",
                p
            )
            .fetch_all(&db.pool)
            .await
        }
        None => {
            sqlx::query_as!(
                PageSettings,
                "
                SELECT 
                    page, setting, value 
                FROM 
                    page_settings  
                "
            )
            .fetch_all(&db.pool)
            .await
        }
    }
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_page(page: &str, db: State<'_, Database>) -> Result<bool, String> {
    let row = sqlx::query!(
        "
        UPDATE
            page_settings
        SET
            value = NOT value
        WHERE
            page = ? 
        AND 
            setting = 'is_active'
        RETURNING
            value
        ",
        page
    )
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(row.value == 1)
}

#[tauri::command]
pub async fn update_setting() -> Result<(), String> {
    Ok(())
}
