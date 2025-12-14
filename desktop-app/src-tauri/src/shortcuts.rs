use crate::{
    domain::AppError,
    utils::{WidgetConfig, WIDGET_LAYOUTS_KEY, WIDGET_WEBVIEWS},
};

use tauri::{App, AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_store::StoreExt;
use tokio::sync::RwLock;

pub fn register_shortcuts(app: &App) -> Result<(), AppError> {
    let tauri_store = app
        .store(WIDGET_LAYOUTS_KEY)
        .map_err(|e| AppError::Tauri(format!("{e}")))?;

    let ctrl_f6 = Shortcut::new(Some(Modifiers::CONTROL), Code::F6);

    app.global_shortcut()
        .on_shortcut(ctrl_f6, move |app, shortcut, event| {
            if shortcut == &ctrl_f6 && event.state() == ShortcutState::Pressed {
                let state: tauri::State<RwLock<bool>> = app.state();

                let new_edit_mode = {
                    let mut edit_mode = state.blocking_write();
                    *edit_mode = !*edit_mode;

                    *edit_mode
                };

                if let Err(e) = app.emit("toggle-edit-mode", new_edit_mode) {
                    tracing::error!("Failed to emit toggle-edit-mode event: {e}");
                }

                if new_edit_mode == true {
                    // Update widgets + persist only when leaving edit mode
                    exit_edit_mode_and_save(app);
                } else {
                    // Entering edit mode → enable pointer events
                    enter_edit_mode(app);
                }
            }
        })?;

    Ok(())
}

fn enter_edit_mode(app: &AppHandle) {
    for widget in WIDGET_WEBVIEWS.iter() {
        let window_label = format!("{}-widget", widget.label);
        if let Some(window) = app.get_webview_window(&window_label) {
            let _ = window.set_ignore_cursor_events(false);
        }
    }
}

fn exit_edit_mode_and_save(app: &AppHandle) -> Result<(), AppError> {
    let tauri_store = app
        .store(WIDGET_LAYOUTS_KEY)
        .map_err(|e| AppError::Tauri(format!("{e}")))?;

    for widget in WIDGET_WEBVIEWS.iter() {
        let window_label = format!("{}-widget", widget.label);

        if let Some(window) = app.get_webview_window(&window_label) {
            if let Err(e) = window.set_ignore_cursor_events(true) {
                tracing::warn!("Failed to update cursor events for {}: {e}", widget.label);
            }

            if let (Ok(pos), Ok(size)) = (window.outer_position(), window.outer_size()) {
                let config = WidgetConfig::new(
                    pos.x as f64,
                    pos.y as f64,
                    size.width as f64,
                    size.height as f64,
                );

                if let Ok(value) = serde_json::to_value(config) {
                    tauri_store.set(widget.label, value);
                }
            }
        }
    }

    if let Err(e) = tauri_store.save() {
        tracing::error!("Failed to persist widget layouts: {e}");
    }

    Ok(())
}
