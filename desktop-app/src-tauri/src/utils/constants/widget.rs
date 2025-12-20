use once_cell::sync::Lazy;

use crate::domain::WebviewDefinition;

pub static WIDGET_DEFINITIONS: Lazy<Vec<WebviewDefinition>> = Lazy::new(|| {
    vec![
        WebviewDefinition::new("inputs", 50.0, 50.0, 400.0, 150.0),
        WebviewDefinition::new("standings", 200.0, 200.0, 400.0, 600.0),
        WebviewDefinition::new("track-map", 600.0, 200.0, 300.0, 300.0),
        WebviewDefinition::new("relative", 100.0, 500.0, 400.0, 200.0),
    ]
});
