use super::{attribute::Attribute, Style};
use crate::{
    attribute, color::Color, length::Length, AttributeGetter, PreBaseState, StyleBaseState,
    StyleState,
};
use std::fmt::Display;

#[derive(Hash, Eq, PartialEq)]
pub struct XYPosition(PositionX, PositionY);

//TODO: missing some fields
#[derive(Hash, Eq, PartialEq)]
pub enum PositionY {
    Top,
    Bottom,
    Center,
}

//TODO: missing some fields
#[derive(Hash, Eq, PartialEq)]
pub enum PositionX {
    Left,
    Right,
    Center,
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

#[derive(Default)]
pub struct BackgroundSizeState;
pub struct BackgroundPreXPosition;
impl StyleState for BackgroundSizeState {}
impl StyleState for BackgroundPreXPosition {}
impl StyleState for PositionX {}

impl Style<StyleBaseState<()>> {
    pub fn background_color(self) -> Style<StyleBaseState<AttributeGetter<Color>>> {
        self.into_prebase(Box::new(Attribute::BackgroundColor))
    }

    pub fn background_size(self) -> Style<BackgroundSizeState> {
        let Style(style, _) = self;
        Style(style, BackgroundSizeState)
    }

    pub fn background_image(self, source: &str) -> Style<StyleBaseState<()>> {
        let Self(mut style, _) = self;
        style.insert(Attribute::BackgroundImage(source.to_string()));
        Style(style, StyleBaseState(()))
    }

    pub fn background_position(self) -> Style<BackgroundPreXPosition> {
        let Self(style, _) = self;
        Style(style, BackgroundPreXPosition)
    }

    pub fn background_position_x(self) -> Style<StyleBaseState<AttributeGetter<PositionX>>> {
        let Self(style, _) = self;
        Style(
            style,
            StyleBaseState(Box::new(Attribute::BackgroundPositionX)),
        )
    }

    pub fn background_position_y(self) -> Style<StyleBaseState<AttributeGetter<PositionY>>> {
        let Self(style, _) = self;
        Style(
            style,
            StyleBaseState(Box::new(Attribute::BackgroundPositionY)),
        )
    }
}

impl Style<BackgroundSizeState> {
    fn inner(self, size: Size) -> Style<StyleBaseState<()>> {
        let Self(mut style, _) = self;
        style.insert(Attribute::BackgroundSize(size));
        Style(style, StyleBaseState(()))
    }

    pub fn initial(self) -> Style<StyleBaseState<()>> {
        self.inner(Size::Initial)
    }

    pub fn auto(self) -> Style<StyleBaseState<()>> {
        self.inner(Size::Auto)
    }

    pub fn inherit(self) -> Style<StyleBaseState<()>> {
        self.inner(Size::Inherit)
    }

    pub fn contain(self) -> Style<StyleBaseState<()>> {
        self.inner(Size::Contain)
    }

    pub fn cover(self) -> Style<StyleBaseState<()>> {
        self.inner(Size::Cover)
    }

    pub fn length(self) -> Style<StyleBaseState<AttributeGetter<Length>>> {
        let Self(style, _) = self;
        Style(
            style,
            StyleBaseState(Box::new(|x| Attribute::BackgroundSize(Size::Length(x)))),
        )
    }
}

impl Style<StyleBaseState<AttributeGetter<PositionX>>> {
    pub fn left(self) -> Style<StyleBaseState<()>> {
        self.base(PositionX::Left)
    }

    pub fn right(self) -> Style<StyleBaseState<()>> {
        self.base(PositionX::Right)
    }

    pub fn center(self) -> Style<StyleBaseState<()>> {
        self.base(PositionX::Center)
    }
}

impl Style<StyleBaseState<AttributeGetter<PositionY>>> {
    pub fn top(self) -> Style<StyleBaseState<()>> {
        self.base(PositionY::Top)
    }

    pub fn bottom(self) -> Style<StyleBaseState<()>> {
        self.base(PositionY::Bottom)
    }

    pub fn center(self) -> Style<StyleBaseState<()>> {
        self.base(PositionY::Center)
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
    fn inner(self, y: PositionY) -> Style<StyleBaseState<()>> {
        let Self(mut style, x) = self;
        style.insert(attribute::Attribute::BackgroundPosition(XYPosition(x, y)));
        Style(style, StyleBaseState(()))
    }

    pub fn top(self) -> Style<StyleBaseState<()>> {
        self.inner(PositionY::Top)
    }

    pub fn bottom(self) -> Style<StyleBaseState<()>> {
        self.inner(PositionY::Bottom)
    }

    pub fn center(self) -> Style<StyleBaseState<()>> {
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
