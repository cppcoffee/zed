use crate::{
    SettingField, SettingItem, SettingsPage, SettingsPageItem, SubPageLink, USER, PROJECT,
    pages::render_tool_permissions_setup_page,
    pages::render_edit_prediction_setup_page,
};
use super::utils::concat_sections;
use super::languages_and_tools::edit_prediction_language_settings_section;

pub fn ai_page() -> SettingsPage {
    fn general_section() -> [SettingsPageItem; 2] {
        [
            SettingsPageItem::SectionHeader("General"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Disable AI",
                description: "Whether to disable all AI features in Zed.",
                field: Box::new(SettingField {
                    json_path: Some("disable_ai"),
                    pick: |settings_content| settings_content.project.disable_ai.as_ref(),
                    write: |settings_content, value| {
                        settings_content.project.disable_ai = value;
                    },
                }),
                metadata: None,
                files: USER | PROJECT,
            }),
        ]
    }

    fn agent_configuration_section() -> [SettingsPageItem; 12] {
        [
            SettingsPageItem::SectionHeader("Agent Configuration"),
            SettingsPageItem::SubPageLink(SubPageLink {
                title: "Tool Permissions".into(),
                r#type: Default::default(),
                json_path: Some("agent.tool_permissions"),
                description: Some("Set up regex patterns to auto-allow, auto-deny, or always request confirmation, for specific tool inputs.".into()),
                in_json: true,
                files: USER,
                render: render_tool_permissions_setup_page,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Single File Review",
                description: "When enabled, agent edits will also be displayed in single-file buffers for review.",
                field: Box::new(SettingField {
                    json_path: Some("agent.single_file_review"),
                    pick: |settings_content| {
                        settings_content.agent.as_ref()?.single_file_review.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .agent
                            .get_or_insert_default()
                            .single_file_review = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Enable Feedback",
                description: "Show voting thumbs up/down icon buttons for feedback on agent edits.",
                field: Box::new(SettingField {
                    json_path: Some("agent.enable_feedback"),
                    pick: |settings_content| {
                        settings_content.agent.as_ref()?.enable_feedback.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .agent
                            .get_or_insert_default()
                            .enable_feedback = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Notify When Agent Waiting",
                description: "Where to show notifications when the agent has completed its response or needs confirmation before running a tool action.",
                field: Box::new(SettingField {
                    json_path: Some("agent.notify_when_agent_waiting"),
                    pick: |settings_content| {
                        settings_content
                            .agent
                            .as_ref()?
                            .notify_when_agent_waiting
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .agent
                            .get_or_insert_default()
                            .notify_when_agent_waiting = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Play Sound When Agent Done",
                description: "Whether to play a sound when the agent has either completed its response, or needs user input.",
                field: Box::new(SettingField {
                    json_path: Some("agent.play_sound_when_agent_done"),
                    pick: |settings_content| {
                        settings_content
                            .agent
                            .as_ref()?
                            .play_sound_when_agent_done
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .agent
                            .get_or_insert_default()
                            .play_sound_when_agent_done = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Expand Edit Card",
                description: "Whether to have edit cards in the agent panel expanded, showing a Preview of the diff.",
                field: Box::new(SettingField {
                    json_path: Some("agent.expand_edit_card"),
                    pick: |settings_content| {
                        settings_content.agent.as_ref()?.expand_edit_card.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .agent
                            .get_or_insert_default()
                            .expand_edit_card = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Expand Terminal Card",
                description: "Whether to have terminal cards in the agent panel expanded, showing the whole command output.",
                field: Box::new(SettingField {
                    json_path: Some("agent.expand_terminal_card"),
                    pick: |settings_content| {
                        settings_content
                            .agent
                            .as_ref()?
                            .expand_terminal_card
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .agent
                            .get_or_insert_default()
                            .expand_terminal_card = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Cancel Generation On Terminal Stop",
                description: "Whether clicking the stop button on a running terminal tool should also cancel the agent's generation. Note that this only applies to the stop button, not to ctrl+c inside the terminal.",
                field: Box::new(SettingField {
                    json_path: Some("agent.cancel_generation_on_terminal_stop"),
                    pick: |settings_content| {
                        settings_content
                            .agent
                            .as_ref()?
                            .cancel_generation_on_terminal_stop
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .agent
                            .get_or_insert_default()
                            .cancel_generation_on_terminal_stop = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Use Modifier To Send",
                description: "Whether to always use cmd-enter (or ctrl-enter on Linux or Windows) to send messages.",
                field: Box::new(SettingField {
                    json_path: Some("agent.use_modifier_to_send"),
                    pick: |settings_content| {
                        settings_content
                            .agent
                            .as_ref()?
                            .use_modifier_to_send
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .agent
                            .get_or_insert_default()
                            .use_modifier_to_send = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Message Editor Min Lines",
                description: "Minimum number of lines to display in the agent message editor.",
                field: Box::new(SettingField {
                    json_path: Some("agent.message_editor_min_lines"),
                    pick: |settings_content| {
                        settings_content
                            .agent
                            .as_ref()?
                            .message_editor_min_lines
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .agent
                            .get_or_insert_default()
                            .message_editor_min_lines = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Show Turn Stats",
                description: "Whether to show turn statistics like elapsed time during generation and final turn duration.",
                field: Box::new(SettingField {
                    json_path: Some("agent.show_turn_stats"),
                    pick: |settings_content| {
                        settings_content.agent.as_ref()?.show_turn_stats.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .agent
                            .get_or_insert_default()
                            .show_turn_stats = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    fn context_servers_section() -> [SettingsPageItem; 2] {
        [
            SettingsPageItem::SectionHeader("Context Servers"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Context Server Timeout",
                description: "Default timeout in seconds for context server tool calls. Can be overridden per-server in context_servers configuration.",
                field: Box::new(SettingField {
                    json_path: Some("context_server_timeout"),
                    pick: |settings_content| {
                        settings_content.project.context_server_timeout.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content.project.context_server_timeout = value;
                    },
                }),
                metadata: None,
                files: USER | PROJECT,
            }),
        ]
    }

    fn edit_prediction_display_sub_section() -> [SettingsPageItem; 2] {
        [
            SettingsPageItem::SettingItem(SettingItem {
                title: "Display Mode",
                description: "When to show edit predictions previews in buffer. The eager mode displays them inline, while the subtle mode displays them only when holding a modifier key.",
                field: Box::new(SettingField {
                    json_path: Some("edit_prediction.display_mode"),
                    pick: |settings_content| {
                        settings_content
                            .project
                            .all_languages
                            .edit_predictions
                            .as_ref()?
                            .mode
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project
                            .all_languages
                            .edit_predictions
                            .get_or_insert_default()
                            .mode = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Display In Text Threads",
                description: "Whether edit predictions are enabled when editing text threads in the agent panel.",
                field: Box::new(SettingField {
                    json_path: Some("edit_prediction.in_text_threads"),
                    pick: |settings_content| {
                        settings_content
                            .project
                            .all_languages
                            .edit_predictions
                            .as_ref()?
                            .enabled_in_text_threads
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .project
                            .all_languages
                            .edit_predictions
                            .get_or_insert_default()
                            .enabled_in_text_threads = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    SettingsPage {
        title: "AI",
        items: concat_sections![
            general_section(),
            agent_configuration_section(),
            context_servers_section(),
            edit_prediction_language_settings_section(),
            edit_prediction_display_sub_section()
        ],
    }
}
