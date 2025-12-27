use super::types::*;
use crate::{
    domain::{DisplayIn, DisplayIndex},
    utils::to_camel_case,
};
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
                if section_name == "general" {
                    let general_section_map = page.general_section_mut(&section_name);

                    general_section_map.insert(field.to_string(), GeneralSetting { is_active });
                } else {
                    let section_map = page.section_mut(section_name);
                    let display_in =
                        DisplayIn::from_index(&display_index, &page_name, &setting_key);

                    section_map.insert(
                        field.to_string(),
                        NestedSetting {
                            is_active,
                            display_in,
                        },
                    );
                }
            } else {
                page.settings
                    .insert(setting_key.clone(), PageSettingValue::Bool(is_active));
            }
        }

        Self { index }
    }

    pub fn into_inner(self) -> HashMap<String, PageConfig> {
        self.index
    }
}

impl Deref for PageIndex {
    type Target = HashMap<String, PageConfig>;

    fn deref(&self) -> &Self::Target {
        &self.index
    }
}
