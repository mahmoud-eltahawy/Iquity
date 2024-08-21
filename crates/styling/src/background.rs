use super::{Attribute, Style};
use crate::{color::Color, length::Length, PreBase, StyleBaseState, StyleState};
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

pub struct BaseState;
pub struct SizeState;
pub struct PreXPosition;
impl StyleState for SizeState {}
impl StyleState for BaseState {}
impl StyleState for PreXPosition {}
impl StyleState for PositionX {}

impl Style<BaseState> {
    fn pre_base<T>(self, fun: Box<dyn FnOnce(T) -> Attribute>) -> Style<PreBase<T>> {
        let Self(style, _) = self;
        Style(style, fun)
    }

    pub fn color(self) -> Style<PreBase<Color>> {
        self.pre_base(Box::new(Attribute::BackgroundColor))
    }

    pub fn size(self) -> Style<SizeState> {
        let Style(style, _) = self;
        Style(style, SizeState)
    }

    pub fn image(self, source: &str) -> Style<StyleBaseState> {
        let Self(mut style, _) = self;
        style.insert(Attribute::BackgroundImage(source.to_string()));
        Style(style, ())
    }

    pub fn repeat(self) -> Style<PreBase<Repeat>> {
        self.pre_base(Box::new(Attribute::BackgroundRepeat))
    }

    pub fn origin(self) -> Style<PreBase<Origin>> {
        self.pre_base(Box::new(Attribute::BackgroundOrigin))
    }

    pub fn clip(self) -> Style<PreBase<Origin>> {
        self.pre_base(Box::new(Attribute::BackgroundClip))
    }

    pub fn blend_mode(self) -> Style<PreBase<BlendMode>> {
        self.pre_base(Box::new(Attribute::BackgroundBlendMode))
    }

    pub fn attachment(self) -> Style<PreBase<Attachment>> {
        self.pre_base(Box::new(Attribute::BackgroundAttachment))
    }

    pub fn position(self) -> Style<PreXPosition> {
        let Self(style, _) = self;
        Style(style, PreXPosition)
    }

    pub fn position_x(self) -> Style<PreBase<PositionX>> {
        let Self(style, _) = self;
        Style(style, Box::new(Attribute::BackgroundPositionX))
    }

    pub fn position_y(self) -> Style<PreBase<PositionY>> {
        let Self(style, _) = self;
        Style(style, Box::new(Attribute::BackgroundPositionY))
    }
}

impl Style<SizeState> {
    fn inner(self, size: Size) -> Style<StyleBaseState> {
        let Self(mut style, _) = self;
        style.insert(Attribute::BackgroundSize(size));
        Style(style, ())
    }

    pub fn initial(self) -> Style<StyleBaseState> {
        self.inner(Size::Initial)
    }

    pub fn auto(self) -> Style<StyleBaseState> {
        self.inner(Size::Auto)
    }

    pub fn inherit(self) -> Style<StyleBaseState> {
        self.inner(Size::Inherit)
    }

    pub fn contain(self) -> Style<StyleBaseState> {
        self.inner(Size::Contain)
    }

    pub fn cover(self) -> Style<StyleBaseState> {
        self.inner(Size::Cover)
    }

    pub fn length(self) -> Style<PreBase<Length>> {
        let Self(style, _) = self;
        Style(
            style,
            Box::new(|x| Attribute::BackgroundSize(Size::Length(x))),
        )
    }
}

impl Style<PreBase<PositionX>> {
    pub fn left(self) -> Style<StyleBaseState> {
        self.base(PositionX::Left)
    }

    pub fn right(self) -> Style<StyleBaseState> {
        self.base(PositionX::Right)
    }

    pub fn center(self) -> Style<StyleBaseState> {
        self.base(PositionX::Center)
    }
}

impl Style<PreBase<PositionY>> {
    pub fn top(self) -> Style<StyleBaseState> {
        self.base(PositionY::Top)
    }

    pub fn bottom(self) -> Style<StyleBaseState> {
        self.base(PositionY::Bottom)
    }

    pub fn center(self) -> Style<StyleBaseState> {
        self.base(PositionY::Center)
    }
}

impl Style<PreBase<Origin>> {
    pub fn padding_box(self) -> Style<StyleBaseState> {
        self.base(Origin::PaddingBox)
    }

    pub fn border_box(self) -> Style<StyleBaseState> {
        self.base(Origin::BorderBox)
    }

    pub fn content_box(self) -> Style<StyleBaseState> {
        self.base(Origin::ContentBox)
    }

    pub fn initial(self) -> Style<StyleBaseState> {
        self.base(Origin::Initial)
    }

    pub fn inherit(self) -> Style<StyleBaseState> {
        self.base(Origin::Inherit)
    }
}

impl Style<PreBase<BlendMode>> {
    pub fn normal(self) -> Style<StyleBaseState> {
        self.base(BlendMode::Normal)
    }

    pub fn multiply(self) -> Style<StyleBaseState> {
        self.base(BlendMode::Multiply)
    }

    pub fn screen(self) -> Style<StyleBaseState> {
        self.base(BlendMode::Screen)
    }

    pub fn overlay(self) -> Style<StyleBaseState> {
        self.base(BlendMode::Overlay)
    }

    pub fn darken(self) -> Style<StyleBaseState> {
        self.base(BlendMode::Darken)
    }

    pub fn lighten(self) -> Style<StyleBaseState> {
        self.base(BlendMode::Lighten)
    }

    pub fn color_dodge(self) -> Style<StyleBaseState> {
        self.base(BlendMode::ColorDodge)
    }

    pub fn saturation(self) -> Style<StyleBaseState> {
        self.base(BlendMode::Saturation)
    }

    pub fn color(self) -> Style<StyleBaseState> {
        self.base(BlendMode::Color)
    }

    pub fn luminosity(self) -> Style<StyleBaseState> {
        self.base(BlendMode::Luminosity)
    }
}

impl Style<PreXPosition> {
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
    fn inner(self, y: PositionY) -> Style<StyleBaseState> {
        let Self(mut style, x) = self;
        style.insert(Attribute::BackgroundPosition(XYPosition(x, y)));
        Style(style, ())
    }

    pub fn top(self) -> Style<StyleBaseState> {
        self.inner(PositionY::Top)
    }

    pub fn bottom(self) -> Style<StyleBaseState> {
        self.inner(PositionY::Bottom)
    }

    pub fn center(self) -> Style<StyleBaseState> {
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

impl Style<PreBase<Attachment>> {
    pub fn scroll(self) -> Style<StyleBaseState> {
        self.base(Attachment::Scroll)
    }

    pub fn fixed(self) -> Style<StyleBaseState> {
        self.base(Attachment::Fixed)
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

impl Style<PreBase<Repeat>> {
    pub fn x(self) -> Style<StyleBaseState> {
        self.base(Repeat::X)
    }
    pub fn y(self) -> Style<StyleBaseState> {
        self.base(Repeat::Y)
    }
    pub fn none(self) -> Style<StyleBaseState> {
        self.base(Repeat::None)
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
