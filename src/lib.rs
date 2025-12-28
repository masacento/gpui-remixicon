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
