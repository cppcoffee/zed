use crate::init_test;
use fs::FakeFs;
use gpui::TestAppContext;
use project::DisableAiSettings;
use settings::{LocalSettingsKind, LocalSettingsPath, SettingsStore};
use util::rel_path::RelPath;
use std::path::Path;
use std::sync::Arc;

#[gpui::test]
async fn test_is_ai_disabled_for_file(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        Path::new("/root"),
        serde_json::json!({
            "file.rs": "fn main() {}",
        }),
    )
    .await;

    let project = project::Project::test(fs.clone(), ["/root".as_ref()], cx).await;
    let worktree_id = project.update(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/root/file.rs", cx)
        })
        .await
        .unwrap();

    let file = buffer.read_with(cx, |buffer, _| buffer.file().cloned()).unwrap();

    // 1. Default: Global false, Project default (none). Result: false.
    cx.update(|cx| {
        assert!(
            !DisableAiSettings::is_ai_disabled_for_file(Some(&file), cx),
            "Default should be false"
        );
    });

    // 2. Global true. Result: true.
    cx.update_global::<SettingsStore, _>(|store, cx| {
        store.set_user_settings(r#"{ "disable_ai": true }"#, cx).unwrap();
    });
    cx.update(|cx| {
        assert!(
            DisableAiSettings::is_ai_disabled_for_file(Some(&file), cx),
            "Global true should disable AI"
        );
    });

    // Reset global to false
    cx.update_global::<SettingsStore, _>(|store, cx| {
        store.set_user_settings(r#"{ "disable_ai": false }"#, cx).unwrap();
    });

    // 3. Project true. Result: true.
    // We use the root path to simulate project-level settings.
    let root_path = RelPath::new(Path::new(""), util::paths::PathStyle::Posix).unwrap();
    let root_arc_path: Arc<RelPath> = Arc::from(root_path.as_ref());

    cx.update_global::<SettingsStore, _>(|store, cx| {
        store
            .set_local_settings(
                worktree_id,
                LocalSettingsPath::InWorktree(root_arc_path.clone()),
                LocalSettingsKind::Settings,
                Some(r#"{ "disable_ai": true }"#),
                cx,
            )
            .unwrap();
    });

    cx.update(|cx| {
        assert!(
            DisableAiSettings::is_ai_disabled_for_file(Some(&file), cx),
            "Project true should disable AI"
        );
    });

    // 4. Project false. Result: false.
    cx.update_global::<SettingsStore, _>(|store, cx| {
        store
            .set_local_settings(
                worktree_id,
                LocalSettingsPath::InWorktree(root_arc_path.clone()),
                LocalSettingsKind::Settings,
                Some(r#"{ "disable_ai": false }"#),
                cx,
            )
            .unwrap();
    });
    cx.update(|cx| {
        assert!(
            !DisableAiSettings::is_ai_disabled_for_file(Some(&file), cx),
            "Project false should allow AI (if global is false)"
        );
    });

    // 5. Global true, Project false. Result: true (SaturatingBool).
    cx.update_global::<SettingsStore, _>(|store, cx| {
        store.set_user_settings(r#"{ "disable_ai": true }"#, cx).unwrap();
    });
    cx.update(|cx| {
        assert!(
            DisableAiSettings::is_ai_disabled_for_file(Some(&file), cx),
            "Global true should override Project false"
        );
    });
}
