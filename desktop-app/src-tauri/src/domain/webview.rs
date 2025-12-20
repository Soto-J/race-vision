use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use tauri::{LogicalPosition, LogicalSize};

// Cow = Clone-On-Write
// Borrow if possible
// Own when necessary
//
// Visual mental model
// &'static str
// │
// ├── fast
// ├── zero allocation
// ├── compile-time only
// └── inflexible
//
// Cow<'static, str>
// │
// ├── Borrowed(&'static str)   ← no alloc
// ├── Owned(String)            ← runtime data
// ├── flexible
// └── safe for long-lived storage

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebviewDefinition {
    pub label: Cow<'static, str>,
    pub x_axis: f64,
    pub y_axis: f64,
    pub width: f64,
    pub height: f64,
    // pub always_on_top: bool,
    // pub decorations: bool,
    // pub skip_taskbar: bool,
}

impl WebviewDefinition {
    pub fn new(
        label: impl Into<Cow<'static, str>>,
        x_axis: f64,
        y_axis: f64,
        width: f64,
        height: f64,
    ) -> Self {
        Self {
            label: label.into(),
            x_axis,
            y_axis,
            width,
            height,
        }
    }

    pub fn logical_position(&self) -> LogicalPosition<f64> {
        LogicalPosition::new(self.x_axis, self.y_axis)
    }

    pub fn logical_size(&self) -> LogicalSize<f64> {
        LogicalSize::new(self.width, self.height)
    }
}

// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// pub struct WidgetConfig {
//     pub x_axis: f64,
//     pub y_axis: f64,
//     pub width: f64,
//     pub height: f64,
// }

// impl WidgetConfig {
//     pub fn new(x_axis: f64, y_axis: f64, width: f64, height: f64) -> Self {
//         Self {
//             x_axis,
//             y_axis,
//             width,
//             height,
//         }
//     }
// }

// impl From<&WidgetDefinition> for WidgetConfig {
//     fn from(def: &WidgetDefinition) -> Self {
//         Self {
//             x_axis: def.position.x,
//             y_axis: def.position.y,
//             width: def.size.width,
//             height: def.size.height,
//         }
//     }
// }
