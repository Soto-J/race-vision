use crate::{domain::AppError, utils::WIDGETS};

use serde::Deserialize;
use tauri::{App, AppHandle, Manager, PhysicalPosition, PhysicalSize};
use tauri_plugin_store::StoreExt;

#[derive(Debug, Deserialize)]
struct WidgetLayout {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

pub fn register_widgets(app: &App) -> Result<(), AppError> {
    let tauri_store = app
        .store("widget-layouts.json")
        .map_err(|e| AppError::TauriError(format!("{e:?}")))?;

    for widget in WIDGETS {
        let widget_layout = match tauri_store.get(widget) {
            Some(val) => serde_json::from_value(val)
                .map_err(|e| {
                    AppError::TauriError(format!("Failed to parse layout for {widget:?}: {e}"))
                })
                .ok(),
            None => None,
        };

        init_widget_window(app.handle(), widget.as_ref(), widget_layout)?;
    }

    Ok(())
}

fn init_widget_window(
    app: &AppHandle,
    label: &str,
    widget_layout: Option<WidgetLayout>,
) -> Result<(), AppError> {
    let window = app
        .get_webview_window(label)
        .ok_or_else(|| AppError::InitializationFailed(format!("window {label} not found")))?;

    if let Some(widget) = widget_layout {
        window
            .set_position(PhysicalPosition::new(widget.x, widget.y))
            .map_err(|e| AppError::TauriError(format!("{e:?}")))?;

        window
            .set_size(PhysicalSize::new(widget.width, widget.height))
            .map_err(|e| AppError::TauriError(format!("{e:?}")))?;
    }

    window
        .show()
        .map_err(|e| AppError::TauriError(format!("{e:?}")))?;

    window
        .set_ignore_cursor_events(true)
        .map_err(|e| AppError::TauriError(format!("{e:?}")))?;

    Ok(())
}
