use std::{collections::HashSet, fmt::Display};

use crate::{background, color::Color, length::Length, simple_props};

use super::{AttributeGetter, PreStyleBase, Style, StyleBaseState};

#[derive(Hash, Eq, PartialEq)]
pub enum Attribute {
    AccentColor(Color),
    FontSize(Length),
    Top(Length),
    Bottom(Length),
    Right(Length),
    Left(Length),
    Height(Length),
    Width(Length),
    Margin(Length),
    Padding(Length),
    BackgroundColor(Color),
    BackgroundImage(String),
    BackgroundPositionX(background::PositionX),
    BackgroundPositionY(background::PositionY),
    BackgroundPosition(background::XYPosition),
    BackgroundSize(background::Size),
    BackgroundBlendMode(simple_props::BlendMode),
    BackgroundRepeat(simple_props::Repeat),
    BackgroundAttachment(simple_props::Attachment),
    BackgroundOrigin(simple_props::Origin),
    BackgroundClip(simple_props::Origin),
    AlignContent(simple_props::AlignContent),
    AlignItems(simple_props::AlignItems),
    AlignSelf(simple_props::AlignSelf),
    All(simple_props::All),
    Position(simple_props::Position),
    BoxDecorationBreak(simple_props::BoxDecorationBreak),
    BoxSizing(simple_props::BoxSizing),
}

impl Style<StyleBaseState> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashSet::with_capacity(capacity), Default::default())
    }

    pub(crate) fn help<T>(self, fun: AttributeGetter<T>) -> Style<PreStyleBase<T>> {
        Style(self.get_attributes(), PreStyleBase(fun))
    }

    pub fn background(self) -> Style<background::BackgroundBaseState> {
        let Self(style, _) = self;
        Style(style, background::BackgroundBaseState)
    }

    pub fn accent_color(self) -> Style<PreStyleBase<Color>> {
        self.help(Box::new(Attribute::AccentColor))
    }

    pub fn fontsize(self) -> Style<PreStyleBase<Length>> {
        self.help(Box::new(Attribute::FontSize))
    }

    pub fn margin(self) -> Style<PreStyleBase<Length>> {
        self.help(Box::new(Attribute::Margin))
    }

    pub fn padding(self) -> Style<PreStyleBase<Length>> {
        self.help(Box::new(Attribute::Padding))
    }

    pub fn bottom(self) -> Style<PreStyleBase<Length>> {
        self.help(Box::new(Attribute::Bottom))
    }

    pub fn height(self) -> Style<PreStyleBase<Length>> {
        self.help(Box::new(Attribute::Height))
    }

    pub fn width(self) -> Style<PreStyleBase<Length>> {
        self.help(Box::new(Attribute::Width))
    }

    pub fn align_content(self) -> Style<PreStyleBase<simple_props::AlignContent>> {
        self.help(Box::new(Attribute::AlignContent))
    }

    pub fn align_items(self) -> Style<PreStyleBase<simple_props::AlignItems>> {
        self.help(Box::new(Attribute::AlignItems))
    }

    pub fn align_self(self) -> Style<PreStyleBase<simple_props::AlignSelf>> {
        self.help(Box::new(Attribute::AlignSelf))
    }

    pub fn all(self) -> Style<PreStyleBase<simple_props::All>> {
        self.help(Box::new(Attribute::All))
    }

    pub fn position(self) -> Style<PreStyleBase<simple_props::Position>> {
        self.help(Box::new(Attribute::Position))
    }

    pub fn box_decoration_break(self) -> Style<PreStyleBase<simple_props::BoxDecorationBreak>> {
        self.help(Box::new(Attribute::BoxDecorationBreak))
    }

    pub fn box_sizing(self) -> Style<PreStyleBase<simple_props::BoxSizing>> {
        self.help(Box::new(Attribute::BoxSizing))
    }
}

impl Display for Style<StyleBaseState> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self
            .0
            .iter()
            .map(|x| match x {
                Attribute::FontSize(x) => format!("font-size:{x};"),
                Attribute::Top(x) => format!("top:{x};"),
                Attribute::Bottom(x) => format!("bottom:{x};"),
                Attribute::Right(x) => format!("right:{x};"),
                Attribute::Left(x) => format!("left:{x};"),
                Attribute::Height(x) => format!("height:{x};"),
                Attribute::Width(x) => format!("width:{x};"),
                Attribute::Margin(x) => format!("margin:{x};"),
                Attribute::Padding(x) => format!("padding:{x};"),
                Attribute::BackgroundColor(x) => {
                    format!("background-color:{x};")
                }
                Attribute::BackgroundImage(x) => format!("background-image:url({x});"),
                Attribute::BackgroundRepeat(x) => {
                    format!("background-repeat:{x};")
                }
                Attribute::BackgroundAttachment(x) => {
                    format!("background-attachment:{x};")
                }
                Attribute::BackgroundPosition(x) => format!("background-position:{x};"),
                Attribute::BackgroundPositionX(x) => format!("background-position-x:{x};"),
                Attribute::BackgroundPositionY(x) => format!("background-position-y:{x};"),
                Attribute::BackgroundOrigin(x) => format!("background-origin:{x};"),
                Attribute::BackgroundClip(x) => format!("background-clip:{x};"),
                Attribute::BackgroundBlendMode(x) => {
                    format!("background-blend-mode:{x};")
                }
                Attribute::BackgroundSize(x) => format!("background-size:{x};"),
                Attribute::AccentColor(x) => format!("accent-color:{x};"),
                Attribute::AlignContent(x) => format!("align-content:{x};"),
                Attribute::AlignItems(x) => format!("align-items:{x};"),
                Attribute::AlignSelf(x) => format!("align-self:{x};"),
                Attribute::All(x) => format!("all:{x};"),
                Attribute::BoxDecorationBreak(x) => format!("box-decoration-break:{x};"),
                Attribute::Position(x) => format!("position:{x};"),
                Attribute::BoxSizing(x) => format!("box-sizing:{x};"),
            })
            .fold(String::new(), move |acc, x| acc + &x);
        write!(f, "{}", result)
    }
}
