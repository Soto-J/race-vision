//! Global keyboard shortcut handling.
//!
//! Provides application-wide shortcuts such as edit-mode toggling.
use crate::{
    domain::DomainError,
    utils::constants::{
        widget::{WidgetConfig, WIDGET_DEFINITIONS},
        TOGGLE_EDIT_MODE, WIDGET_LAYOUTS_KEY,
    },
};

use tauri::{App, AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_store::StoreExt;
use tokio::sync::RwLock;

/// Registers all global keyboard shortcuts for the application.
///
/// Currently registers:
/// - **Ctrl+F6**: Toggle edit mode for widget windows
///   - When enabled: Windows become draggable/resizable
///   - When disabled: Positions are saved and windows ignore cursor events
///
/// # Errors
/// Returns `DomainError::Shortcut` if shortcut registration fails
pub fn register_shortcuts(app: &App) -> Result<(), DomainError> {
    let ctrl_f6 = Shortcut::new(Some(Modifiers::CONTROL), Code::F6);

    app.global_shortcut()
        .on_shortcut(ctrl_f6, move |app, shortcut, event| {
            let ctrl_f6_is_pressed =
                shortcut == &ctrl_f6 && event.state() == ShortcutState::Pressed;

            if ctrl_f6_is_pressed {
                let state: tauri::State<RwLock<bool>> = app.state();

                let new_edit_mode = {
                    let mut edit_mode = state.blocking_write();
                    *edit_mode = !*edit_mode;

                    *edit_mode
                };

                if let Err(e) = app.emit(TOGGLE_EDIT_MODE, new_edit_mode) {
                    tracing::error!("Failed to emit {TOGGLE_EDIT_MODE} event: {e}");
                }

                if new_edit_mode {
                    // Entering edit mode → enable pointer events
                    enter_edit_mode(app);
                } else {
                    // Update widgets + persist only when leaving edit mode
                    if let Err(e) = exit_edit_mode_and_save(app) {
                        tracing::error!("Failed to exit edit mode cleanly: {e}");
                    }
                }
            }
        })?;

    Ok(())
}

fn enter_edit_mode(app: &AppHandle) {
    for widget in WIDGET_DEFINITIONS.iter() {
        if let Some(window) = app.get_webview_window(widget.label) {
            if let Err(e) = window.set_ignore_cursor_events(false) {
                tracing::warn!("Failed to update cursor events for {}: {e}", widget.label);
            }
        }
    }
}

fn exit_edit_mode_and_save(app: &AppHandle) -> Result<(), DomainError> {
    let tauri_store = app
        .store(WIDGET_LAYOUTS_KEY)
        .map_err(|e| DomainError::Tauri(format!("{e}")))?;

    for widget in WIDGET_DEFINITIONS.iter() {
        if let Some(window) = app.get_webview_window(widget.label) {
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
