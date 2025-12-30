use crate::RemixIcon;
use gpui::{
    AnyElement, App, AppContext as _, Context, Entity, Hsla, IntoElement, Pixels, Radians, Render,
    RenderOnce, SharedString, StyleRefinement, Styled, Svg, Transformation, Window,
    prelude::FluentBuilder as _, svg,
};

/// Types implementing this trait can automatically be converted to [`Icon`].
///
/// This allows RemixIcon types to function as drop-in replacements for other UI components.
pub trait IconNamed {
    /// Returns the embedded path of the icon.
    fn path(self) -> SharedString;
}

impl<T: RemixIcon> IconNamed for T {
    fn path(self) -> SharedString {
        RemixIcon::path(self)
    }
}

impl<T: IconNamed> From<T> for Icon {
    fn from(value: T) -> Self {
        Icon::build(value)
    }
}

/// Icon size variants.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    /// Extra small: 12px
    XSmall,
    /// Small: 14px
    Small,
    /// Medium: 16px (default)
    Medium,
    /// Large: 24px
    Large,
    /// Custom size in pixels
    Size(Pixels),
}

impl From<Pixels> for Size {
    fn from(px: Pixels) -> Self {
        Size::Size(px)
    }
}

/// Trait for types that can have a size.
pub trait Sizable {
    /// Set the size of the element.
    fn with_size(self, size: impl Into<Size>) -> Self;
}

/// A RemixIcon component for GPUI.
///
/// This type is designed to be compatible with gpui-component's Icon type.
#[derive(IntoElement)]
pub struct Icon {
    base: Svg,
    style: StyleRefinement,
    path: SharedString,
    text_color: Option<Hsla>,
    size: Option<Size>,
    rotation: Option<Radians>,
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            base: svg().flex_none().size_4(),
            style: StyleRefinement::default(),
            path: "".into(),
            text_color: None,
            size: None,
            rotation: None,
        }
    }
}

impl Clone for Icon {
    fn clone(&self) -> Self {
        let mut this = Self::default().path(self.path.clone());
        this.style = self.style.clone();
        this.rotation = self.rotation;
        this.size = self.size;
        this.text_color = self.text_color;
        this
    }
}

impl Icon {
    /// Create a new Icon from any type that implements IconNamed.
    pub fn new(icon: impl Into<Icon>) -> Self {
        icon.into()
    }

    fn build(name: impl IconNamed) -> Self {
        Self::default().path(name.path())
    }

    /// Set the icon path of the Assets bundle.
    ///
    /// For example: `icons/foo.svg`
    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = path.into();
        self
    }

    /// Create a new view for the icon.
    pub fn view(self, cx: &mut App) -> Entity<Icon> {
        cx.new(|_| self)
    }

    /// Apply a transformation to the icon.
    pub fn transform(mut self, transformation: Transformation) -> Self {
        self.base = self.base.with_transformation(transformation);
        self
    }

    /// Create an empty icon (for custom paths).
    pub fn empty() -> Self {
        Self::default()
    }

    /// Rotate the icon by the given angle.
    pub fn rotate(mut self, radians: impl Into<Radians>) -> Self {
        self.rotation = Some(radians.into());
        self
    }

    /// Set extra small size (12px).
    pub fn xsmall(self) -> Self {
        self.with_size(Size::XSmall)
    }

    /// Set small size (14px).
    pub fn small(self) -> Self {
        self.with_size(Size::Small)
    }

    /// Set medium size (16px).
    pub fn medium(self) -> Self {
        self.with_size(Size::Medium)
    }

    /// Set large size (24px).
    pub fn large(self) -> Self {
        self.with_size(Size::Large)
    }

    /// Set a custom size in pixels.
    pub fn custom_size(self, px: impl Into<Pixels>) -> Self {
        self.with_size(Size::Size(px.into()))
    }
}

impl Styled for Icon {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }

    fn text_color(mut self, color: impl Into<Hsla>) -> Self {
        self.text_color = Some(color.into());
        self
    }
}

impl Sizable for Icon {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = Some(size.into());
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text_color = self.text_color.unwrap_or_else(|| window.text_style().color);
        let text_size = window.text_style().font_size.to_pixels(window.rem_size());
        let has_base_size = self.style.size.width.is_some() || self.style.size.height.is_some();

        let mut base = self.base;
        *base.style() = self.style;

        base.flex_shrink_0()
            .text_color(text_color)
            .when(!has_base_size, |this| this.size(text_size))
            .when_some(self.size, |this, size| match size {
                Size::Size(px) => this.size(px),
                Size::XSmall => this.size_3(),
                Size::Small => this.size_3p5(),
                Size::Medium => this.size_4(),
                Size::Large => this.size_6(),
            })
            .path(self.path)
            .when_some(self.rotation, |this, rotation| {
                this.with_transformation(Transformation::rotate(rotation))
            })
    }
}

impl From<Icon> for AnyElement {
    fn from(icon: Icon) -> Self {
        icon.into_any_element()
    }
}

impl Render for Icon {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let text_color = self.text_color.unwrap_or_else(|| window.text_style().color);
        let text_size = window.text_style().font_size.to_pixels(window.rem_size());
        let has_base_size = self.style.size.width.is_some() || self.style.size.height.is_some();

        let mut base = svg().flex_none();
        *base.style() = self.style.clone();

        base.flex_shrink_0()
            .text_color(text_color)
            .when(!has_base_size, |this| this.size(text_size))
            .when_some(self.size, |this, size| match size {
                Size::Size(px) => this.size(px),
                Size::XSmall => this.size_3(),
                Size::Small => this.size_3p5(),
                Size::Medium => this.size_4(),
                Size::Large => this.size_6(),
            })
            .path(self.path.clone())
            .when_some(self.rotation, |this, rotation| {
                this.with_transformation(Transformation::rotate(rotation))
            })
    }
}
