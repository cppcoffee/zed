use gpui::App;
use crate::SettingsPage;
use crate::settings_pages::{
    general::general_page,
    appearance::appearance_page,
    keymap::keymap_page,
    editor::editor_page,
    languages_and_tools::languages_and_tools_page,
    search_and_files::search_and_files_page,
    window_and_layout::window_and_layout_page,
    panels::panels_page,
    debugger::debugger_page,
    terminal::terminal_page,
    version_control::version_control_page,
    collaboration::collaboration_page,
    ai::ai_page,
    network::network_page,
};

pub(crate) fn settings_data(cx: &App) -> Vec<SettingsPage> {
    vec![
        general_page(),
        appearance_page(),
        keymap_page(),
        editor_page(),
        languages_and_tools_page(cx),
        search_and_files_page(),
        window_and_layout_page(),
        panels_page(),
        debugger_page(),
        terminal_page(),
        version_control_page(),
        collaboration_page(),
        ai_page(),
        network_page(),
    ]
}
