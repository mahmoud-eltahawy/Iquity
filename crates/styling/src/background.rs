use super::{attribute::Attribute, Style};
use crate::{
    attribute,
    color::Color,
    length::Length,
    simple_props::{self, ToAttribute},
    AttributeGetter, Attributs, PreBaseState, StyleBaseState, StyleState,
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
pub struct BackgroundBaseState;
pub struct BackgroundSizeState;
pub struct BackgroundPreXPosition;
pub struct PreBackgroundBase<T>(AttributeGetter<T>);
impl StyleState for BackgroundSizeState {}
impl StyleState for BackgroundBaseState {}
impl StyleState for BackgroundPreXPosition {}
impl StyleState for PositionX {}
impl<T> StyleState for PreBackgroundBase<T> {}

impl<T> PreBaseState<T, BackgroundBaseState> for Style<PreBackgroundBase<T>> {
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

    pub fn repeat(self) -> Style<PreBackgroundBase<simple_props::BackgroundRepeat>> {
        self.pre_base(Box::new(ToAttribute::attribute))
    }

    pub fn origin(self) -> Style<PreBackgroundBase<simple_props::BackgroundOrigin>> {
        self.pre_base(Box::new(ToAttribute::attribute))
    }

    pub fn clip(self) -> Style<PreBackgroundBase<simple_props::BackgroundClip>> {
        self.pre_base(Box::new(ToAttribute::attribute))
    }

    pub fn blend_mode(self) -> Style<PreBackgroundBase<simple_props::BackgroundBlendMode>> {
        self.pre_base(Box::new(ToAttribute::attribute))
    }

    pub fn attachment(self) -> Style<PreBackgroundBase<simple_props::BackgroundAttachment>> {
        self.pre_base(Box::new(ToAttribute::attribute))
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

impl Style<PreBackgroundBase<simple_props::BackgroundOrigin>> {
    pub fn padding_box(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundOrigin::PaddingBox)
    }

    pub fn border_box(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundOrigin::BorderBox)
    }

    pub fn content_box(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundOrigin::ContentBox)
    }

    pub fn initial(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundOrigin::Initial)
    }

    pub fn inherit(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundOrigin::Inherit)
    }
}

impl Style<PreBackgroundBase<simple_props::BackgroundClip>> {
    pub fn padding_box(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundClip::PaddingBox)
    }

    pub fn border_box(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundClip::BorderBox)
    }

    pub fn content_box(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundClip::ContentBox)
    }

    pub fn initial(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundClip::Initial)
    }

    pub fn inherit(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundClip::Inherit)
    }
}

impl Style<PreBackgroundBase<simple_props::BackgroundBlendMode>> {
    pub fn normal(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundBlendMode::Normal)
    }

    pub fn multiply(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundBlendMode::Multiply)
    }

    pub fn screen(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundBlendMode::Screen)
    }

    pub fn overlay(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundBlendMode::Overlay)
    }

    pub fn darken(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundBlendMode::Darken)
    }

    pub fn lighten(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundBlendMode::Lighten)
    }

    pub fn color_dodge(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundBlendMode::ColorDodge)
    }

    pub fn saturation(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundBlendMode::Saturation)
    }

    pub fn color(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundBlendMode::Color)
    }

    pub fn luminosity(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundBlendMode::Luminosity)
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
        style.insert(attribute::Attribute::BackgroundPosition(XYPosition(x, y)));
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

impl Style<PreBackgroundBase<simple_props::BackgroundAttachment>> {
    pub fn scroll(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundAttachment::Scroll)
    }

    pub fn fixed(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundAttachment::Fixed)
    }

    pub fn local(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundAttachment::Local)
    }

    pub fn initial(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundAttachment::Initial)
    }

    pub fn inherit(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundAttachment::Inherit)
    }
}

impl Style<PreBackgroundBase<simple_props::BackgroundRepeat>> {
    pub fn x(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundRepeat::RepeatX)
    }
    pub fn y(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundRepeat::RepeatY)
    }
    pub fn none(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundRepeat::NoRepeat)
    }
    pub fn space(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundRepeat::Space)
    }
    pub fn round(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundRepeat::Round)
    }
    pub fn initial(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundRepeat::Initial)
    }
    pub fn inherit(self) -> Style<BackgroundBaseState> {
        self.base(simple_props::BackgroundRepeat::Inherit)
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
