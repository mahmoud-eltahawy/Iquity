use super::{attribute::Attribute, Style};
use crate::{
    attribute, color::Color, length::Length, AttributeGetter, Attributs, PreBaseState,
    StyleBaseState, StyleState,
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
pub struct BackgroundBaseState<T>(pub T);
pub struct BackgroundSizeState;
pub struct BackgroundPreXPosition;
impl StyleState for BackgroundSizeState {}
impl StyleState for BackgroundBaseState<()> {}
impl StyleState for BackgroundPreXPosition {}
impl StyleState for PositionX {}
impl<T> StyleState for BackgroundBaseState<AttributeGetter<T>> {}

impl<T> PreBaseState<T, BackgroundBaseState<()>>
    for Style<BackgroundBaseState<AttributeGetter<T>>>
{
    fn destruct(self) -> (Attributs, AttributeGetter<T>) {
        let Self(attrs, BackgroundBaseState(fun)) = self;
        (attrs, fun)
    }
}

impl Style<BackgroundBaseState<()>> {
    pub(crate) fn into_prebase<T>(
        self,
        fun: AttributeGetter<T>,
    ) -> Style<BackgroundBaseState<AttributeGetter<T>>> {
        let Self(style, _) = self;
        Style(style, BackgroundBaseState(fun))
    }

    pub fn base(self) -> Style<StyleBaseState<()>> {
        let Self(style, _) = self;
        Style(style, Default::default())
    }

    pub fn color(self) -> Style<BackgroundBaseState<AttributeGetter<Color>>> {
        self.into_prebase(Box::new(Attribute::BackgroundColor))
    }

    pub fn size(self) -> Style<BackgroundSizeState> {
        let Style(style, _) = self;
        Style(style, BackgroundSizeState)
    }

    pub fn image(self, source: &str) -> Style<BackgroundBaseState<()>> {
        let Self(mut style, _) = self;
        style.insert(Attribute::BackgroundImage(source.to_string()));
        Style(style, BackgroundBaseState(()))
    }

    pub fn position(self) -> Style<BackgroundPreXPosition> {
        let Self(style, _) = self;
        Style(style, BackgroundPreXPosition)
    }

    pub fn position_x(self) -> Style<BackgroundBaseState<AttributeGetter<PositionX>>> {
        let Self(style, _) = self;
        Style(
            style,
            BackgroundBaseState(Box::new(Attribute::BackgroundPositionX)),
        )
    }

    pub fn position_y(self) -> Style<BackgroundBaseState<AttributeGetter<PositionY>>> {
        let Self(style, _) = self;
        Style(
            style,
            BackgroundBaseState(Box::new(Attribute::BackgroundPositionY)),
        )
    }
}

impl Style<BackgroundSizeState> {
    fn inner(self, size: Size) -> Style<BackgroundBaseState<()>> {
        let Self(mut style, _) = self;
        style.insert(Attribute::BackgroundSize(size));
        Style(style, BackgroundBaseState(()))
    }

    pub fn initial(self) -> Style<BackgroundBaseState<()>> {
        self.inner(Size::Initial)
    }

    pub fn auto(self) -> Style<BackgroundBaseState<()>> {
        self.inner(Size::Auto)
    }

    pub fn inherit(self) -> Style<BackgroundBaseState<()>> {
        self.inner(Size::Inherit)
    }

    pub fn contain(self) -> Style<BackgroundBaseState<()>> {
        self.inner(Size::Contain)
    }

    pub fn cover(self) -> Style<BackgroundBaseState<()>> {
        self.inner(Size::Cover)
    }

    pub fn length(self) -> Style<BackgroundBaseState<AttributeGetter<Length>>> {
        let Self(style, _) = self;
        Style(
            style,
            BackgroundBaseState(Box::new(|x| Attribute::BackgroundSize(Size::Length(x)))),
        )
    }
}

impl Style<BackgroundBaseState<AttributeGetter<PositionX>>> {
    pub fn left(self) -> Style<BackgroundBaseState<()>> {
        self.base(PositionX::Left)
    }

    pub fn right(self) -> Style<BackgroundBaseState<()>> {
        self.base(PositionX::Right)
    }

    pub fn center(self) -> Style<BackgroundBaseState<()>> {
        self.base(PositionX::Center)
    }
}

impl Style<BackgroundBaseState<AttributeGetter<PositionY>>> {
    pub fn top(self) -> Style<BackgroundBaseState<()>> {
        self.base(PositionY::Top)
    }

    pub fn bottom(self) -> Style<BackgroundBaseState<()>> {
        self.base(PositionY::Bottom)
    }

    pub fn center(self) -> Style<BackgroundBaseState<()>> {
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
    fn inner(self, y: PositionY) -> Style<BackgroundBaseState<()>> {
        let Self(mut style, x) = self;
        style.insert(attribute::Attribute::BackgroundPosition(XYPosition(x, y)));
        Style(style, BackgroundBaseState(()))
    }

    pub fn top(self) -> Style<BackgroundBaseState<()>> {
        self.inner(PositionY::Top)
    }

    pub fn bottom(self) -> Style<BackgroundBaseState<()>> {
        self.inner(PositionY::Bottom)
    }

    pub fn center(self) -> Style<BackgroundBaseState<()>> {
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
