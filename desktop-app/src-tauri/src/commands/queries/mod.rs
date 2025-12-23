use tauri::State;
use tokio::sync::RwLock;

pub mod get_all_settings;
pub mod get_page_settings;
pub mod set_page_active;

pub use get_all_settings::*;
pub use get_page_settings::*;
pub use set_page_active::*;

#[tauri::command]
pub async fn set_watched_vars(
    vars: Vec<String>,
    watched_vars: State<'_, RwLock<Vec<String>>>,
) -> Result<(), String> {
    *watched_vars.write().await = vars;

    Ok(())
}
