use crate::domain::{
    Database, DisplayIn, DisplayPageSettings, NestedSetting, PageConfig, PageSettingValue,
    PageSettings, Session,
};
use std::collections::{hash_map::Entry, HashMap};
use tauri::State;

#[tauri::command]
pub async fn get_all_page_settings(
    db: State<'_, Database>,
) -> Result<HashMap<String, PageConfig>, String> {
    let page_results = sqlx::query_as!(
        PageSettings,
        "
        SELECT
            page, 
            setting, 
            value != 0 AS 'value!: bool'
        FROM
            page_settings
        "
    )
    .fetch_all(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let display_results = sqlx::query_as!(
        DisplayPageSettings,
        "
        SELECT
            page, 
            setting, 
            session, 
            is_visible != 0 AS 'is_visible!: bool'
        FROM
            page_setting_display
        "
    )
    .fetch_all(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    let normalized = merge_settings(page_results, display_results);

    Ok(normalized)
}

fn merge_settings(
    page_results: Vec<PageSettings>,
    display_results: Vec<DisplayPageSettings>,
) -> HashMap<String, PageConfig> {
    // Build Displayin
    let mut display_index: HashMap<(String, String, Session), bool> = HashMap::new();

    for d in display_results {
        if let Some(session) = Session::from_str(&d.session) {
            display_index.insert((d.page, d.setting, session), d.is_visible);
        }
    }

    // Build nested structure per page
    let mut pages_map: HashMap<String, PageConfig> = HashMap::new();

    for setting in page_results {
        let page_name = &setting.page;
        let setting_key = &setting.setting;

        let page = pages_map
            .entry(page_name.clone())
            .or_insert_with(|| PageConfig {
                settings: HashMap::new(),
            });

        let mut iter = setting_key.splitn(2, '.');
        let first = iter.next().unwrap();
        let second = iter.next();

        if let Some(field) = second {
            let section = first.to_string();
            let field = field.to_string();

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

            let mut display_in = DisplayIn {
                practice: false,
                qualy: false,
                race: false,
            };

            for session in Session::ALL {
                let visible = display_index
                    .get(&(page_name.clone(), setting_key.clone(), session))
                    .copied()
                    .unwrap_or(false);

                match session {
                    Session::Practice => display_in.practice = visible,
                    Session::Qualy => display_in.qualy = visible,
                    Session::Race => display_in.race = visible,
                }
            }

            section_map.insert(
                field,
                NestedSetting {
                    is_active: setting.value,
                    display_in,
                },
            );
        } else {
            page.settings
                .insert(setting_key.clone(), PageSettingValue::Bool(setting.value));
        }
    }

    pages_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_nested_settings_correctly() {}
}
