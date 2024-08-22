use super::{Attribute, Style};
use crate::{
    color::Color, length::Length, AttributeGetter, Attributs, PreState, StyleBaseState, StyleState,
};
use std::fmt::Display;

#[derive(Hash, Eq, PartialEq)]
pub struct XYPosition(PositionX, PositionY);

#[derive(Hash, Eq, PartialEq)]
pub enum PositionY {
    Top,
    Bottom,
    Center,
}

#[derive(Hash, Eq, PartialEq)]
pub enum PositionX {
    Left,
    Right,
    Center,
}

#[derive(Hash, Eq, PartialEq)]
pub enum Repeat {
    X,
    Y,
    None,
}

#[derive(Hash, Eq, PartialEq)]
pub enum Origin {
    PaddingBox,
    BorderBox,
    ContentBox,
    Initial,
    Inherit,
}

#[derive(Hash, Eq, PartialEq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    Saturation,
    Color,
    Luminosity,
}

#[derive(Hash, Eq, PartialEq)]
pub enum Size {
    Auto,
    Initial,
    Inherit,
    Contain,
    Cover,
    Length(Length),
}

#[derive(Hash, Eq, PartialEq)]
pub enum Attachment {
    Fixed,
    Scroll,
}

#[derive(Default)]
pub struct BackgroundBaseState;
pub struct BackgroundSizeState;
pub struct BackgroundPreXPosition;
pub struct PreBackgroundBase<T>(AttributeGetter<T>);
impl StyleState for BackgroundSizeState {}
impl StyleState for BackgroundBaseState {}
impl StyleState for BackgroundPreXPosition {}
impl StyleState for PositionX {}
impl<T> StyleState for PreBackgroundBase<T> {}

impl<T> PreState<T, BackgroundBaseState> for Style<PreBackgroundBase<T>> {
    fn destruct(self) -> (Attributs, AttributeGetter<T>) {
        let Self(attrs, PreBackgroundBase(fun)) = self;
        (attrs, fun)
    }
}

impl Style<BackgroundBaseState> {
    fn pre_base<T>(self, fun: AttributeGetter<T>) -> Style<PreBackgroundBase<T>> {
        let Self(style, _) = self;
        Style(style, PreBackgroundBase(fun))
    }

    pub fn base(self) -> Style<StyleBaseState> {
        let Self(style, _) = self;
        Style(style, Default::default())
    }

    pub fn color(self) -> Style<PreBackgroundBase<Color>> {
        self.pre_base(Box::new(Attribute::BackgroundColor))
    }

    pub fn size(self) -> Style<BackgroundSizeState> {
        let Style(style, _) = self;
        Style(style, BackgroundSizeState)
    }

    pub fn image(self, source: &str) -> Style<BackgroundBaseState> {
        let Self(mut style, _) = self;
        style.insert(Attribute::BackgroundImage(source.to_string()));
        Style(style, BackgroundBaseState)
    }

    pub fn repeat(self) -> Style<PreBackgroundBase<Repeat>> {
        self.pre_base(Box::new(Attribute::BackgroundRepeat))
    }

    pub fn origin(self) -> Style<PreBackgroundBase<Origin>> {
        self.pre_base(Box::new(Attribute::BackgroundOrigin))
    }

    pub fn clip(self) -> Style<PreBackgroundBase<Origin>> {
        self.pre_base(Box::new(Attribute::BackgroundClip))
    }

    pub fn blend_mode(self) -> Style<PreBackgroundBase<BlendMode>> {
        self.pre_base(Box::new(Attribute::BackgroundBlendMode))
    }

    pub fn attachment(self) -> Style<PreBackgroundBase<Attachment>> {
        self.pre_base(Box::new(Attribute::BackgroundAttachment))
    }

    pub fn position(self) -> Style<BackgroundPreXPosition> {
        let Self(style, _) = self;
        Style(style, BackgroundPreXPosition)
    }

