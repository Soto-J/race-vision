use crate::domain::Database;

use tauri::State;

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
