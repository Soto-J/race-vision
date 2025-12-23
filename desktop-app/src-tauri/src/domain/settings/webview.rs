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
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    // pub always_on_top: bool,
    // pub decorations: bool,
    // pub skip_taskbar: bool,
}

impl WebviewDefinition {
    pub fn new(
        label: impl Into<Cow<'static, str>>,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Self {
        Self {
            label: label.into(),
            x,
            y,
            width,
            height,
        }
    }

    pub fn logical_position(&self) -> LogicalPosition<f64> {
        LogicalPosition::new(self.x, self.y)
    }

    pub fn logical_size(&self) -> LogicalSize<f64> {
        LogicalSize::new(self.width, self.height)
    }
}