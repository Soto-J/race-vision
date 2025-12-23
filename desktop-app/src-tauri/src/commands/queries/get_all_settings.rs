use crate::domain::{
    Database, DisplayIn, NestedSetting, PageConfig, PageSettingValue, PageSettings,
};
use std::collections::{hash_map::Entry, HashMap};
use tauri::State;

#[tauri::command]
pub async fn get_all_page_settings(
    db: State<'_, Database>,
) -> Result<HashMap<String, PageConfig>, String> {
    let page_settings = sqlx::query_as!(
        PageSettings,
        "
        SELECT
            page, setting, value != 0 AS 'value!: bool'
        FROM
            page_settings
        "
    )
    .fetch_all(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    // Fetch all display settings
    let display_settings = sqlx::query!(
        "
        SELECT
            page, setting, session, is_visible != 0 AS 'is_visible!: bool'
        FROM
            page_setting_display
        "
    )
    .fetch_all(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut display_index: HashMap<(String, String, String), bool> = HashMap::new();

    for d in display_settings {
        display_index.insert(
            (d.page.clone(), d.setting.clone(), d.session.clone()),
            d.is_visible,
        );
    }

    // Build nested structure per page
    let mut pages_map: HashMap<String, PageConfig> = HashMap::new();

    for setting in page_settings {
        let page = pages_map
            .entry(setting.page.clone())
            .or_insert_with(|| PageConfig {
                settings: HashMap::new(),
            });

        let parts: Vec<&str> = setting.setting.split('.').collect();

        if parts.len() == 1 {
            page.settings.insert(
                setting.setting.clone(),
                PageSettingValue::Bool(setting.value),
            );
        } else {
            let section = parts[0].to_string();
            let field = parts[1].to_string();

            let section_map = match page.settings.entry(section.clone()) {
                Entry::Vacant(e) => {
                    e.insert(PageSettingValue::Section(HashMap::new()));
                    match page.settings.get_mut(&section).unwrap() {
                        PageSettingValue::Section(map) => map,
                        _ => unreachable!(),
                    }
                }
                Entry::Occupied(e) => match e.into_mut() {
                    PageSettingValue::Section(map) => map,
                    _ => unreachable!(),
                },
            };

            let display_in = DisplayIn {
                practice: display_index
                    .get(&(
                        setting.page.clone(),
                        setting.setting.clone(),
                        "practice".into(),
                    ))
                    .copied()
                    .unwrap_or(false),
                qualy: display_index
                    .get(&(
                        setting.page.clone(),
                        setting.setting.clone(),
                        "qualy".into(),
                    ))
                    .copied()
                    .unwrap_or(false),
                race: display_index
                    .get(&(setting.page.clone(), setting.setting.clone(), "race".into()))
                    .copied()
                    .unwrap_or(false),
            };

            section_map.insert(
                field,
                NestedSetting {
                    is_active: setting.value,
                    display_in,
                },
            );
        }
    }

    Ok(pages_map)
}
