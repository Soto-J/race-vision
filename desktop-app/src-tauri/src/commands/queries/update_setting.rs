use crate::domain::{Database};

use tauri::State;

struct Response {
    status: String,
    message: String,
}

#[tauri::command]
pub async fn update_setting(page: &str, db: State<'_, Database>) -> Result<String, String> {
    todo!()
}
