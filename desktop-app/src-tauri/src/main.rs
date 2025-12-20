// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use desktop_app_lib::domain::DomainError;

fn main() -> Result<(), DomainError> {
    desktop_app_lib::run()?;

    Ok(())
}
