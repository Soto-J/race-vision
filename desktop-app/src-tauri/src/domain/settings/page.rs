use crate::domain::{display::DisplayIn, DisplayIndex};

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Deref};

/// An indexed, API-ready view of page settings.
///
/// `PageIndex` is a builder-style wrapper around a `HashMap<String, PageConfig>`
/// that transforms flat database rows (`PageSettings`) into the nested structure
/// expected by the frontend.
///
/// ## Responsibilities
/// - Groups settings by page
/// - Splits dotted setting keys into sections (e.g. `inputs.throttle`)
/// - Normalizes keys to `camelCase`
/// - Merges per-setting display rules from `DisplayIndex`
/// - Produces a fully-owned, serializable structure
///
/// ## Invariants
/// - A setting key is either:
///   - a top-level boolean (`revLights`)
///   - or a section containing nested settings (`inputs.throttle`)
/// - Once a key becomes a section, it must not later become a boolean
///
/// This type intentionally owns all data; cloning small strings here is expected
/// and acceptable, as this builder runs once during configuration load.
#[derive(Debug, Default)]
pub struct PageIndex {
    index: HashMap<String, PageConfig>,
}

impl PageIndex {
    /// Builds a `PageIndex` from flat page settings and display rules.
    ///
    /// # Parameters
    /// - `page_results`: Raw rows from the `page_settings` query.
    ///   Each row represents a single boolean setting.
    /// - `display_index`: Precomputed visibility rules keyed by
    ///   `(page, setting, session)`.
    ///
    /// # Behavior
    /// - Groups settings by page
    /// - Converts setting keys to `camelCase`
    /// - Splits dotted keys (`section.field`) into nested sections
    /// - Attaches `DisplayIn` metadata to nested settings
    ///
    /// # Example
    /// A setting with key:
    /// ```text
    /// inputs.throttle
    /// ```
    /// becomes:
    /// ```json
    /// {
    ///   "inputs": {
    ///     "throttle": {
    ///       "isActive": true,
    ///       "displayIn": { ... }
    ///     }
    ///   }
    /// }
    /// ```
    pub fn build(page_results: Vec<PageSettings>, display_index: DisplayIndex) -> Self {
        let mut index = HashMap::new();

        for setting in page_results {
            let PageSettings {
                page: page_name,
                setting,
                value: is_active,
            } = setting;

            let page = index
                .entry(page_name.clone())
                .or_insert_with(|| PageConfig {
                    settings: HashMap::new(),
                });

            let setting_key = to_camel_case(&setting);

            let mut iter = setting_key.splitn(2, '.');
            let section_name = iter.next().unwrap();
            let field_name = iter.next();

            if let Some(field) = field_name {
                let section_map = page.section_mut(&section_name);

                let display_in = DisplayIn::from_index(&display_index, &page_name, &setting_key);

                section_map.insert(
                    field.to_string(),
                    NestedSetting {
                        is_active,
                        display_in,
                    },
                );
            } else {
                page.settings
                    .insert(setting_key.clone(), PageSettingValue::Bool(is_active));
            }
        }

        Self { index }
    }

    /// Consumes the index and returns the underlying map.
    pub fn into_inner(self) -> HashMap<String, PageConfig> {
        self.index
    }
}

impl Deref for PageIndex {
    type Target = HashMap<String, PageConfig>;

    /// Allows read-only access to the underlying map without consuming `PageIndex`.
    fn deref(&self) -> &Self::Target {
        &self.index
    }
}

/// A single row from the `page_settings` query.
///
/// This represents a flat, boolean configuration value before
/// any grouping or transformation is applied.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PageSettings {
    /// Page identifier (e.g. `"inputs"`, `"standings"`).
    pub page: String,
    /// Raw setting key, potentially dotted and underscored
    /// (e.g. `"inputs_throttle"` or `"inputs.throttle"`).
    pub setting: String,
    /// Whether the setting is enabled.
    pub value: bool,
}

/// A value in a page configuration.
///
/// A setting is either:
/// - a simple boolean value
/// - or a section containing nested settings
///
/// This enum is `untagged` to produce a clean JSON shape for the frontend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PageSettingValue {
    /// A top-level boolean setting.
    Bool(bool),
    /// A section containing nested settings.
    Section(HashMap<String, NestedSetting>),
}

/// A nested setting belonging to a section.
///
/// Nested settings always include both an activation flag
/// and session-specific display rules.
impl PageSettingValue {
    pub fn as_section_mut(&mut self) -> &mut HashMap<String, NestedSetting> {
        match self {
            PageSettingValue::Section(map) => map,
            _ => panic!("PageSettingValue was not a Section"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NestedSetting {
    /// Whether the setting is enabled.
    pub is_active: bool,
    /// Session-specific visibility rules.
    pub display_in: DisplayIn,
}

/// Configuration for a single page.
///
/// Settings are flattened so that both top-level booleans and
/// nested sections appear at the same level in the JSON output.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageConfig {
    /// Page settings keyed by their `camelCase` name.
    #[serde(flatten)]
    pub settings: HashMap<String, PageSettingValue>,
}

impl PageConfig {
    /// Returns a mutable reference to a section, creating it if necessary.
    pub fn section_mut(&mut self, name: &str) -> &mut HashMap<String, NestedSetting> {
        self.settings
            .entry(name.to_string())
            .or_insert_with(|| PageSettingValue::Section(HashMap::new()))
            .as_section_mut()
    }
}

fn to_camel_case(s: &str) -> String {
    let mut parts = s.split("_");

    let first = parts.next().unwrap_or("").to_lowercase();

    let rest = parts.into_iter().map(|part| {
        let mut chars = part.chars();

        match chars.next() {
            Some(ch) => ch.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    });

    first + rest.collect::<String>().as_str()
}
