mod icon;

pub use icon::{Icon, IconSize};

use gpui::SharedString;

/// Trait for types that can be used as RemixIcon names.
pub trait RemixIcon {
    /// Returns the asset path for this icon.
    fn path(self) -> SharedString;
}

// Include generated icon names
include!(concat!(env!("OUT_DIR"), "/icon_name.rs"));
