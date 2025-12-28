# gpui-remixicon

[Remix Icon](https://remixicon.com) component for [GPUI](https://gpui.rs/).

> Remix Icon is a set of open-source neutral-style system symbols for designers and developers. Unlike a patchwork icon library, 3100+ icons are all elaborately crafted so that they are born with the genes of readability, consistency, and perfect pixels. Each icon was designed in "Outlined" and "Filled" styles based on a 24x24 grid.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
gpui-remixicon = { git = "https://github.com/masacento/gpui-remixicon" }
```

### Selective Import

To reduce binary size, you can import only the icon categories you need:

```toml
[dependencies]
gpui-remixicon = { git = "https://github.com/masacento/gpui-remixicon", default-features = false, features = ["arrows", "system"] }
```

## Usage

### Basic Usage

```rust
use gpui::*;
use gpui_remixicon::{Assets, Icon, system, arrows};

fn main() {
    // Assets includes embedded SVG icons
    let app = Application::new().with_assets(Assets);
    // ...
}

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

### Icon Categories (Features)

Icons are organized by category modules. Each category can be enabled/disabled via Cargo features:

| Feature | Module | Description |
|---------|--------|-------------|
| `arrows` | `arrows` | Arrow icons |
| `buildings` | `buildings` | Building icons |
| `business` | `business` | Business icons |
| `communication` | `communication` | Communication icons |
| `design` | `design` | Design tools icons |
| `development` | `development` | Development icons |
| `device` | `device` | Device icons |
| `document` | `document` | Document icons |
| `editor` | `editor` | Editor icons |
| `finance` | `finance` | Finance icons |
| `food` | `food` | Food icons |
| `health_and_medical` | `health_and_medical` | Health & Medical icons |
| `logos` | `logos` | Logo icons |
| `map` | `map` | Map icons |
| `media` | `media` | Media icons |
| `others` | `others` | Other icons |
| `system` | `system` | System icons |
| `user_and_faces` | `user_and_faces` | User & Faces icons |
| `weather` | `weather` | Weather icons |
| `all` | - | All categories (default) |

### Category-Specific Assets

Each category also provides its own `Assets` struct if you only need icons from one category:

```rust
use gpui_remixicon::arrows::{ArrowsAssets, Icon};

let app = Application::new().with_assets(ArrowsAssets);
```

### Example

```rust
use gpui::*;
use gpui_remixicon::{Assets, Icon, system, arrows};

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

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|_| MyView)
        })
        .unwrap();
    });
}
```

Run the gallery example:

```bash
cargo run --example gallery
```

## License

This crate is licensed under the Apache License 2.0.

[Remix Icon](https://github.com/Remix-Design/RemixIcon) is licensed under the [Apache License Version 2.0](https://github.com/Remix-Design/RemixIcon/blob/master/License). Feel free to use these icons in your products and distribute them. The only thing we ask is that these icons are not for sale.
