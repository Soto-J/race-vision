use crate::domain::{
    Database, DisplayIn, DisplayKeys, DisplayPageSettings, NestedSetting, PageConfig,
    PageSettingValue, PageSettings, Session,
};
use std::collections::HashMap;
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

    let display_index: HashMap<DisplayKeys, bool> = display_results
        .into_iter()
        .filter_map(|d| {
            Session::from_str(&d.session)
                .map(|session| (DisplayKeys::parse(d.page, d.setting, session), d.is_visible))
        })
        .collect();

    // Build nested structure per page
    let mut pages_map: HashMap<String, PageConfig> = HashMap::new();

    for setting in page_results {
        let page_name = &setting.page;
        let setting_key = &setting.setting;

        let page = pages_map
            .entry(page_name.to_owned())
            .or_insert_with(|| PageConfig {
                settings: HashMap::new(),
            });

        let mut iter = setting_key.splitn(2, '.');
        let first = iter.next().unwrap();
        let second = iter.next();

        if let Some(field) = second {
            let section = first.to_string();
            let field = field.to_string();

            let section_map = page.section_mut(&section);

            let display_in = DisplayIn::from_index(&display_index, page_name, setting_key);

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
    use crate::get_sqlite_pool;

    #[tokio::test]
    async fn merges_nested_settings_correctly() {
        let pool = get_sqlite_pool().await;
        let db = Database::new(pool);

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
        .expect("failed to get page_settings");

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
        .expect("failed to get page_settings_display");

        let map = merge_settings(page_results, display_results);

        let json = serde_json::to_string_pretty(&map).expect("failed to serialize");
        println!("{}", json);
    }
}

fn to_camel_case(word: String) -> String {
    let res = word
        .split("_")
        .into_iter()
        .map(|w| w.chars().into_iter()[0].to_upper_case())
        .collect()
        .join("");
}
