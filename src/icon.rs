use crate::RemixIcon;
use gpui::{
    AnyElement, App, Hsla, IntoElement, Pixels, Radians, RenderOnce, SharedString, Styled,
    Transformation, Window, prelude::FluentBuilder as _, svg,
};

/// Icon size variants.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconSize {
    /// Extra small: 12px
    XSmall,
    /// Small: 14px
    Small,
    /// Medium: 16px (default)
    Medium,
    /// Large: 24px
    Large,
    /// Custom size in pixels
    Custom(Pixels),
}

/// A RemixIcon component for GPUI.
#[derive(IntoElement)]
pub struct Icon {
    path: SharedString,
    size: Option<IconSize>,
    color: Option<Hsla>,
    rotation: Option<Radians>,
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            path: "".into(),
            size: None,
            color: None,
            rotation: None,
        }
    }
}

impl Icon {
    /// Create a new Icon from a RemixIcon type.
    pub fn new(icon: impl RemixIcon) -> Self {
        Self {
            path: icon.path(),
            ..Default::default()
        }
    }

    /// Create an empty icon (for custom paths).
    pub fn empty() -> Self {
        Self::default()
    }

    /// Set a custom SVG path.
    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = path.into();
        self
    }

    /// Set the icon size.
    pub fn size(mut self, size: IconSize) -> Self {
        self.size = Some(size);
        self
    }

    /// Set extra small size (12px).
    pub fn xsmall(self) -> Self {
        self.size(IconSize::XSmall)
    }

    /// Set small size (14px).
    pub fn small(self) -> Self {
        self.size(IconSize::Small)
    }

    /// Set medium size (16px).
    pub fn medium(self) -> Self {
        self.size(IconSize::Medium)
    }

    /// Set large size (24px).
    pub fn large(self) -> Self {
        self.size(IconSize::Large)
    }

    /// Set a custom size in pixels.
    pub fn custom_size(self, px: impl Into<Pixels>) -> Self {
        self.size(IconSize::Custom(px.into()))
    }

    /// Set the icon color.
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Rotate the icon by the given angle in radians.
    pub fn rotate(mut self, radians: impl Into<Radians>) -> Self {
        self.rotation = Some(radians.into());
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text_color = self.color.unwrap_or_else(|| window.text_style().color);

        let base = svg()
            .flex_none()
            .flex_shrink_0()
            .text_color(text_color)
            .path(self.path);

        let base = match self.size {
            Some(IconSize::XSmall) => base.size_3(),
            Some(IconSize::Small) => base.size_3p5(),
            Some(IconSize::Medium) => base.size_4(),
            Some(IconSize::Large) => base.size_6(),
            Some(IconSize::Custom(px)) => base.size(px),
            None => base.size_4(),
        };

        base.when_some(self.rotation, |this, rotation| {
            this.with_transformation(Transformation::rotate(rotation))
        })
    }
}

impl From<Icon> for AnyElement {
    fn from(icon: Icon) -> Self {
        icon.into_any_element()
    }
}
