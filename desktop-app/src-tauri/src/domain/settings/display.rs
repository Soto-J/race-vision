use super::Session;
use crate::domain::DomainError;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A lookup index for session-specific display visibility.
///
/// `DisplayIndex` is built from raw `DisplayPageSettings` rows and provides
/// efficient access to visibility rules keyed by `(page, setting, session)`.
///
/// This type is intended as an internal helper used during configuration
/// assembly and is not part of the public API surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisplayIndex {
    index: HashMap<DisplayKeys, bool>,
}

impl DisplayIndex {
    /// Builds a `DisplayIndex` from raw display settings.
    ///
    /// Invalid rows (e.g. those containing unknown session values) are
    /// discarded during construction.
    ///
    /// # Parameters
    /// - `display_results`: Raw rows from the `display_page_settings` query.
    ///
    /// # Behavior
    /// - Parses session strings into `Session` values
    /// - Indexes valid rows by `(page, setting, session)`
    /// - Silently drops invalid rows
    pub fn build(display_results: Vec<DisplayPageSettings>) -> Self {
        let index: HashMap<DisplayKeys, bool> = display_results
            .into_iter()
            .filter_map(|d| {
                let is_visible = d.is_visible;

                DisplayKeys::try_from(d).ok().map(|key| (key, is_visible))
            })
            .collect();

        Self { index }
    }
}

/// Crate-internal key used for indexing display visibility.
///
/// Combines a page, setting, and session into a single hashable key.
/// This type is intentionally crate-private and should not be exposed
/// outside the configuration domain.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DisplayKeys {
    page: String,
    setting: String,
    session: Session,
}

impl TryFrom<DisplayPageSettings> for DisplayKeys {
    type Error = DomainError;
    /// Attempts to construct a `DisplayKeys` from a raw database row.
    ///
    /// # Errors
    /// Returns `DomainError::InvalidSession` if the session string
    /// cannot be parsed into a valid `Session`.
    fn try_from(value: DisplayPageSettings) -> Result<Self, Self::Error> {
        let session =
            Session::from_str(&value.session).ok_or_else(|| DomainError::InvalidSession)?;

        Ok(DisplayKeys {
            page: value.page,
            setting: value.setting,
            session,
        })
    }
}

/// A raw display visibility row.
///
/// This type represents the direct result of the
/// `display_page_settings` query before validation or indexing.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayPageSettings {
    /// Page identifier (e.g. `"inputs"`).
    pub page: String,
    /// Setting identifier (e.g. `"inputsThrottle"`).
    pub setting: String,
    /// Session name as stored in the database.
    pub session: String,
    /// Whether the setting should be visible in the given session.
    pub is_visible: bool,
}

/// Session-specific visibility flags for a setting.
///
/// This type is part of the public API surface and is designed to
/// serialize cleanly for frontend consumption.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayIn {
    pub practice: bool,
    pub qualy: bool,
    pub race: bool,
}

impl DisplayIn {
    /// Resolves session visibility for a setting from a `DisplayIndex`.
    ///
    /// Missing entries default to `false`.
    ///
    /// # Parameters
    /// - `index`: The prebuilt display visibility index.
    /// - `page`: Page identifier.
    /// - `setting`: Setting identifier.
    ///
    /// # Returns
    /// A fully populated `DisplayIn` value with per-session visibility.
    pub fn from_index(index: &DisplayIndex, page: &str, setting: &str) -> Self {
        let mut display_in = DisplayIn {
            practice: false,
            qualy: false,
            race: false,
        };

        for session in Session::ALL {
            let key = DisplayKeys {
                page: page.to_owned(),
                setting: setting.to_owned(),
                session,
            };

            let visible = index.index.get(&key).copied().unwrap_or(false);

            match session {
                Session::Practice => display_in.practice = visible,
                Session::Qualy => display_in.qualy = visible,
                Session::Race => display_in.race = visible,
            }
        }

        display_in
    }
}
