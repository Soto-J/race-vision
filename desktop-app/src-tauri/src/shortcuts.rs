use crate::{domain::AppError, utils::WIDGETS};

use tauri::{App, Emitter, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tokio::sync::RwLock;

pub fn register_shortcuts(app: &App) -> Result<(), AppError> {
    let f6 = Shortcut::new(Some(Modifiers::CONTROL), Code::F6);

    app.global_shortcut()
        .on_shortcut(f6, move |app, shortcut, event| {
            if shortcut == &f6 && event.state() == ShortcutState::Pressed {
                let state: tauri::State<RwLock<bool>> = app.state();

                let mut mode = state.blocking_write();

                *mode = !*mode;

                let _ = app.emit("toggle-edit-mode", *mode);

                // update each widget window
                for widget in WIDGETS {
                    if let Some(window) = app.get_webview_window(widget) {
                        // If edit mode: allow dragging → enable pointer events
                        // If not edit mode: disable all interactions
                        let _ = window.set_ignore_cursor_events(*mode);
                    }
                }
            }
        })?;

    Ok(())
}

fn toggle_cursor_event(app: &App) -> Result<(), AppError> {
    for widget in WIDGETS {
        let window = app
            .get_webview_window(widget)
            .ok_or_else(|| AppError::InitializationFailed(format!("window {widget} not found")))?;
    }

    Ok(())
}
