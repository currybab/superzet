use gpui::{
    App, Context, Entity, FocusHandle, Focusable, Render, Size, Tiling, Window, WindowBounds,
    WindowKind, WindowOptions, prelude::*, px,
};
use platform_title_bar::PlatformTitleBar;
use release_channel::ReleaseChannel;
use ui::{Label, prelude::*};
use util::ResultExt;
use workspace::client_side_decorations;

pub struct AudioTestWindow {
    title_bar: Option<Entity<PlatformTitleBar>>,
    focus_handle: FocusHandle,
}

impl AudioTestWindow {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let title_bar = if !cfg!(target_os = "macos") {
            Some(cx.new(|cx| PlatformTitleBar::new("audio-test-title-bar", cx)))
        } else {
            None
        };

        Self {
            title_bar,
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for AudioTestWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = v_flex()
            .id("audio-test-window")
            .track_focus(&self.focus_handle)
            .size_full()
            .p_4()
            .when(cfg!(target_os = "macos"), |this| this.pt_10())
            .gap_2()
            .bg(cx.theme().colors().editor_background)
            .child(Label::new("Audio testing is temporarily unavailable."))
            .child(
                Label::new("This stub keeps the UI surface in place without pulling audio and WebRTC dependencies into settings_ui.")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            );

        client_side_decorations(
            v_flex()
                .size_full()
                .text_color(cx.theme().colors().text)
                .children(self.title_bar.clone())
                .child(content),
            window,
            cx,
            Tiling::default(),
        )
    }
}

impl Focusable for AudioTestWindow {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

pub fn open_audio_test_window(_window: &mut Window, cx: &mut App) {
    let existing = cx
        .windows()
        .into_iter()
        .find_map(|window| window.downcast::<AudioTestWindow>());

    if let Some(existing) = existing {
        existing
            .update(cx, |_, window, _| window.activate_window())
            .log_err();
        return;
    }

    let app_id = ReleaseChannel::global(cx).app_id();
    let window_size = Size {
        width: px(480.0),
        height: px(180.0),
    };
    let window_min_size = Size {
        width: px(360.0),
        height: px(160.0),
    };

    cx.open_window(
        WindowOptions {
            titlebar: Some(gpui::TitlebarOptions {
                title: Some("Audio Test".into()),
                appears_transparent: true,
                traffic_light_position: Some(gpui::point(px(12.0), px(12.0))),
            }),
            focus: true,
            show: true,
            is_movable: true,
            kind: WindowKind::Normal,
            window_background: cx.theme().window_background_appearance(),
            app_id: Some(app_id.to_owned()),
            window_decorations: Some(gpui::WindowDecorations::Client),
            window_bounds: Some(WindowBounds::centered(window_size, cx)),
            window_min_size: Some(window_min_size),
            ..Default::default()
        },
        |_, cx| cx.new(AudioTestWindow::new),
    )
    .log_err();
}
