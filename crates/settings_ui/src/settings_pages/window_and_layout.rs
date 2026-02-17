use crate::{
    DynamicItem, SettingField, SettingItem, SettingsPage, SettingsPageItem, USER,
};
use super::utils::{concat_sections, dynamic_variants, show_scrollbar_or_editor};
use settings::{self};
use strum::IntoDiscriminant as _;

pub fn window_and_layout_page() -> SettingsPage {
    fn window_section() -> [SettingsPageItem; 6] {
        [
            SettingsPageItem::SectionHeader("Window"),
            SettingsPageItem::DynamicItem(DynamicItem {
                discriminant: SettingItem {
                    files: USER,
                    title: "Window Decorations",
                    description: "Whether to use client-side or server-side window decorations.",
                    field: Box::new(SettingField {
                        json_path: Some("window_decorations$"),
                        pick: |settings_content| {
                            Some(
                                &dynamic_variants::<settings::WindowDecorations>()[settings_content
                                    .workspace
                                    .window_decorations
                                    .as_ref()?
                                    .discriminant()
                                    as usize],
                            )
                        },
                        write: |settings_content, value| {
                            let Some(value) = value else {
                                settings_content.workspace.window_decorations = None;
                                return;
                            };
                            let settings_value = settings_content
                                .workspace
                                .window_decorations
                                .get_or_insert_with(|| settings::WindowDecorations::Server);
                            *settings_value = match value {
                                settings::WindowDecorationsDiscriminants::Server => {
                                    settings::WindowDecorations::Server
                                }
                                settings::WindowDecorationsDiscriminants::Client => {
                                    settings::WindowDecorations::Client
                                }
                            };
                        },
                    }),
                    metadata: None,
                },
                pick_discriminant: |settings_content| {
                    Some(
                        settings_content
                            .workspace
                            .window_decorations
                            .as_ref()?
                            .discriminant() as usize,
                    )
                },
                fields: dynamic_variants::<settings::WindowDecorations>()
                    .into_iter()
                    .map(|variant| match variant {
                        settings::WindowDecorationsDiscriminants::Server => vec![],
                        settings::WindowDecorationsDiscriminants::Client => vec![],
                    })
                    .collect(),
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Window Background",
                description: "Window background appearance.",
                field: Box::new(SettingField {
                    json_path: Some("window_background"),
                    pick: |settings_content| settings_content.workspace.window_background.as_ref(),
                    write: |settings_content, value| {
                        settings_content.workspace.window_background = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Centered Layout",
                description: "Whether to center the layout when the window is wide.",
                field: Box::new(SettingField {
                    json_path: Some("centered_layout.enabled"),
                    pick: |settings_content| {
                        settings_content
                            .workspace
                            .centered_layout
                            .as_ref()?
                            .enabled
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .workspace
                            .centered_layout
                            .get_or_insert_default()
                            .enabled = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Centered Layout Left Padding",
                description: "The left padding for the centered layout.",
                field: Box::new(SettingField {
                    json_path: Some("centered_layout.left_padding"),
                    pick: |settings_content| {
                        settings_content
                            .workspace
                            .centered_layout
                            .as_ref()?
                            .left_padding
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .workspace
                            .centered_layout
                            .get_or_insert_default()
                            .left_padding = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Centered Layout Right Padding",
                description: "The right padding for the centered layout.",
                field: Box::new(SettingField {
                    json_path: Some("centered_layout.right_padding"),
                    pick: |settings_content| {
                        settings_content
                            .workspace
                            .centered_layout
                            .as_ref()?
                            .right_padding
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .workspace
                            .centered_layout
                            .get_or_insert_default()
                            .right_padding = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Active Pane Magnification",
                description: "Magnify the active pane.",
                field: Box::new(SettingField {
                    json_path: Some("active_pane_magnification"),
                    pick: |settings_content| {
                        settings_content.workspace.active_pane_magnification.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content.workspace.active_pane_magnification = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    fn layout_section() -> [SettingsPageItem; 6] {
        [
            SettingsPageItem::SectionHeader("Layout"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Pane Split Direction - Horizontal",
                description: "Direction to split panes when splitting horizontally.",
                field: Box::new(SettingField {
                    json_path: Some("pane_split_direction_horizontal"),
                    pick: |settings_content| {
                        settings_content
                            .workspace
                            .pane_split_direction_horizontal
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content.workspace.pane_split_direction_horizontal = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Pane Split Direction - Vertical",
                description: "Direction to split panes when splitting vertically.",
                field: Box::new(SettingField {
                    json_path: Some("pane_split_direction_vertical"),
                    pick: |settings_content| {
                        settings_content
                            .workspace
                            .pane_split_direction_vertical
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content.workspace.pane_split_direction_vertical = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Bottom Dock Layout",
                description: "How to layout the bottom dock.",
                field: Box::new(SettingField {
                    json_path: Some("bottom_dock_layout"),
                    pick: |settings_content| settings_content.workspace.bottom_dock_layout.as_ref(),
                    write: |settings_content, value| {
                        settings_content.workspace.bottom_dock_layout = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show Call Status Icon",
                description: "Whether to show the call status icon in the status bar.",
                field: Box::new(SettingField {
                    json_path: Some("show_call_status_icon"),
                    pick: |settings_content| {
                        settings_content.workspace.show_call_status_icon.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content.workspace.show_call_status_icon = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Status Bar - Show Cursor Position",
                description: "Whether to show the cursor position in the status bar.",
                field: Box::new(SettingField {
                    json_path: Some("status_bar.show_cursor_position"),
                    pick: |settings_content| {
                        settings_content
                            .status_bar
                            .as_ref()?
                            .show_cursor_position
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .status_bar
                            .get_or_insert_default()
                            .show_cursor_position = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Status Bar - Show Vim Mode",
                description: "Whether to show the Vim mode in the status bar.",
                field: Box::new(SettingField {
                    json_path: Some("status_bar.show_vim_mode"),
                    pick: |settings_content| {
                        settings_content
                            .status_bar
                            .as_ref()?
                            .show_vim_mode
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .status_bar
                            .get_or_insert_default()
                            .show_vim_mode = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    fn status_bar_section() -> [SettingsPageItem; 6] {
        [
            SettingsPageItem::SectionHeader("Status Bar"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show Status Bar",
                description: "Whether to show the status bar.",
                field: Box::new(SettingField {
                    json_path: Some("status_bar.show"),
                    pick: |settings_content| settings_content.status_bar.as_ref()?.show.as_ref(),
                    write: |settings_content, value| {
                        settings_content.status_bar.get_or_insert_default().show = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Chat Button",
                description: "Show the chat button in the status bar.",
                field: Box::new(SettingField {
                    json_path: Some("chat_panel.button"),
                    pick: |settings_content| {
                        settings_content.chat_panel.as_ref()?.button.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .chat_panel
                            .get_or_insert_default()
                            .button = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Conversation Button",
                description: "Show the conversation button in the status bar.",
                field: Box::new(SettingField {
                    json_path: Some("collaboration_panel.button"),
                    pick: |settings_content| {
                        settings_content
                            .collaboration_panel
                            .as_ref()?
                            .button
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .collaboration_panel
                            .get_or_insert_default()
                            .button = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Notification Button",
                description: "Show the notification button in the status bar.",
                field: Box::new(SettingField {
                    json_path: Some("notification_panel.button"),
                    pick: |settings_content| {
                        settings_content
                            .notification_panel
                            .as_ref()?
                            .button
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .notification_panel
                            .get_or_insert_default()
                            .button = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Search Button",
                description: "Show the search button in the status bar.",
                field: Box::new(SettingField {
                    json_path: Some("search.button"),
                    pick: |settings_content| {
                        settings_content.editor.search.as_ref()?.button.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .editor
                            .search
                            .get_or_insert_default()
                            .button = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Debugger Button",
                description: "Show the debugger button in the status bar.",
                field: Box::new(SettingField {
                    json_path: Some("debugger.button"),
                    pick: |settings_content| settings_content.debugger.as_ref()?.button.as_ref(),
                    write: |settings_content, value| {
                        settings_content.debugger.get_or_insert_default().button = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    fn title_bar_section() -> [SettingsPageItem; 9] {
        [
            SettingsPageItem::SectionHeader("Title Bar"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show Branch Icon",
                description: "Show the branch icon beside branch switcher in the titlebar.",
                field: Box::new(SettingField {
                    json_path: Some("title_bar.show_branch_icon"),
                    pick: |settings_content| {
                        settings_content
                            .title_bar
                            .as_ref()?
                            .show_branch_icon
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .title_bar
                            .get_or_insert_default()
                            .show_branch_icon = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show Branch Name",
                description: "Show the branch name button in the titlebar.",
                field: Box::new(SettingField {
                    json_path: Some("title_bar.show_branch_name"),
                    pick: |settings_content| {
                        settings_content
                            .title_bar
                            .as_ref()?
                            .show_branch_name
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .title_bar
                            .get_or_insert_default()
                            .show_branch_name = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show Project Items",
                description: "Show the project host and name in the titlebar.",
                field: Box::new(SettingField {
                    json_path: Some("title_bar.show_project_items"),
                    pick: |settings_content| {
                        settings_content
                            .title_bar
                            .as_ref()?
                            .show_project_items
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .title_bar
                            .get_or_insert_default()
                            .show_project_items = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show Onboarding Banner",
                description: "Show banners announcing new features in the titlebar.",
                field: Box::new(SettingField {
                    json_path: Some("title_bar.show_onboarding_banner"),
                    pick: |settings_content| {
                        settings_content
                            .title_bar
                            .as_ref()?
                            .show_onboarding_banner
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .title_bar
                            .get_or_insert_default()
                            .show_onboarding_banner = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show Sign In",
                description: "Show the sign in button in the titlebar.",
                field: Box::new(SettingField {
                    json_path: Some("title_bar.show_sign_in"),
                    pick: |settings_content| {
                        settings_content.title_bar.as_ref()?.show_sign_in.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .title_bar
                            .get_or_insert_default()
                            .show_sign_in = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show User Menu",
                description: "Show the user menu button in the titlebar.",
                field: Box::new(SettingField {
                    json_path: Some("title_bar.show_user_menu"),
                    pick: |settings_content| {
                        settings_content.title_bar.as_ref()?.show_user_menu.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .title_bar
                            .get_or_insert_default()
                            .show_user_menu = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show User Picture",
                description: "Show user picture in the titlebar.",
                field: Box::new(SettingField {
                    json_path: Some("title_bar.show_user_picture"),
                    pick: |settings_content| {
                        settings_content
                            .title_bar
                            .as_ref()?
                            .show_user_picture
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .title_bar
                            .get_or_insert_default()
                            .show_user_picture = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show Menus",
                description: "Show the menus in the titlebar.",
                field: Box::new(SettingField {
                    json_path: Some("title_bar.show_menus"),
                    pick: |settings_content| {
                        settings_content.title_bar.as_ref()?.show_menus.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .title_bar
                            .get_or_insert_default()
                            .show_menus = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    fn tab_bar_section() -> [SettingsPageItem; 9] {
        [
            SettingsPageItem::SectionHeader("Tab Bar"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show Tab Bar",
                description: "Show the tab bar in the editor.",
                field: Box::new(SettingField {
                    json_path: Some("tab_bar.show"),
                    pick: |settings_content| settings_content.tab_bar.as_ref()?.show.as_ref(),
                    write: |settings_content, value| {
                        settings_content.tab_bar.get_or_insert_default().show = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show Git Status In Tabs",
                description: "Show the Git file status on a tab item.",
                field: Box::new(SettingField {
                    json_path: Some("tabs.git_status"),
                    pick: |settings_content| settings_content.tabs.as_ref()?.git_status.as_ref(),
                    write: |settings_content, value| {
                        settings_content.tabs.get_or_insert_default().git_status = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show File Icons In Tabs",
                description: "Show the file icons on a tab item.",
                field: Box::new(SettingField {
                    json_path: Some("tabs.file_icons"),
                    pick: |settings_content| settings_content.tabs.as_ref()?.file_icons.as_ref(),
                    write: |settings_content, value| {
                        settings_content.tabs.get_or_insert_default().file_icons = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show Navigation History",
                description: "Show the navigation history buttons on the tab bar.",
                field: Box::new(SettingField {
                    json_path: Some("tab_bar.show_nav_history_buttons"),
                    pick: |settings_content| {
                        settings_content
                            .tab_bar
                            .as_ref()?
                            .show_nav_history_buttons
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .tab_bar
                            .get_or_insert_default()
                            .show_nav_history_buttons = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Always Show New Tab Button",
                description: "Always show the new tab button on the tab bar.",
                field: Box::new(SettingField {
                    json_path: Some("tabs.always_show_new_tab_button"),
                    pick: |settings_content| {
                        settings_content
                            .tabs
                            .as_ref()?
                            .always_show_new_tab_button
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .tabs
                            .get_or_insert_default()
                            .always_show_new_tab_button = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show Close Button",
                description: "Show the close button on the tab bar.",
                field: Box::new(SettingField {
                    json_path: Some("tabs.show_close_button"),
                    pick: |settings_content| {
                        settings_content.tabs.as_ref()?.show_close_button.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content.tabs.get_or_insert_default().show_close_button = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Activate On Close",
                description: "Which tab to activate when the current tab is closed.",
                field: Box::new(SettingField {
                    json_path: Some("tabs.activate_on_close"),
                    pick: |settings_content| {
                        settings_content.tabs.as_ref()?.activate_on_close.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content.tabs.get_or_insert_default().activate_on_close = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Close Position",
                description: "Where to show the close button on a tab.",
                field: Box::new(SettingField {
                    json_path: Some("tabs.close_position"),
                    pick: |settings_content| {
                        settings_content.tabs.as_ref()?.close_position.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content.tabs.get_or_insert_default().close_position = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Scrollbar",
                description: "When to show the scrollbar in the tab bar.",
                field: Box::new(SettingField {
                    json_path: Some("tab_bar.show_scrollbar"),
                    pick: |settings_content| {
                        show_scrollbar_or_editor(settings_content, |settings_content| {
                            settings_content
                                .tab_bar
                                .as_ref()?
                                .show_scrollbar
                                .as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        settings_content
                            .tab_bar
                            .get_or_insert_default()
                            .show_scrollbar = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    SettingsPage {
        title: "Window & Layout",
        items: concat_sections!(
            window_section(),
            layout_section(),
            status_bar_section(),
            title_bar_section(),
            tab_bar_section(),
        ),
    }
}
