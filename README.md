# gpui-remixicon

[Remix Icon](https://remixicon.com) component for [GPUI](https://gpui.rs/).

> Remix Icon is a set of open-source neutral-style system symbols for designers and developers. Unlike a patchwork icon library, 3100+ icons are all elaborately crafted so that they are born with the genes of readability, consistency, and perfect pixels. Each icon was designed in "Outlined" and "Filled" styles based on a 24x24 grid.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
gpui-remixicon = { git = "https://github.com/masacento/gpui-remixicon" }
```

## Usage

### Basic Usage

```rust
use gpui_remixicon::{Icon, system, arrows};

// Create an icon
Icon::new(system::Icon::AddLine)

// With size
Icon::new(system::Icon::CheckLine).small()
Icon::new(system::Icon::CloseLine).medium()
Icon::new(system::Icon::StarFill).large()

// With color
Icon::new(arrows::Icon::ArrowUpLine).color(gpui::red())
```

### Available Sizes

| Method | Size |
|--------|------|
| `.xsmall()` | 12px |
| `.small()` | 14px |
| `.medium()` | 16px |
| `.large()` | 24px |
| `.custom_size(px(32.0))` | Custom |

### Icon Categories

Icons are organized by category modules:

- `arrows` - Arrow icons
- `buildings` - Building icons
- `business` - Business icons
- `communication` - Communication icons
- `design` - Design tools icons
- `development` - Development icons
- `device` - Device icons
- `document` - Document icons
- `editor` - Editor icons
- `finance` - Finance icons
- `food` - Food icons
- `healthmedical` - Health & Medical icons
- `logos` - Logo icons
- `map` - Map icons
- `media` - Media icons
- `others` - Other icons
- `system` - System icons
- `userfaces` - User & Faces icons
- `weather` - Weather icons

### Asset Setup

You need to configure your application's `AssetSource` to load the SVG files:

```rust
use gpui::*;
use std::borrow::Cow;

struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        // Load from your assets directory
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

fn main() {
    let app = Application::new().with_assets(Assets);
    // ...
}
```

Copy the `assets/icons` directory from this crate to your project's assets directory.

### Example

```rust
use gpui::*;
use gpui_remixicon::{Icon, system, arrows};

struct MyView;

impl Render for MyView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap_2()
            .child(Icon::new(system::Icon::AddLine).small())
            .child(Icon::new(system::Icon::CheckFill).medium())
            .child(Icon::new(arrows::Icon::ArrowRightLine).large())
    }
}
```

Run the gallery example:

```bash
cargo run --example gallery
```

## License

This crate is licensed under the Apache License 2.0.

[Remix Icon](https://github.com/Remix-Design/RemixIcon) is licensed under the [Apache License Version 2.0](https://github.com/Remix-Design/RemixIcon/blob/master/License). Feel free to use these icons in your products and distribute them. The only thing we ask is that these icons are not for sale.
