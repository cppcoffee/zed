use crate::{
    SettingField, SettingItem, SettingsPage, SettingsPageItem, USER,
};
use super::utils::{show_scrollbar_or_editor};

pub fn panels_page() -> SettingsPage {
    fn project_panel_section() -> [SettingsPageItem; 21] {
        [
            SettingsPageItem::SectionHeader("Project Panel"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Project Panel Dock",
                description: "Where to dock the project panel.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.dock"),
                    pick: |settings_content| settings_content.project_panel.as_ref()?.dock.as_ref(),
                    write: |settings_content, value| {
                        settings_content.project_panel.get_or_insert_default().dock = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Project Panel Default Width",
                description: "Default width of the project panel in pixels.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.default_width"),
                    pick: |settings_content| {
                        settings_content
                            .project_panel
                            .as_ref()?
                            .default_width
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .default_width = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Hide .gitignore",
                description: "Whether to hide the gitignore entries in the project panel.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.hide_gitignore"),
                    pick: |settings_content| {
                        settings_content
                            .project_panel
                            .as_ref()?
                            .hide_gitignore
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .hide_gitignore = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Entry Spacing",
                description: "Spacing between worktree entries in the project panel.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.entry_spacing"),
                    pick: |settings_content| {
                        settings_content
                            .project_panel
                            .as_ref()?
                            .entry_spacing
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .entry_spacing = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "File Icons",
                description: "Show file icons in the project panel.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.file_icons"),
                    pick: |settings_content| {
                        settings_content.project_panel.as_ref()?.file_icons.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .file_icons = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Git Status",
                description: "Show the Git status in the project panel.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.git_status"),
                    pick: |settings_content| {
                        settings_content.project_panel.as_ref()?.git_status.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .git_status = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Indent Size",
                description: "Amount of indentation for nested items.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.indent_size"),
                    pick: |settings_content| {
                        settings_content
                            .project_panel
                            .as_ref()?
                            .indent_size
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .indent_size = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Auto Reveal Entries",
                description: "Whether to reveal entries in the project panel automatically when a corresponding project entry becomes active.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.auto_reveal_entries"),
                    pick: |settings_content| {
                        settings_content
                            .project_panel
                            .as_ref()?
                            .auto_reveal_entries
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .auto_reveal_entries = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Starts Open",
                description: "Whether the project panel should open on startup.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.starts_open"),
                    pick: |settings_content| {
                        settings_content
                            .project_panel
                            .as_ref()?
                            .starts_open
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .starts_open = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Auto Fold Directories",
                description: "Whether to fold directories automatically and show compact folders when a directory has only one subdirectory inside.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.auto_fold_dirs"),
                    pick: |settings_content| {
                        settings_content
                            .project_panel
                            .as_ref()?
                            .auto_fold_dirs
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .auto_fold_dirs = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Bold Folder Labels",
                description: "Whether to show folder names with bold text in the project panel.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.bold_folder_labels"),
                    pick: |settings_content| {
                        settings_content
                            .project_panel
                            .as_ref()?
                            .bold_folder_labels
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .bold_folder_labels = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show Scrollbar",
                description: "Show the scrollbar in the project panel.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.scrollbar.show"),
                    pick: |settings_content| {
                        show_scrollbar_or_editor(settings_content, |settings_content| {
                            settings_content
                                .project_panel
                                .as_ref()?
                                .scrollbar
                                .as_ref()?
                                .show
                                .as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .scrollbar
                            .get_or_insert_default()
                            .show = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show Diagnostics",
                description: "Which files containing diagnostic errors/warnings to mark in the project panel.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.show_diagnostics"),
                    pick: |settings_content| {
                        settings_content
                            .project_panel
                            .as_ref()?
                            .show_diagnostics
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .show_diagnostics = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Sticky Scroll",
                description: "Whether to stick parent directories at top of the project panel.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.sticky_scroll"),
                    pick: |settings_content| {
                        settings_content
                            .project_panel
                            .as_ref()?
                            .sticky_scroll
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .sticky_scroll = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                files: USER,
                title: "Show Indent Guides",
                description: "Show indent guides in the project panel.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.indent_guides.show"),
                    pick: |settings_content| {
                        settings_content
                            .project_panel
                            .as_ref()?
                            .indent_guides
                            .as_ref()?
                            .show
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .indent_guides
                            .get_or_insert_default()
                            .show = value;
                    },
                }),
                metadata: None,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                files: USER,
                title: "Indent Guides Line Width",
                description: "The width of the indent guides in pixels, between 1 and 10.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.indent_guides.line_width"),
                    pick: |settings_content| {
                        settings_content
                            .project_panel
                            .as_ref()?
                            .indent_guides
                            .as_ref()?
                            .line_width
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .indent_guides
                            .get_or_insert_default()
                            .line_width = value;
                    },
                }),
                metadata: None,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                files: USER,
                title: "Indent Guides Active Line Width",
                description: "The width of the active indent guide in pixels, between 1 and 10.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.indent_guides.active_line_width"),
                    pick: |settings_content| {
                        settings_content
                            .project_panel
                            .as_ref()?
                            .indent_guides
                            .as_ref()?
                            .active_line_width
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .indent_guides
                            .get_or_insert_default()
                            .active_line_width = value;
                    },
                }),
                metadata: None,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                files: USER,
                title: "Indent Guides Coloring",
                description: "Determines how indent guides are colored.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.indent_guides.coloring"),
                    pick: |settings_content| {
                        settings_content
                            .project_panel
                            .as_ref()?
                            .indent_guides
                            .as_ref()?
                            .coloring
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .indent_guides
                            .get_or_insert_default()
                            .coloring = value;
                    },
                }),
                metadata: None,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                files: USER,
                title: "Indent Guides Background Coloring",
                description: "Determines how indent guide backgrounds are colored.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.indent_guides.background_coloring"),
                    pick: |settings_content| {
                        settings_content
                            .project_panel
                            .as_ref()?
                            .indent_guides
                            .as_ref()?
                            .background_coloring
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .indent_guides
                            .get_or_insert_default()
                            .background_coloring = value;
                    },
                }),
                metadata: None,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Sort Mode",
                description: "How to sort items in the project panel.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.sort_mode"),
                    pick: |settings_content| {
                        settings_content
                            .project_panel
                            .as_ref()?
                            .sort_mode
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .sort_mode = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Reveal on Hover",
                description: "Whether to reveal the item under cursor in the project panel.",
                field: Box::new(SettingField {
                    json_path: Some("project_panel.reveal_on_hover"),
                    pick: |settings_content| {
                        settings_content
                            .project_panel
                            .as_ref()?
                            .reveal_on_hover
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project_panel
                            .get_or_insert_default()
                            .reveal_on_hover = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    fn outline_panel_section() -> [SettingsPageItem; 9] {
        [
            SettingsPageItem::SectionHeader("Outline Panel"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Outline Panel Dock",
                description: "Where to dock the outline panel.",
                field: Box::new(SettingField {
                    json_path: Some("outline_panel.dock"),
                    pick: |settings_content| settings_content.outline_panel.as_ref()?.dock.as_ref(),
                    write: |settings_content, value| {
                        settings_content.outline_panel.get_or_insert_default().dock = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Outline Panel Default Width",
                description: "Default width of the outline panel in pixels.",
                field: Box::new(SettingField {
                    json_path: Some("outline_panel.default_width"),
                    pick: |settings_content| {
                        settings_content
                            .outline_panel
                            .as_ref()?
                            .default_width
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .outline_panel
                            .get_or_insert_default()
                            .default_width = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Indent Size",
                description: "Amount of indentation for nested items.",
                field: Box::new(SettingField {
                    json_path: Some("outline_panel.indent_size"),
                    pick: |settings_content| {
                        settings_content
                            .outline_panel
                            .as_ref()?
                            .indent_size
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .outline_panel
                            .get_or_insert_default()
                            .indent_size = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Auto Reveal Entries",
                description: "Whether to reveal entries in the outline panel automatically when a corresponding editor entry becomes active.",
                field: Box::new(SettingField {
                    json_path: Some("outline_panel.auto_reveal_entries"),
                    pick: |settings_content| {
                        settings_content
                            .outline_panel
                            .as_ref()?
                            .auto_reveal_entries
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .outline_panel
                            .get_or_insert_default()
                            .auto_reveal_entries = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Auto Fold Dirs",
                description: "Whether to fold directories automatically and show compact folders when a directory has only one subdirectory inside.",
                field: Box::new(SettingField {
                    json_path: Some("outline_panel.auto_fold_dirs"),
                    pick: |settings_content| {
                        settings_content
                            .outline_panel
                            .as_ref()?
                            .auto_fold_dirs
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .outline_panel
                            .get_or_insert_default()
                            .auto_fold_dirs = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                files: USER,
                title: "Show Indent Guides",
                description: "Show indent guides in the outline panel.",
                field: Box::new(SettingField {
                    json_path: Some("outline_panel.indent_guides.show"),
                    pick: |settings_content| {
                        settings_content
                            .outline_panel
                            .as_ref()?
                            .indent_guides
                            .as_ref()?
                            .show
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .outline_panel
                            .get_or_insert_default()
                            .indent_guides
                            .get_or_insert_default()
                            .show = value;
                    },
                }),
                metadata: None,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                files: USER,
                title: "Indent Guides Line Width",
                description: "The width of the indent guides in pixels, between 1 and 10.",
                field: Box::new(SettingField {
                    json_path: Some("outline_panel.indent_guides.line_width"),
                    pick: |settings_content| {
                        settings_content
                            .outline_panel
                            .as_ref()?
                            .indent_guides
                            .as_ref()?
                            .line_width
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .outline_panel
                            .get_or_insert_default()
                            .indent_guides
                            .get_or_insert_default()
                            .line_width = value;
                    },
                }),
                metadata: None,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                files: USER,
                title: "Indent Guides Active Line Width",
                description: "The width of the active indent guide in pixels, between 1 and 10.",
                field: Box::new(SettingField {
                    json_path: Some("outline_panel.indent_guides.active_line_width"),
                    pick: |settings_content| {
                        settings_content
                            .outline_panel
                            .as_ref()?
                            .indent_guides
                            .as_ref()?
                            .active_line_width
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .outline_panel
                            .get_or_insert_default()
                            .indent_guides
                            .get_or_insert_default()
                            .active_line_width = value;
                    },
                }),
                metadata: None,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                files: USER,
                title: "Indent Guides Coloring",
                description: "Determines how indent guides are colored.",
                field: Box::new(SettingField {
                    json_path: Some("outline_panel.indent_guides.coloring"),
                    pick: |settings_content| {
                        settings_content
                            .outline_panel
                            .as_ref()?
                            .indent_guides
                            .as_ref()?
                            .coloring
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .outline_panel
                            .get_or_insert_default()
                            .indent_guides
                            .get_or_insert_default()
                            .coloring = value;
                    },
                }),
                metadata: None,
            }),
        ]
    }

    fn collaboration_panel_section() -> [SettingsPageItem; 2] {
        [
            SettingsPageItem::SectionHeader("Collaboration Panel"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Collaboration Panel Dock",
                description: "Where to dock the collaboration panel.",
                field: Box::new(SettingField {
                    json_path: Some("collaboration_panel.dock"),
                    pick: |settings_content| {
                        settings_content.collaboration_panel.as_ref()?.dock.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .collaboration_panel
                            .get_or_insert_default()
                            .dock = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Collaboration Panel Default Width",
                description: "Default width of the collaboration panel in pixels.",
                field: Box::new(SettingField {
                    json_path: Some("collaboration_panel.default_width"),
                    pick: |settings_content| {
                        settings_content
                            .collaboration_panel
                            .as_ref()?
                            .default_width
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .collaboration_panel
                            .get_or_insert_default()
                            .default_width = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    fn chat_panel_section() -> [SettingsPageItem; 2] {
        [
            SettingsPageItem::SectionHeader("Chat Panel"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Chat Panel Dock",
                description: "Where to dock the chat panel.",
                field: Box::new(SettingField {
                    json_path: Some("chat_panel.dock"),
                    pick: |settings_content| settings_content.chat_panel.as_ref()?.dock.as_ref(),
                    write: |settings_content, value| {
                        settings_content.chat_panel.get_or_insert_default().dock = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Chat Panel Default Width",
                description: "Default width of the chat panel in pixels.",
                field: Box::new(SettingField {
                    json_path: Some("chat_panel.default_width"),
                    pick: |settings_content| {
                        settings_content
                            .chat_panel
                            .as_ref()?
                            .default_width
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content.chat_panel.get_or_insert_default().default_width = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    fn message_editor_section() -> [SettingsPageItem; 2] {
        [
            SettingsPageItem::SectionHeader("Message Editor"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Auto Replace Emoji Shortcode",
                description: "Whether to automatically replace emoji shortcodes with emoji characters.",
                field: Box::new(SettingField {
                    json_path: Some("message_editor.auto_replace_emoji_shortcode"),
                    pick: |settings_content| {
                        settings_content
                            .message_editor
                            .as_ref()?
                            .auto_replace_emoji_shortcode
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .message_editor
                            .get_or_insert_default()
                            .auto_replace_emoji_shortcode = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show Scrollbar",
                description: "When to show the scrollbar in the message editor.",
                field: Box::new(SettingField {
                    json_path: Some("message_editor.scrollbar.show"),
                    pick: |settings_content| {
                        show_scrollbar_or_editor(settings_content, |settings_content| {
                            settings_content
                                .message_editor
                                .as_ref()?
                                .scrollbar
                                .as_ref()?
                                .show
                                .as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        settings_content
                            .message_editor
                            .get_or_insert_default()
                            .scrollbar
                            .get_or_insert_default()
                            .show = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    fn notification_panel_section() -> [SettingsPageItem; 2] {
        [
            SettingsPageItem::SectionHeader("Notification Panel"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Notification Panel Dock",
                description: "Where to dock the notification panel.",
                field: Box::new(SettingField {
                    json_path: Some("notification_panel.dock"),
                    pick: |settings_content| {
                        settings_content
                            .notification_panel
                            .as_ref()?
                            .dock
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .notification_panel
                            .get_or_insert_default()
                            .dock = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Notification Panel Default Width",
                description: "Default width of the notification panel in pixels.",
                field: Box::new(SettingField {
                    json_path: Some("notification_panel.default_width"),
                    pick: |settings_content| {
                        settings_content
                            .notification_panel
                            .as_ref()?
                            .default_width
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .notification_panel
                            .get_or_insert_default()
                            .default_width = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    fn terminal_panel_section() -> [SettingsPageItem; 2] {
        [
            SettingsPageItem::SectionHeader("Terminal Panel"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Terminal Panel Dock",
                description: "Where to dock the terminal panel.",
                field: Box::new(SettingField {
                    json_path: Some("terminal.dock"),
                    pick: |settings_content| settings_content.terminal.as_ref()?.dock.as_ref(),
                    write: |settings_content, value| {
                        settings_content.terminal.get_or_insert_default().dock = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Terminal Panel Default Width",
                description: "Default width of the terminal panel in pixels.",
                field: Box::new(SettingField {
                    json_path: Some("terminal.default_width"),
                    pick: |settings_content| {
                        settings_content.terminal.as_ref()?.default_width.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content.terminal.get_or_insert_default().default_width = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    fn assistant_panel_section() -> [SettingsPageItem; 3] {
        [
            SettingsPageItem::SectionHeader("Assistant Panel"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Assistant Panel Dock",
                description: "Where to dock the assistant panel.",
                field: Box::new(SettingField {
                    json_path: Some("assistant.dock"),
                    pick: |settings_content| settings_content.assistant.as_ref()?.dock.as_ref(),
                    write: |settings_content, value| {
                        settings_content.assistant.get_or_insert_default().dock = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Assistant Panel Default Width",
                description: "Default width of the assistant panel in pixels.",
                field: Box::new(SettingField {
                    json_path: Some("assistant.default_width"),
                    pick: |settings_content| {
                        settings_content.assistant.as_ref()?.default_width.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .assistant
                            .get_or_insert_default()
                            .default_width = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Default Model",
                description: "The default model to use for the assistant.",
                field: Box::new(SettingField {
                    json_path: Some("assistant.default_model"),
                    pick: |settings_content| {
                        settings_content.assistant.as_ref()?.default_model.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .assistant
                            .get_or_insert_default()
                            .default_model = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    use super::utils::concat_sections;

    SettingsPage {
        title: "Panels",
        items: concat_sections!(
            project_panel_section(),
            outline_panel_section(),
            collaboration_panel_section(),
            chat_panel_section(),
            message_editor_section(),
            notification_panel_section(),
            terminal_panel_section(),
            assistant_panel_section(),
        ),
    }
}
