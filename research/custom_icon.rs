use gpui::*;
use gpui_component::{Icon, IconNamed, Root, button::Button, h_flex, v_flex};
use std::borrow::Cow;

#[derive(Clone, Copy)]
enum CustomIconName {
    Sparkle,
}

impl IconNamed for CustomIconName {
    fn path(self) -> SharedString {
        match self {
            Self::Sparkle => "icons/custom/sparkle.svg".into(),
        }
    }
}

struct AppAssets {
    component: gpui_component_assets::Assets,
}

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        if path == "icons/custom/sparkle.svg" {
            return Ok(Some(Cow::Borrowed(include_bytes!(
                "assets/icons/custom/sparkle.svg"
            ))));
        }

        self.component.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut assets = self.component.list(path)?;

        if "icons/custom".starts_with(path) || path.starts_with("icons/custom") {
            assets.push("icons/custom/sparkle.svg".into());
        }

        assets.sort();
        assets.dedup();
        Ok(assets)
    }
}

struct CustomIconExample;

impl Render for CustomIconExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .p_4()
            .gap_4()
            .child("Custom icon example (IconNamed + custom assets)")
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(Icon::new(CustomIconName::Sparkle).size_16())
                    .child("This is a custom icon"),
            )
            .child(
                Button::new("custom-icon-button")
                    .icon(CustomIconName::Sparkle)
                    .label("Button with custom icon"),
            )
    }
}

fn main() {
    let app = Application::new().with_assets(AppAssets {
        component: gpui_component_assets::Assets,
    });

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| CustomIconExample);
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
