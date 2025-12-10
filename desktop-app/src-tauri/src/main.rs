// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use desktop_app_lib::domain::AppError;

fn main() -> Result<(), AppError> {
    desktop_app_lib::run()?;
    Ok(())
}
