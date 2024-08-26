use std::{collections::HashSet, fmt::Display};

use crate::{background, color::Color, length::Length, simple_props::SimpleAttribute};

use super::{AttributeGetter, Style, StyleBaseState};

pub(crate) trait ToAttribute {
    fn attribute(self) -> Attribute;
}

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
    SimpleAttribute(SimpleAttribute),
}

impl Style<StyleBaseState<()>> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashSet::with_capacity(capacity), Default::default())
    }

    pub(crate) fn into_prebase<T>(
        self,
        fun: AttributeGetter<T>,
    ) -> Style<StyleBaseState<AttributeGetter<T>>> {
        Style(self.get_attributes(), StyleBaseState(fun))
    }

    pub fn accent_color(self) -> Style<StyleBaseState<AttributeGetter<Color>>> {
        self.into_prebase(Box::new(Attribute::AccentColor))
    }

    pub fn fontsize(self) -> Style<StyleBaseState<AttributeGetter<Length>>> {
        self.into_prebase(Box::new(Attribute::FontSize))
    }

    pub fn margin(self) -> Style<StyleBaseState<AttributeGetter<Length>>> {
        self.into_prebase(Box::new(Attribute::Margin))
    }

    pub fn padding(self) -> Style<StyleBaseState<AttributeGetter<Length>>> {
        self.into_prebase(Box::new(Attribute::Padding))
    }

    pub fn bottom(self) -> Style<StyleBaseState<AttributeGetter<Length>>> {
        self.into_prebase(Box::new(Attribute::Bottom))
    }

    pub fn height(self) -> Style<StyleBaseState<AttributeGetter<Length>>> {
        self.into_prebase(Box::new(Attribute::Height))
    }

    pub fn width(self) -> Style<StyleBaseState<AttributeGetter<Length>>> {
        self.into_prebase(Box::new(Attribute::Width))
    }
}

impl Display for Style<StyleBaseState<()>> {
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
