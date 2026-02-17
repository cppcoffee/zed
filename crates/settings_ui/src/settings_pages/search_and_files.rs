use crate::{
    SettingField, SettingItem, SettingsPage, SettingsPageItem, USER,
};
use super::utils::concat_sections;

pub fn search_and_files_page() -> SettingsPage {
    fn search_section() -> [SettingsPageItem; 6] {
        [
            SettingsPageItem::SectionHeader("Search"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Whole Word",
                description: "Whether to match whole words only.",
                field: Box::new(SettingField {
                    json_path: Some("search.whole_word"),
                    pick: |settings_content| {
                        settings_content.editor.search.as_ref()?.whole_word.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .editor
                            .search
                            .get_or_insert_default()
                            .whole_word = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Case Sensitive",
                description: "Whether to match case sensitively.",
                field: Box::new(SettingField {
                    json_path: Some("search.case_sensitive"),
                    pick: |settings_content| {
                        settings_content.editor.search.as_ref()?.case_sensitive.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .editor
                            .search
                            .get_or_insert_default()
                            .case_sensitive = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Include Ignored",
                description: "Whether to include ignored files.",
                field: Box::new(SettingField {
                    json_path: Some("search.include_ignored"),
                    pick: |settings_content| {
                        settings_content.editor.search.as_ref()?.include_ignored.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .editor
                            .search
                            .get_or_insert_default()
                            .include_ignored = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Use Regex",
                description: "Whether to use regular expressions.",
                field: Box::new(SettingField {
                    json_path: Some("search.regex"),
                    pick: |settings_content| settings_content.editor.search.as_ref()?.regex.as_ref(),
                    write: |settings_content, value| {
                        settings_content
                            .editor
                            .search
                            .get_or_insert_default()
                            .regex = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Center On Match",
                description: "Whether to auto-scroll the match to the center of the screen.",
                field: Box::new(SettingField {
                    json_path: Some("search.center_on_match"),
                    pick: |settings_content| {
                        settings_content.editor.search.as_ref()?.center_on_match.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .editor
                            .search
                            .get_or_insert_default()
                            .center_on_match = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Seed Search Query From Cursor",
                description: "When to populate a new search's query based on the text under the cursor.",
                field: Box::new(SettingField {
                    json_path: Some("seed_search_query_from_cursor"),
                    pick: |settings_content| {
                        settings_content
                            .editor
                            .seed_search_query_from_cursor
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content.editor.seed_search_query_from_cursor = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    fn file_finder_section() -> [SettingsPageItem; 6] {
        [
            SettingsPageItem::SectionHeader("File Finder"),
            // todo: null by default
            SettingsPageItem::SettingItem(SettingItem {
                title: "Include Ignored in Search",
                description: "Use gitignored files when searching.",
                field: Box::new(SettingField {
                    json_path: Some("file_finder.include_ignored"),
                    pick: |settings_content| {
                        settings_content
                            .file_finder
                            .as_ref()?
                            .include_ignored
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .file_finder
                            .get_or_insert_default()
                            .include_ignored = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "File Icons",
                description: "Show file icons in the file finder.",
                field: Box::new(SettingField {
                    json_path: Some("file_finder.file_icons"),
                    pick: |settings_content| {
                        settings_content.file_finder.as_ref()?.file_icons.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .file_finder
                            .get_or_insert_default()
                            .file_icons = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Modal Max Width",
                description: "Determines how much space the file finder can take up in relation to the available window width.",
                field: Box::new(SettingField {
                    json_path: Some("file_finder.modal_max_width"),
                    pick: |settings_content| {
                        settings_content
                            .file_finder
                            .as_ref()?
                            .modal_max_width
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .file_finder
                            .get_or_insert_default()
                            .modal_max_width = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Skip Focus For Active In Search",
                description: "Whether the file finder should skip focus for the active file in search results.",
                field: Box::new(SettingField {
                    json_path: Some("file_finder.skip_focus_for_active_in_search"),
                    pick: |settings_content| {
                        settings_content
                            .file_finder
                            .as_ref()?
                            .skip_focus_for_active_in_search
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .file_finder
                            .get_or_insert_default()
                            .skip_focus_for_active_in_search = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Git Status",
                description: "Show the Git status in the file finder.",
                field: Box::new(SettingField {
                    json_path: Some("file_finder.git_status"),
                    pick: |settings_content| {
                        settings_content.file_finder.as_ref()?.git_status.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .file_finder
                            .get_or_insert_default()
                            .git_status = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    fn file_scan_section() -> [SettingsPageItem; 5] {
        [
            SettingsPageItem::SectionHeader("File Scan"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "File Scan Exclusions",
                description: "Files or globs of files that will be excluded by Zed entirely. They will be skipped during file scans, file searches, and not be displayed in the project file tree. Takes precedence over \"File Scan Inclusions\"",
                field: Box::new(
                    SettingField {
                        json_path: Some("file_scan_exclusions"),
                        pick: |settings_content| {
                            settings_content
                                .project
                                .worktree
                                .file_scan_exclusions
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.project.worktree.file_scan_exclusions = value;
                        },
                    }
                    .unimplemented(),
                ),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "File Scan Inclusions",
                description: "Files or globs of files that will be included by Zed, even when ignored by git. This is useful for files that are not tracked by git, but are still important to your project. Note that globs that are overly broad can slow down Zed's file scanning. \"File Scan Exclusions\" takes precedence over these inclusions",
                field: Box::new(
                    SettingField {
                        json_path: Some("file_scan_inclusions"),
                        pick: |settings_content| {
                            settings_content
                                .project
                                .worktree
                                .file_scan_inclusions
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.project.worktree.file_scan_inclusions = value;
                        },
                    }
                    .unimplemented(),
                ),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Restore File State",
                description: "Restore previous file state when reopening.",
                field: Box::new(SettingField {
                    json_path: Some("restore_on_file_reopen"),
                    pick: |settings_content| {
                        settings_content.workspace.restore_on_file_reopen.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content.workspace.restore_on_file_reopen = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Close on File Delete",
                description: "Automatically close files that have been deleted.",
                field: Box::new(SettingField {
                    json_path: Some("close_on_file_delete"),
                    pick: |settings_content| {
                        settings_content.workspace.close_on_file_delete.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content.workspace.close_on_file_delete = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    SettingsPage {
        title: "Search & Files",
        items: concat_sections![search_section(), file_finder_section(), file_scan_section()],
    }
}
