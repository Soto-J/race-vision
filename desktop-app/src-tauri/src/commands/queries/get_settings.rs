use crate::domain::{
    types::{PageConfig, PageSettings},
    Database, DisplayIndex, DisplayPageSettings, PageIndex,
};
use std::collections::HashMap;
use tauri::State;

#[tauri::command]
pub async fn get_all_page_settings(
    db: State<'_, Database>,
) -> Result<HashMap<String, PageConfig>, String> {
    let page_results = sqlx::query_as!(
        PageSettings,
        "
        SELECT
            page, 
            setting, 
            value != 0 AS 'value!: bool'
        FROM
            page_settings
        "
    )
    .fetch_all(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let display_results = sqlx::query_as!(
        DisplayPageSettings,
        "
        SELECT
            page, 
            setting, 
            session, 
            is_visible != 0 AS 'is_visible!: bool'
        FROM
            page_setting_display
        "
    )
    .fetch_all(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(merge_settings(page_results, display_results))
}

fn merge_settings(
    page_results: Vec<PageSettings>,
    display_results: Vec<DisplayPageSettings>,
) -> HashMap<String, PageConfig> {
    let display_index = DisplayIndex::build(display_results);

    PageIndex::build(page_results, display_index).into_inner()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_sqlite_pool;

    #[tokio::test]
    async fn merges_nested_settings_correctly() {
        let pool = get_sqlite_pool().await;
        let db = Database::new(pool);

        let page_results = sqlx::query_as!(
            PageSettings,
            "
            SELECT
                page, 
                setting, 
                value != 0 AS 'value!: bool'
            FROM
                page_settings
            "
        )
        .fetch_all(&db.pool)
        .await
        .expect("failed to get page_settings");

        let display_results = sqlx::query_as!(
            DisplayPageSettings,
            "
            SELECT
                page, 
                setting, 
                session, 
                is_visible != 0 AS 'is_visible!: bool'
            FROM
                page_setting_display
            "
        )
        .fetch_all(&db.pool)
        .await
        .expect("failed to get page_settings_display");

        let map = merge_settings(page_results, display_results);

        let json = serde_json::to_string_pretty(&map).expect("failed to serialize");
        println!("{}", json);
    }
}
