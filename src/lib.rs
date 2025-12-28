//! RemixIcon icons for GPUI.
//!
//! Each icon category is available as a module (e.g. `arrows`, `system`), and each module exposes
//! an `Icon` enum that implements [`RemixIcon`].
//!
//! Category module docs include a mapping table from RemixIcon labels (e.g. `arrow-up-s-line`) to
//! Rust variants (e.g. [`arrows::Icon::ArrowUpSLine`]).
//!
//! ```
//! use gpui_remixicon::{Icon, arrows};
//!
//! let up = Icon::new(arrows::Icon::ArrowUpSLine);
//! ```

mod icon;

pub use icon::{Icon, IconSize};

use gpui::SharedString;

/// Trait for types that can be used as RemixIcon names.
pub trait RemixIcon {
    /// Returns the asset path for this icon.
    fn path(self) -> SharedString;
}

// Include generated icon names (category modules and RemixIconName enum)
include!(concat!(env!("OUT_DIR"), "/icon_name.rs"));

// Include generated assets (Assets struct with AssetSource impl)
include!(concat!(env!("OUT_DIR"), "/assets.rs"));
