use crate::{
    SettingField, SettingItem, SettingsFieldMetadata, SettingsPage, SettingsPageItem, USER,
};
use super::utils::concat_sections;

pub fn network_page() -> SettingsPage {
    fn network_section() -> [SettingsPageItem; 3] {
        [
            SettingsPageItem::SectionHeader("Network"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Proxy",
                description: "The proxy to use for network requests.",
                field: Box::new(SettingField {
                    json_path: Some("proxy"),
                    pick: |settings_content| settings_content.proxy.as_ref(),
                    write: |settings_content, value| {
                        settings_content.proxy = value;
                    },
                }),
                metadata: Some(Box::new(SettingsFieldMetadata {
                    placeholder: Some("socks5h://localhost:10808"),
                    ..Default::default()
                })),
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Server URL",
                description: "The URL of the Zed server to connect to.",
                field: Box::new(SettingField {
                    json_path: Some("server_url"),
                    pick: |settings_content| settings_content.server_url.as_ref(),
                    write: |settings_content, value| {
                        settings_content.server_url = value;
                    },
                }),
                metadata: Some(Box::new(SettingsFieldMetadata {
                    placeholder: Some("https://zed.dev"),
                    ..Default::default()
                })),
                files: USER,
            }),
        ]
    }

    SettingsPage {
        title: "Network",
        items: concat_sections![network_section()],
    }
}
