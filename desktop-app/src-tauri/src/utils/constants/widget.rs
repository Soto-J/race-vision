use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tauri::{LogicalPosition, LogicalSize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WidgetDefinition {
    pub label: &'static str,

    pub size: Size,
    pub position: Position,
    // pub always_on_top: bool,
    // pub decorations: bool,
    // pub skip_taskbar: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl WidgetDefinition {
    pub fn new(label: &'static str, x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            label,
            position: Position { x, y },
            size: Size { width, height },
        }
    }

    pub fn logical_position(&self) -> LogicalPosition<f64> {
        LogicalPosition::new(self.position.x, self.position.y)
    }

    pub fn logical_size(&self) -> LogicalSize<f64> {
        LogicalSize::new(self.size.width, self.size.height)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WidgetConfig {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl WidgetConfig {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl From<&WidgetDefinition> for WidgetConfig {
    fn from(def: &WidgetDefinition) -> Self {
        Self {
            x: def.position.x,
            y: def.position.y,
            width: def.size.width,
            height: def.size.height,
        }
    }
}

pub static WIDGET_DEFINITIONS: Lazy<Vec<WidgetDefinition>> = Lazy::new(|| {
    vec![
        WidgetDefinition::new("inputs", 50.0, 50.0, 400.0, 150.0),
        WidgetDefinition::new("standings", 200.0, 200.0, 400.0, 600.0),
        WidgetDefinition::new("track-map", 600.0, 200.0, 300.0, 300.0),
        WidgetDefinition::new("relative", 100.0, 500.0, 400.0, 200.0),
    ]
});
