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
    utils::constants::{DEFAULT_DASHBOARD_HEIGHT, DEFAULT_DASHBOARD_WIDTH},
};

use sqlx::{Row, SqlitePool};
use tauri::{App, LogicalPosition, LogicalSize, Manager, WebviewWindow};

/// Registers the main dashboard webview window.
///
/// This function creates the main dashboard window with default dimensions.
/// Widget overlay windows are registered separately via [`register_widget_webviews`].
///
/// # Errors
/// Returns a [`DomainError`] if the dashboard webview fails to be created or configured.
pub fn register_dashboard(app: &App) -> Result<(), DomainError> {
    tracing::info!(phase = "startup", "registering webviews");

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

    Ok(())
}

/// Loads and configures all widget webviews from the database.
///
/// This async function queries the SQLite database for all saved widget layouts
/// and creates corresponding webview windows with their persisted geometry.
/// Each widget is configured as an overlay-style window.
///
/// Only widgets that exist in the database will be created. No default widgets
/// are created if the database is empty.
///
/// # Errors
/// Returns a [`DomainError`] if database queries fail or window configuration fails.
pub async fn register_widget_webviews(app: &App) -> Result<(), DomainError> {
    tracing::info!("loading widget webviews");

    let sqlite_pool = app.state::<SqlitePool>();

    let widget_layouts = sqlx::query("SELECT * FROM webview_layout")
        .fetch_all(&*sqlite_pool)
        .await?;

    for widget in widget_layouts {
        let name = widget.get("name");
        let x = widget.get("x_axis");
        let y = widget.get("y_axis");
        let width = widget.get("width");
        let height = widget.get("height");
        tracing::info!("restoring {} → {}x{} @ ({}, {})", name, width, height, x, y);

        let webview = tauri::WebviewWindowBuilder::new(
            app,
            name,
            tauri::WebviewUrl::App(format!("index.html#/widget/{}", name).into()),
        )
        .title(capitalize(name))
        .build()
        .map_err(|e| DomainError::Tauri(format!("failed to load {}: {}", name, e)))?;

        configure_webview(webview, x, y, width, height)?;
    }

    Ok(())
}

/// Applies overlay window configuration to a widget webview.
///
/// This function:
/// - Sets the window size and position from provided coordinates
/// - Disables window decorations
/// - Removes the window from the taskbar
/// - Forces the window to remain always on top
///
/// All geometry is applied using logical units.
///
/// # Parameters
/// - `webview`: The webview window to configure
/// - `x`, `y`: Logical position coordinates
/// - `width`, `height`: Logical dimensions
///
/// # Errors
/// Returns a [`DomainError`] if any window configuration step fails.
fn configure_webview(
    webview: WebviewWindow,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> Result<(), DomainError> {
    webview
        .set_size(LogicalSize::new(width, height))
        .map_err(|e| {
            DomainError::Tauri(format!("failed to set size for {}: {e}", webview.label()))
        })?;
    webview
        .set_position(LogicalPosition::new(x, y))
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
            "failed to set skip taskbar for {}: {e}",
            webview.label()
        ))
    })?;
    webview.set_always_on_top(true).map_err(|e| {
        DomainError::Tauri(format!(
            "failed to set always on top for {}: {e}",
            webview.label()
        ))
    })?;

    Ok(())
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();

    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