    pub fn position_x(self) -> Style<PreBackgroundBase<PositionX>> {
        let Self(style, _) = self;
        Style(
            style,
            PreBackgroundBase(Box::new(Attribute::BackgroundPositionX)),
        )
    }

    pub fn position_y(self) -> Style<PreBackgroundBase<PositionY>> {
        let Self(style, _) = self;
        Style(
            style,
            PreBackgroundBase(Box::new(Attribute::BackgroundPositionY)),
        )
    }
}

impl Style<BackgroundSizeState> {
    fn inner(self, size: Size) -> Style<BackgroundBaseState> {
        let Self(mut style, _) = self;
        style.insert(Attribute::BackgroundSize(size));
        Style(style, BackgroundBaseState)
    }

    pub fn initial(self) -> Style<BackgroundBaseState> {
        self.inner(Size::Initial)
    }

    pub fn auto(self) -> Style<BackgroundBaseState> {
        self.inner(Size::Auto)
    }

    pub fn inherit(self) -> Style<BackgroundBaseState> {
        self.inner(Size::Inherit)
    }

    pub fn contain(self) -> Style<BackgroundBaseState> {
        self.inner(Size::Contain)
    }

    pub fn cover(self) -> Style<BackgroundBaseState> {
        self.inner(Size::Cover)
    }

    pub fn length(self) -> Style<PreBackgroundBase<Length>> {
        let Self(style, _) = self;
        Style(
            style,
            PreBackgroundBase(Box::new(|x| Attribute::BackgroundSize(Size::Length(x)))),
        )
    }
}

impl Style<PreBackgroundBase<PositionX>> {
    pub fn left(self) -> Style<BackgroundBaseState> {
        self.base(PositionX::Left)
    }

    pub fn right(self) -> Style<BackgroundBaseState> {
        self.base(PositionX::Right)
    }

    pub fn center(self) -> Style<BackgroundBaseState> {
        self.base(PositionX::Center)
    }
}

impl Style<PreBackgroundBase<PositionY>> {
    pub fn top(self) -> Style<BackgroundBaseState> {
        self.base(PositionY::Top)
    }

    pub fn bottom(self) -> Style<BackgroundBaseState> {
        self.base(PositionY::Bottom)
    }

    pub fn center(self) -> Style<BackgroundBaseState> {
        self.base(PositionY::Center)
    }
}

impl Style<PreBackgroundBase<Origin>> {
    pub fn padding_box(self) -> Style<BackgroundBaseState> {
        self.base(Origin::PaddingBox)
    }

    pub fn border_box(self) -> Style<BackgroundBaseState> {
        self.base(Origin::BorderBox)
    }

    pub fn content_box(self) -> Style<BackgroundBaseState> {
        self.base(Origin::ContentBox)
    }

    pub fn initial(self) -> Style<BackgroundBaseState> {
        self.base(Origin::Initial)
    }

    pub fn inherit(self) -> Style<BackgroundBaseState> {
        self.base(Origin::Inherit)
    }
}

impl Style<PreBackgroundBase<BlendMode>> {
    pub fn normal(self) -> Style<BackgroundBaseState> {
        self.base(BlendMode::Normal)
    }

    pub fn multiply(self) -> Style<BackgroundBaseState> {
        self.base(BlendMode::Multiply)
    }

    pub fn screen(self) -> Style<BackgroundBaseState> {
        self.base(BlendMode::Screen)
    }

    pub fn overlay(self) -> Style<BackgroundBaseState> {
        self.base(BlendMode::Overlay)
    }

    pub fn darken(self) -> Style<BackgroundBaseState> {
        self.base(BlendMode::Darken)
    }

    pub fn lighten(self) -> Style<BackgroundBaseState> {
        self.base(BlendMode::Lighten)
    }

    pub fn color_dodge(self) -> Style<BackgroundBaseState> {
        self.base(BlendMode::ColorDodge)
    }

    pub fn saturation(self) -> Style<BackgroundBaseState> {
        self.base(BlendMode::Saturation)
    }

