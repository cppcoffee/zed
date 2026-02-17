use crate::{
    ActionLink, SettingField, SettingItem, SettingsPage, SettingsPageItem, USER,
    pages::open_audio_test_window,
};
use super::utils::{concat_sections, DEFAULT_EMPTY_AUDIO_OUTPUT, DEFAULT_EMPTY_AUDIO_INPUT};
use std::sync::Arc;
use ui::IntoElement;

pub fn collaboration_page() -> SettingsPage {
    fn calls_section() -> [SettingsPageItem; 3] {
        [
            SettingsPageItem::SectionHeader("Calls"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Mute On Join",
                description: "Whether the microphone should be muted when joining a channel or a call.",
                field: Box::new(SettingField {
                    json_path: Some("calls.mute_on_join"),
                    pick: |settings_content| settings_content.calls.as_ref()?.mute_on_join.as_ref(),
                    write: |settings_content, value| {
                        settings_content.calls.get_or_insert_default().mute_on_join = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Share On Join",
                description: "Whether your current project should be shared when joining an empty channel.",
                field: Box::new(SettingField {
                    json_path: Some("calls.share_on_join"),
                    pick: |settings_content| {
                        settings_content.calls.as_ref()?.share_on_join.as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content.calls.get_or_insert_default().share_on_join = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    fn experimental_section() -> [SettingsPageItem; 9] {
        [
            SettingsPageItem::SectionHeader("Experimental"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Rodio Audio",
                description: "Opt into the new audio system.",
                field: Box::new(SettingField {
                    json_path: Some("audio.experimental.rodio_audio"),
                    pick: |settings_content| settings_content.audio.as_ref()?.rodio_audio.as_ref(),
                    write: |settings_content, value| {
                        settings_content.audio.get_or_insert_default().rodio_audio = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Auto Microphone Volume",
                description: "Automatically adjust microphone volume (requires rodio audio).",
                field: Box::new(SettingField {
                    json_path: Some("audio.experimental.auto_microphone_volume"),
                    pick: |settings_content| {
                        settings_content
                            .audio
                            .as_ref()?
                            .auto_microphone_volume
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .audio
                            .get_or_insert_default()
                            .auto_microphone_volume = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Auto Speaker Volume",
                description: "Automatically adjust volume of other call members (requires rodio audio).",
                field: Box::new(SettingField {
                    json_path: Some("audio.experimental.auto_speaker_volume"),
                    pick: |settings_content| {
                        settings_content
                            .audio
                            .as_ref()?
                            .auto_speaker_volume
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .audio
                            .get_or_insert_default()
                            .auto_speaker_volume = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Denoise",
                description: "Remove background noises (requires rodio audio).",
                field: Box::new(SettingField {
                    json_path: Some("audio.experimental.denoise"),
                    pick: |settings_content| settings_content.audio.as_ref()?.denoise.as_ref(),
                    write: |settings_content, value| {
                        settings_content.audio.get_or_insert_default().denoise = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Legacy Audio Compatible",
                description: "Use audio parameters compatible with previous versions (requires rodio audio).",
                field: Box::new(SettingField {
                    json_path: Some("audio.experimental.legacy_audio_compatible"),
                    pick: |settings_content| {
                        settings_content
                            .audio
                            .as_ref()?
                            .legacy_audio_compatible
                            .as_ref()
                    },
                    write: |settings_content, value| {
                        settings_content
                            .audio
                            .get_or_insert_default()
                            .legacy_audio_compatible = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::ActionLink(ActionLink {
                title: "Test Audio".into(),
                description: Some("Test your microphone and speaker setup".into()),
                button_text: "Test Audio".into(),
                on_click: Arc::new(|_settings_window, window, cx| {
                    open_audio_test_window(window, cx);
                }),
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Output Audio Device",
                description: "Select output audio device",
                field: Box::new(SettingField {
                    json_path: Some("audio.experimental.output_audio_device"),
                    pick: |settings_content| {
                        settings_content
                            .audio
                            .as_ref()?
                            .output_audio_device
                            .as_ref()
                            .or(DEFAULT_EMPTY_AUDIO_OUTPUT)
                    },
                    write: |settings_content, value| {
                        settings_content
                            .audio
                            .get_or_insert_default()
                            .output_audio_device = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Input Audio Device",
                description: "Select input audio device",
                field: Box::new(SettingField {
                    json_path: Some("audio.experimental.input_audio_device"),
                    pick: |settings_content| {
                        settings_content
                            .audio
                            .as_ref()?
                            .input_audio_device
                            .as_ref()
                            .or(DEFAULT_EMPTY_AUDIO_INPUT)
                    },
                    write: |settings_content, value| {
                        settings_content
                            .audio
                            .get_or_insert_default()
                            .input_audio_device = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]
    }

    SettingsPage {
        title: "Collaboration",
        items: concat_sections![calls_section(), experimental_section()],
    }
}
