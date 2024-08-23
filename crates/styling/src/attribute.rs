use std::{collections::HashSet, fmt::Display};

use crate::{background, color::Color, length::Length, simple_props};

use super::{AttributeGetter, PreStyleBase, Style, StyleBaseState};

#[derive(Hash, Eq, PartialEq)]
pub enum Attribute {
    AccentColor(Color),
    FontSize(Length),
    Position(simple_props::Position),
    AlignContent(simple_props::AlignContent),
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
    BackgroundRepeat(background::Repeat),
    BackgroundAttachment(background::Attachment),
    BackgroundPositionX(background::PositionX),
    BackgroundPositionY(background::PositionY),
    BackgroundPosition(background::XYPosition),
    BackgroundOrigin(background::Origin),
    BackgroundClip(background::Origin),
    BackgroundBlendMode(background::BlendMode),
    BackgroundSize(background::Size),
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

    pub fn align_content(self) -> Style<PreStyleBase<simple_props::AlignContent>> {
        self.help(Box::new(Attribute::AlignContent))
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

    pub fn position(self) -> Style<PreStyleBase<simple_props::Position>> {
        self.help(Box::new(Attribute::Position))
    }
}

impl Display for Style<StyleBaseState> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self
            .0
            .iter()
            .map(|x| match x {
                Attribute::FontSize(distance) => format!("font-size:{};", distance),
                Attribute::Position(position) => format!("position:{};", position),
                Attribute::Top(distance) => format!("top:{};", distance),
                Attribute::Bottom(distance) => format!("bottom:{};", distance),
                Attribute::Right(distance) => format!("right:{};", distance),
                Attribute::Left(distance) => format!("left:{};", distance),
                Attribute::Height(distance) => format!("height:{};", distance),
                Attribute::Width(distance) => format!("width:{};", distance),
                Attribute::Margin(distance) => format!("margin:{};", distance),
                Attribute::Padding(distance) => format!("padding:{};", distance),
                Attribute::BackgroundColor(color) => {
                    format!("background-color:{};", color)
                }
                Attribute::BackgroundImage(url) => format!("background-image:url({});", url),
                Attribute::BackgroundRepeat(repeat) => {
                    format!("background-repeat:{repeat};")
                }
                Attribute::BackgroundAttachment(attachment) => {
                    format!("background-attachment:{attachment};")
                }
                Attribute::BackgroundPosition(p) => format!("background-position:{p};"),
                Attribute::BackgroundPositionX(p) => format!("background-position-x:{p};"),
                Attribute::BackgroundPositionY(p) => format!("background-position-y:{p};"),
                Attribute::BackgroundOrigin(origin) => format!("background-origin:{origin};"),
                Attribute::BackgroundClip(origin) => format!("background-clip:{origin};"),
                Attribute::BackgroundBlendMode(blend) => {
                    format!("background-blend-mode:{blend};")
                }
                Attribute::BackgroundSize(size) => format!("background-size:{size};"),
                Attribute::AccentColor(color) => format!("accent-color:{color};"),
                Attribute::AlignContent(align) => format!("align-content:{align};"),
            })
            .fold(String::new(), move |acc, x| acc + &x);
        write!(f, "{}", result)
    }
}
