//! Example demonstrating gpui-remixicon as a gpui-component compatible Icon.
//!
//! This example shows how to use RemixIcon with the Icon API that is compatible
//! with gpui-component's Icon type, including:
//! - `Icon::new()` with automatic conversion from RemixIcon types
//! - `Sizable` trait with `with_size()` and size presets
//! - `Styled` trait with `text_color()` and other style methods
//! - `Render` trait for use as `Entity<Icon>`

use gpui::*;
use gpui_component::{Sizable, button::{Button, ButtonVariants}};
use gpui_remixicon::{Assets, arrows, system};

pub struct GpuiIconExample;

impl GpuiIconExample {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for GpuiIconExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .p_8()
            .gap_6()
            .child(
                div()
                    .child("Button with Icon")
                    .text_lg()
                    .font_weight(FontWeight::BOLD),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .child(
                        Button::new("model-selector")
                            .label("Opus")
                            .icon(arrows::Icon::ArrowDropDownLine)
                            .xsmall()
                            .ghost(),
                    )
                    .child(
                        Button::new("check-button")
                            .label("Check")
                            .icon(system::Icon::CheckLine)
                            .small()
                            .primary(),
                    )
                    .child(
                        Button::new("delete-button")
                            .label("Delete")
                            .icon(system::Icon::DeleteBinLine)
                            .danger(),
                    )
                    .child(
                        Button::new("download-button")
                            .label("Download")
                            .icon(system::Icon::DownloadLine)
                            .large()
                            .outline(),
                    ),
            )
    }
}

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(700.0), px(500.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| GpuiIconExample::new(cx)),
        )
        .unwrap();
    });
}