    pub fn color(self) -> Style<BackgroundBaseState> {
        self.base(BlendMode::Color)
    }

    pub fn luminosity(self) -> Style<BackgroundBaseState> {
        self.base(BlendMode::Luminosity)
    }
}

impl Style<BackgroundPreXPosition> {
    fn inner(self, x: PositionX) -> Style<PositionX> {
        let Self(style, _) = self;
        Style(style, x)
    }

    pub fn left(self) -> Style<PositionX> {
        self.inner(PositionX::Left)
    }

    pub fn right(self) -> Style<PositionX> {
        self.inner(PositionX::Right)
    }

    pub fn center(self) -> Style<PositionX> {
        self.inner(PositionX::Center)
    }
}

impl Style<PositionX> {
    fn inner(self, y: PositionY) -> Style<BackgroundBaseState> {
        let Self(mut style, x) = self;
        style.insert(Attribute::BackgroundPosition(XYPosition(x, y)));
        Style(style, BackgroundBaseState)
    }

    pub fn top(self) -> Style<BackgroundBaseState> {
        self.inner(PositionY::Top)
    }

    pub fn bottom(self) -> Style<BackgroundBaseState> {
        self.inner(PositionY::Bottom)
    }

    pub fn center(self) -> Style<BackgroundBaseState> {
        self.inner(PositionY::Center)
    }
}

impl Display for PositionX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            PositionX::Right => "right",
            PositionX::Left => "left",
            PositionX::Center => "center",
        };
        write!(f, "{result}")
    }
}

impl Display for PositionY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            PositionY::Top => "top",
            PositionY::Bottom => "bottom",
            PositionY::Center => "center",
        };
        write!(f, "{result}")
    }
}
impl Display for XYPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let XYPosition(x, y) = self;
        write!(f, "{x} {y}",)
    }
}

impl Style<PreBackgroundBase<Attachment>> {
    pub fn scroll(self) -> Style<BackgroundBaseState> {
        self.base(Attachment::Scroll)
    }

    pub fn fixed(self) -> Style<BackgroundBaseState> {
        self.base(Attachment::Fixed)
    }
}

impl Style<PreBackgroundBase<Repeat>> {
    pub fn x(self) -> Style<BackgroundBaseState> {
        self.base(Repeat::X)
    }
    pub fn y(self) -> Style<BackgroundBaseState> {
        self.base(Repeat::Y)
    }
    pub fn none(self) -> Style<BackgroundBaseState> {
        self.base(Repeat::None)
    }
}

impl Display for Attachment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Attachment::Fixed => "fixed",
            Attachment::Scroll => "scroll",
        };
        write!(f, "{result}")
    }
}

impl Display for Repeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Repeat::X => "repeat-x",
            Repeat::Y => "repeat-y",
            Repeat::None => "no-repeat",
        };
        write!(f, "{result}")
    }
}

impl Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Origin::PaddingBox => "padding-box",
            Origin::BorderBox => "border-box",
            Origin::ContentBox => "content-box",
            Origin::Initial => "initial",
            Origin::Inherit => "inherit",
        };
        write!(f, "{result}")
    }
}

impl Display for BlendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            BlendMode::Normal => "normal",
            BlendMode::Multiply => "multiply",
            BlendMode::Screen => "screen",
            BlendMode::Overlay => "overlay",
            BlendMode::Darken => "darken",
            BlendMode::Lighten => "lighten",
            BlendMode::ColorDodge => "colorDodge",
            BlendMode::Saturation => "saturation",
            BlendMode::Color => "color",
            BlendMode::Luminosity => "luminosity",
        };
        write!(f, "{result}")
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Size::Auto => "auto",
            Size::Initial => "initial",
            Size::Inherit => "inherit",
            Size::Contain => "contain",
            Size::Cover => "cover",
            Size::Length(len) => &len.to_string(),
        };
        write!(f, "{result}")
    }
}
