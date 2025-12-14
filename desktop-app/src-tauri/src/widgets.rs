use crate::{domain::AppError, utils::WIDGETS};

use serde::Deserialize;
use tauri::{App, AppHandle, Manager, PhysicalPosition, PhysicalSize};
use tauri_plugin_store::StoreExt;

pub fn register_webview(app: &App) -> Result<(), AppError> {
    let webview_url = tauri::WebviewUrl::App("index.html".into());

    // DASHBOARD
    let dashboard_url = tauri::WebviewUrl::App("index.html#/dashboard".into());
    tauri::WebviewWindowBuilder::new(app, "dashboard", webview_url.clone())
        .title("Race Vision")
        .build()
        .map_err(|e| AppError::TauriError(format!("{e}")))?;

    // WIDGETS
    for widget in WIDGETS {
        let label = format!("{widget}-widget");
        let title = capitalize_widget(widget);

        let web_view = tauri::WebviewWindowBuilder::new(app, &label, webview_url.clone())
            .title(title)
            .build()
            .map_err(|e| AppError::TauriError(format!("{e}")))?;

        let tauri_store = app
            .store("widget-layouts.json")
            .map_err(|e| AppError::TauriError(format!("{e:?}")))?;

        match tauri_store.get(label) {
            Some(value) => {
                let position: WidgetLayout = serde_json::from_value(value).map_err(|e| {
                    AppError::TauriError(format!("Failed to parse layout for {widget:?}: {e}"))
                })?;

                web_view
                    .set_position(PhysicalPosition::new(position.x, position.y))
                    .map_err(|e| AppError::TauriError(format!("{e:?}")))?;
                web_view
                    .set_size(PhysicalSize::new(position.width, position.height))
                    .map_err(|e| AppError::TauriError(format!("{e:?}")))?;
            }
            None => {
                web_view
                    .set_position(PhysicalPosition::new(3, 2))
                    .map_err(|e| AppError::TauriError(format!("{e:?}")))?;
                web_view
                    .set_size(PhysicalSize::new(3, 2))
                    .map_err(|e| AppError::TauriError(format!("{e:?}")))?;
            }
        };
    }

    Ok(())
}

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

fn capitalize_widget(s: &str) -> String {
    let mut c = s.chars();

    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
