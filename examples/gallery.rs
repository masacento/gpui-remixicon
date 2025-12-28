use gpui::*;
use gpui_remixicon::{Icon, arrows, system};
use std::borrow::Cow;

struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let full_path = format!("{}/assets/{}", env!("CARGO_MANIFEST_DIR"), path);
        match std::fs::read(&full_path) {
            Ok(data) => Ok(Some(Cow::Owned(data))),
            Err(_) => Ok(None),
        }
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let full_path = format!("{}/assets/{}", env!("CARGO_MANIFEST_DIR"), path);
        let mut results = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&full_path) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    results.push(format!("{}/{}", path, name).into());
                }
            }
        }
        Ok(results)
    }
}

struct Gallery;

impl Render for Gallery {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .p_8()
            .gap_6()
            // Small icons
            .child(
                div()
                    .child("Small (14px)")
                    .text_sm()
                    .text_color(rgb(0x888888)),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_4()
                    .child(Icon::new(system::Icon::AddLine).small())
                    .child(Icon::new(system::Icon::CheckLine).small())
                    .child(Icon::new(system::Icon::CloseLine).small())
                    .child(Icon::new(system::Icon::Search2Line).small())
                    .child(Icon::new(system::Icon::SettingsLine).small())
                    .child(Icon::new(system::Icon::StarLine).small())
                    .child(Icon::new(system::Icon::TimeLine).small())
                    .child(Icon::new(system::Icon::DownloadLine).small())
                    .child(Icon::new(system::Icon::UploadLine).small())
                    .child(Icon::new(system::Icon::RefreshLine).small()),
            )
            // Medium icons
            .child(
                div()
                    .child("Medium (16px)")
                    .text_sm()
                    .text_color(rgb(0x888888)),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_4()
                    .child(Icon::new(system::Icon::AddFill).medium())
                    .child(Icon::new(system::Icon::CheckFill).medium())
                    .child(Icon::new(system::Icon::CloseFill).medium())
                    .child(Icon::new(system::Icon::Search2Fill).medium())
                    .child(Icon::new(system::Icon::SettingsFill).medium())
                    .child(Icon::new(system::Icon::StarFill).medium())
                    .child(Icon::new(system::Icon::TimeFill).medium())
                    .child(Icon::new(system::Icon::DownloadFill).medium())
                    .child(Icon::new(system::Icon::UploadFill).medium())
                    .child(Icon::new(system::Icon::RefreshFill).medium()),
            )
            // Large icons
            .child(
                div()
                    .child("Large (24px)")
                    .text_sm()
                    .text_color(rgb(0x888888)),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_4()
                    .child(Icon::new(arrows::Icon::ArrowUpLine).large())
                    .child(Icon::new(arrows::Icon::ArrowDownLine).large())
                    .child(Icon::new(arrows::Icon::ArrowLeftLine).large())
                    .child(Icon::new(arrows::Icon::ArrowRightLine).large())
                    .child(Icon::new(arrows::Icon::ArrowUpCircleLine).large())
                    .child(Icon::new(arrows::Icon::ArrowDownCircleLine).large())
                    .child(Icon::new(arrows::Icon::ArrowLeftCircleLine).large())
                    .child(Icon::new(arrows::Icon::ArrowRightCircleLine).large())
                    .child(Icon::new(arrows::Icon::ArrowUpSLine).large())
                    .child(Icon::new(arrows::Icon::ArrowDownSLine).large()),
            )
    }
}

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(600.0), px(300.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |_window, cx| cx.new(|_| Gallery),
        )
        .unwrap();
    });
}
