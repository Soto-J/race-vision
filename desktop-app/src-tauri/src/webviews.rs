//! Webview window registration and configuration.
//!
//! This module is responsible for creating and configuring all application
//! webview windows, including:
//! - The main dashboard window
//! - All widget overlay windows
//!
//! Widget windows:
//! - Are restored from persisted layout state when available
//! - Fall back to default definitions otherwise
//! - Are configured as always-on-top, decoration-less overlay windows
//!
//! Window geometry is expressed in logical units to ensure proper DPI scaling
//! across different displays.
use crate::{
    domain::DomainError,
    utils::constants::{
        widget::{WidgetConfig, WIDGET_DEFINITIONS},
        DEFAULT_DASHBOARD_HEIGHT, DEFAULT_DASHBOARD_WIDTH, WIDGET_LAYOUTS_KEY,
    },
};

use tauri::{App, LogicalPosition, LogicalSize, WebviewWindow};
use tauri_plugin_store::StoreExt;

/// Registers all application webviews.
///
/// This function creates:
/// - The main dashboard window
/// - All widget overlay windows
///
/// Widget windows are restored from persisted layout state when available.
/// If no saved layout exists for a widget, its default definition is used.
///
/// # Errors
/// Returns a [`DomainError`] if any webview fails to be created or configured.
pub fn register_webviews(app: &App) -> Result<(), DomainError> {
    let dashboard_url = tauri::WebviewUrl::App("index.html#/dashboard".into());

    let dashboard_webview = tauri::WebviewWindowBuilder::new(app, "dashboard", dashboard_url)
        .title("Race Vision")
        .build()
        .map_err(|e| DomainError::Tauri(format!("{e}")))?;

    dashboard_webview
        .set_size(LogicalSize::new(
            DEFAULT_DASHBOARD_WIDTH,
            DEFAULT_DASHBOARD_HEIGHT,
        ))
        .map_err(|e| {
            DomainError::Tauri(format!("failed to set size for dashboard webview: {e}"))
        })?;

    load_widget_webviews(app)?;

    Ok(())
}

/// Loads and configures all widget webviews.
///
/// For each widget definition:
/// - Attempts to restore the widget's layout from persistent storage
/// - Falls back to the widget's default layout if no saved state exists
/// - Applies overlay-style window configuration
///
/// # Errors
/// Returns a [`DomainError`] if layout restoration or window configuration fails.
fn load_widget_webviews(app: &App) -> Result<(), DomainError> {
    let tauri_store = app
        .store(WIDGET_LAYOUTS_KEY)
        .map_err(|e| DomainError::Tauri(format!("{e:?}")))?;

    for widget in WIDGET_DEFINITIONS.iter() {
        let widget_url =
            tauri::WebviewUrl::App(format!("index.html#/widget/{}", widget.label).into());

        let webview = tauri::WebviewWindowBuilder::new(app, widget.label, widget_url)
            .title(capitalize_widget(widget.label))
            .build()
            .map_err(|e| DomainError::Tauri(format!("failed to load {}: {}", widget.label, e)))?;

        match tauri_store.get(widget.label) {
            Some(value) => {
                let config: WidgetConfig = serde_json::from_value(value).map_err(|e| {
                    DomainError::Tauri(format!(
                        "Failed to parse layout for {}: {}",
                        widget.label, e,
                    ))
                })?;

                configure_webview(webview, config)?;
            }
            None => {
                tracing::info!("No saved layout for {}, using defaults", widget.label);
                configure_webview(webview, widget.into())?
            }
        };
    }

    tauri_store.close_resource();

    Ok(())
}

/// Applies overlay window configuration to a widget webview.
///
/// This function:
/// - Sets the window size and position
/// - Disables window decorations
/// - Removes the window from the taskbar
/// - Forces the window to remain always on top
///
/// All geometry is applied using logical units.
///
/// # Errors
/// Returns a [`DomainError`] if any window configuration step fails.
fn configure_webview(webview: WebviewWindow, config: WidgetConfig) -> Result<(), DomainError> {
    webview
        .set_size(LogicalSize::new(config.width, config.height))
        .map_err(|e| {
            DomainError::Tauri(format!("failed to set size for {}: {e}", webview.label()))
        })?;
    webview
        .set_position(LogicalPosition::new(config.x, config.y))
        .map_err(|e| {
            DomainError::Tauri(format!(
                "failed to set position for {}: {e}",
                webview.label()
            ))
        })?;

    webview.set_decorations(false).map_err(|e| {
        DomainError::Tauri(format!(
            "failed to set decorations for {}: {e}",
            webview.label()
        ))
    })?;
    webview.set_skip_taskbar(true).map_err(|e| {
        DomainError::Tauri(format!(
            "failed to set skip taskbar for {}:,{e}",
            webview.label()
        ))
    })?;
    webview.set_always_on_top(true).map_err(|e| {
        DomainError::Tauri(format!(
            "failed to set always on top for {}:,{e}",
            webview.label()
        ))
    })?;

    Ok(())
}

fn capitalize_widget(s: &str) -> String {
    let mut c = s.chars();

    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
