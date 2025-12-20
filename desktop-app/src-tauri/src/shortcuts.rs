//! Global keyboard shortcut handling.
//!
//! Provides application-wide shortcuts such as edit-mode toggling.
use crate::{
    domain::{DomainError, WebviewDefinition},
    utils::constants::{widget::WIDGET_DEFINITIONS, TOGGLE_EDIT_MODE, WIDGET_LAYOUTS_KEY},
};

use sqlx::SqlitePool;
use tauri::{window, App, AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

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
    tracing::info!(phase = "startup", "registering shortcuts");

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
                    if let Err(e) = tauri::async_runtime::block_on(exit_edit_mode_and_save(app)) {
                        tracing::error!("Failed to exit edit mode cleanly: {e}");
                    };
                }
            }
        })?;

    Ok(())
}

fn enter_edit_mode(app: &AppHandle) {
    for widget in WIDGET_DEFINITIONS.iter() {
        if let Some(window) = app.get_webview_window(widget.label.as_ref()) {
            if let Err(e) = window.set_ignore_cursor_events(false) {
                tracing::warn!("Failed to update cursor events for {}: {e}", widget.label);
            }
        }
    }
}

async fn exit_edit_mode_and_save(app: &AppHandle) -> Result<(), DomainError> {
    let sqlite_pool = app.state::<SqlitePool>();

    for widget in WIDGET_DEFINITIONS.iter() {
        if let Some(window) = app.get_webview_window(widget.label.as_ref()) {
            if let Err(e) = window.set_ignore_cursor_events(true) {
                tracing::warn!("Failed to update cursor events for {}: {e}", widget.label);
            }

            if let (Ok(pos), Ok(size), Ok(scale)) = (
                window.outer_position(),
                window.outer_size(),
                window.scale_factor(),
            ) {
                let pos = pos.to_logical(scale);
                let size = size.to_logical(scale);

                let config = WebviewDefinition::new(
                    widget.label.as_ref(),
                    pos.x,
                    pos.y,
                    size.width,
                    size.height,
                );

                sqlx::query(
                    r#"
                    UPDATE
                        webview_layout
                    SET
                        x_axis = ?,
                        y_axis = ?,
                        width  = ?,
                        height = ?
                    WHERE
                        name = ?
                    "#,
                )
                .bind(config.x_axis)
                .bind(config.y_axis)
                .bind(config.width)
                .bind(config.height)
                .bind(config.label)
                .execute(&*sqlite_pool)
                .await?;
            }
        }
    }

    Ok(())
}
