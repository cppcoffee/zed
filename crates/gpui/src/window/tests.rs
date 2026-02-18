use crate::{
    div, AppContext, Bounds, Context, InteractiveElement, IntoElement, Point,
    Render, Size, Styled, TestAppContext, Window, WindowBounds, WindowOptions, px,
};

struct TestView;

impl Render for TestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().id("test-view").size_full()
    }
}

#[crate::test]
fn test_opening_window_from_active_window_update_bug(cx: &mut TestAppContext) {
    cx.update(|cx| {
        // Create the first window.
        let window_1 = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                        Point::new(px(100.), px(100.)),
                        Size::new(px(500.), px(500.)),
                    ))),
                    focus: true,
                    ..Default::default()
                },
                |_, cx| cx.new(|_| TestView),
            )
            .unwrap();

        // Ensure window 1 is active.
        window_1
            .update(cx, |_, window, _| {
                window.activate_window();
            })
            .unwrap();
    });

    // Simulate an update inside window 1 that opens window 2.
    let window_1 = cx.windows()[0];
    cx.update_window(window_1, |_, _, cx| {
        // Verify window 1 is active
        assert_eq!(cx.active_window().unwrap(), window_1);

        // Open window 2.
        let window_2 = cx
            .open_window(WindowOptions::default(), |_, cx| cx.new(|_| TestView))
            .unwrap();

        window_2.update(cx, |_, window, _| {
            let bounds = window.window_bounds();

            let origin = match bounds {
                WindowBounds::Windowed(b) => b.origin,
                _ => panic!("Expected windowed bounds"),
            };

            assert_eq!(origin.x, px(125.0), "Window 2 X origin should be cascaded from Window 1");
            assert_eq!(origin.y, px(125.0), "Window 2 Y origin should be cascaded from Window 1");
        }).unwrap();
    })
    .unwrap();
}
