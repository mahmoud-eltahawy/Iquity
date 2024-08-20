use color::Color;
use length::Length;
use position::Position;
use std::{collections::HashSet, fmt::Display};

mod color;
mod length;
mod position;

pub struct Style(HashSet<CssAttribute>);

pub fn styling() -> Style {
    Style(HashSet::new())
}

impl Style {
    pub fn fontsize(self) -> MedAttribute<Length> {
        MedAttribute {
            style: self,
            fun: Box::new(CssAttribute::FontSize),
        }
    }

    pub fn margin(self) -> MedAttribute<Length> {
        MedAttribute {
            style: self,
            fun: Box::new(CssAttribute::Margin),
        }
    }

    pub fn padding(self) -> MedAttribute<Length> {
        MedAttribute {
            style: self,
            fun: Box::new(CssAttribute::Padding),
        }
    }

    pub fn bottom(self) -> MedAttribute<Length> {
        MedAttribute {
            style: self,
            fun: Box::new(CssAttribute::Bottom),
        }
    }

    pub fn height(self) -> MedAttribute<Length> {
        MedAttribute {
            style: self,
            fun: Box::new(CssAttribute::Height),
        }
    }

    pub fn width(self) -> MedAttribute<Length> {
        MedAttribute {
            style: self,
            fun: Box::new(CssAttribute::Width),
        }
    }

    pub fn position(self) -> MedAttribute<Position> {
        MedAttribute {
            style: self,
            fun: Box::new(CssAttribute::Position),
        }
    }

    pub fn background_color(self) -> MedAttribute<Color> {
        MedAttribute {
            style: self,
            fun: Box::new(CssAttribute::BackgroundColor),
        }
    }
}

pub struct MedAttribute<T> {
    style: Style,
    fun: Box<dyn Fn(T) -> CssAttribute>,
}

impl<T> MedAttribute<T> {
    fn inner(self, position: T) -> Style {
        let Self { mut style, fun } = self;
        let attr = fun(position);
        let success = style.0.insert(attr);
        debug_assert!(success, "value already exists");
        style
    }
}

#[derive(Hash, Eq, PartialEq)]
pub enum CssAttribute {
    FontSize(Length),
    Position(Position),
    BackgroundColor(Color),
    Top(Length),
    Bottom(Length),
    Right(Length),
    Left(Length),
    Height(Length),
    Width(Length),
    Margin(Length),
    Padding(Length),
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self
            .0
            .iter()
            .map(|x| match x {
                CssAttribute::FontSize(distance) => format!("font-size:{};", distance),
                CssAttribute::Position(position) => position.to_string(),
                CssAttribute::BackgroundColor(color) => {
                    format!("background-color:{};", color)
                }
                CssAttribute::Top(distance) => format!("top:{};", distance),
                CssAttribute::Bottom(distance) => format!("bottom:{};", distance),
                CssAttribute::Right(distance) => format!("right:{};", distance),
                CssAttribute::Left(distance) => format!("left:{};", distance),
                CssAttribute::Height(distance) => format!("height:{};", distance),
                CssAttribute::Width(distance) => format!("width:{};", distance),
                CssAttribute::Margin(distance) => format!("margin:{};", distance),
                CssAttribute::Padding(distance) => format!("padding:{};", distance),
            })
            .fold(String::new(), move |acc, x| acc + &x);
        write!(f, "{}", result)
    }
}
