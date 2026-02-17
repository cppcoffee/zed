use crate::{
    SettingField, SettingItem, SettingsPage, SettingsPageItem, USER,
};
use super::utils::concat_sections;

pub fn debugger_page() -> SettingsPage {
    fn general_section() -> [SettingsPageItem; 2] {
        [
            SettingsPageItem::SectionHeader("General"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Stepping Granularity",
                description: "The granularity of stepping through code.",
                field: Box::new(SettingField {
                    json_path: Some("debugger.stepping_granularity"),
                    pick: |settings_content| {
                        settings_content
                            .debugger
                            .as_ref()?
                            .stepping_granularity
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .debugger
                            .get_or_insert_default()
                            .stepping_granularity = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show Active Status",
                description: "Show the active debugger status in the status bar.",
                field: Box::new(SettingField {
                    json_path: Some("debugger.show_active_status"),
                    pick: |settings_content| {
                        settings_content
                            .debugger
                            .as_ref()?
                            .show_active_status
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .debugger
                            .get_or_insert_default()
                            .show_active_status = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    SettingsPage {
        title: "Debugger",
        items: concat_sections!(general_section()),
    }
}
