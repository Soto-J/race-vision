use tauri::State;
use tokio::sync::RwLock;

pub mod get_settings;
pub mod set_page_active;
pub mod update_setting;

pub use get_settings::*;
pub use set_page_active::*;
pub use update_setting::*;

#[tauri::command]
pub async fn set_watched_vars(
    vars: Vec<String>,
    watched_vars: State<'_, RwLock<Vec<String>>>,
) -> Result<(), String> {
    *watched_vars.write().await = vars;

    Ok(())
}
