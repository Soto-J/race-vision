use crate::{
    domain::AppError,
    utils::{WIDGET_LAYOUTS_KEY, WIDGET_WEBVIEWS},
};

use serde::{Deserialize, Serialize};
use tauri::{App, LogicalPosition, LogicalSize};
use tauri_plugin_store::StoreExt;

pub fn register_webviews(app: &App) -> Result<(), AppError> {
    let dashboard_url = tauri::WebviewUrl::App("index.html#/dashboard".into());

    let dashboard_webview = tauri::WebviewWindowBuilder::new(app, "dashboard", dashboard_url)
        .title("Race Vision")
        .build()
        .map_err(|e| AppError::Tauri(format!("{e}")))?;

    // dashboard_webview.set_position(PhysicalPosition::new(x, y));
    dashboard_webview
        .set_size(LogicalSize::new(900, 700))
        .map_err(|e| AppError::Tauri(format!("failed to set size for dashboard webview: {e}")))?;

    load_widget_webview(app)?;

    Ok(())
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct WidgetConfig {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

fn load_widget_webview(app: &App) -> Result<(), AppError> {
    let tauri_store = app
        .store(WIDGET_LAYOUTS_KEY)
        .map_err(|e| AppError::Tauri(format!("{e:?}")))?;

    for widget in WIDGET_WEBVIEWS.iter() {
        let widget_url =
            tauri::WebviewUrl::App(format!("index.html#/widget/{}", widget.label).into());

        let webview = tauri::WebviewWindowBuilder::new(app, widget.label, widget_url)
            .title(capitalize_widget(widget.label))
            .build()
            .map_err(|e| AppError::Tauri(format!("failed to load {}: {}", widget.label, e)))?;

        match tauri_store.get(widget.label) {
            Some(value) => {
                let position: WidgetConfig = serde_json::from_value(value).map_err(|e| {
                    AppError::Tauri(format!(
                        "Failed to parse layout for {}: {}",
                        widget.label, e,
                    ))
                })?;

                webview
                    .set_position(LogicalPosition::new(position.x, position.y))
                    .map_err(|e| AppError::Tauri(format!("{e:?}")))?;
                webview
                    .set_size(LogicalSize::new(position.width, position.height))
                    .map_err(|e| AppError::Tauri(format!("{e:?}")))?;
            }
            None => {
                webview
                    .set_position(widget.logical_position())
                    .map_err(|e| AppError::Tauri(format!("{e:?}")))?;
                webview
                    .set_size(widget.logical_size())
                    .map_err(|e| AppError::Tauri(format!("{e:?}")))?;
            }
        };
    }

    tauri_store.close_resource();

    Ok(())
}

fn capitalize_widget(s: &str) -> String {
    let mut c = s.chars();

    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
