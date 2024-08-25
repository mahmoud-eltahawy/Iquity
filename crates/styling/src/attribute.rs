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
    SimpleAttribute(simple_props::SimpleAttribute),
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
        self.help(Box::new(|x| {
            Attribute::SimpleAttribute(x.simple_attribute())
        }))
    }

    pub fn align_items(self) -> Style<PreStyleBase<simple_props::AlignItems>> {
        self.help(Box::new(|x| {
            Attribute::SimpleAttribute(x.simple_attribute())
        }))
    }

    pub fn align_self(self) -> Style<PreStyleBase<simple_props::AlignSelf>> {
        self.help(Box::new(|x| {
            Attribute::SimpleAttribute(x.simple_attribute())
        }))
    }

    pub fn all(self) -> Style<PreStyleBase<simple_props::All>> {
        self.help(Box::new(|x| {
            Attribute::SimpleAttribute(x.simple_attribute())
        }))
    }

    pub fn position(self) -> Style<PreStyleBase<simple_props::Position>> {
        self.help(Box::new(|x| {
            Attribute::SimpleAttribute(x.simple_attribute())
        }))
    }

    pub fn box_decoration_break(self) -> Style<PreStyleBase<simple_props::BoxDecorationBreak>> {
        self.help(Box::new(|x| {
            Attribute::SimpleAttribute(x.simple_attribute())
        }))
    }

    pub fn box_sizing(self) -> Style<PreStyleBase<simple_props::BoxSizing>> {
        self.help(Box::new(|x| {
            Attribute::SimpleAttribute(x.simple_attribute())
        }))
    }
}

impl Display for Style<StyleBaseState> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self
            .0
            .iter()
            .map(|x| match x {
                Attribute::AccentColor(x) => format!("accent-color:{x};"),
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
                Attribute::BackgroundPosition(x) => format!("background-position:{x};"),
                Attribute::BackgroundPositionX(x) => format!("background-position-x:{x};"),
                Attribute::BackgroundPositionY(x) => format!("background-position-y:{x};"),
                Attribute::BackgroundSize(x) => format!("background-size:{x};"),
                Attribute::SimpleAttribute(x) => x.to_string(),
            })
            .fold(String::new(), move |acc, x| acc + &x);
        write!(f, "{}", result)
    }
}
