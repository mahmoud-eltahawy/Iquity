use background::{Attachment, DuetPosition, MedBackground, Repeat};
use color::Color;
use length::Length;
use position::CssPosition;
use std::{collections::HashSet, fmt::Display};

mod background;
mod color;
mod length;
mod position;

#[derive(Default)]
pub struct Style(HashSet<CssAttribute>);

pub struct MedAttribute<T> {
    core: Style,
    fun: Box<dyn FnOnce(T) -> CssAttribute>,
}

#[derive(Hash, Eq, PartialEq)]
pub enum CssAttribute {
    FontSize(Length),
    Position(CssPosition),
    BackgroundColor(Color),
    BackgroundImage(String),
    BackgroundRepeat(Repeat),
    BackgroundAttachment(Attachment),
    BackgroundPosition(DuetPosition),
    Top(Length),
    Bottom(Length),
    Right(Length),
    Left(Length),
    Height(Length),
    Width(Length),
    Margin(Length),
    Padding(Length),
}

impl Style {
    fn med_attr<T>(self, fun: Box<dyn FnOnce(T) -> CssAttribute>) -> MedAttribute<T> {
        MedAttribute { core: self, fun }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashSet::with_capacity(capacity))
    }

    pub fn background(self) -> MedBackground {
        MedBackground { style: self }
    }

    pub fn fontsize(self) -> MedAttribute<Length> {
        self.med_attr(Box::new(CssAttribute::FontSize))
    }

    pub fn margin(self) -> MedAttribute<Length> {
        self.med_attr(Box::new(CssAttribute::Margin))
    }

    pub fn padding(self) -> MedAttribute<Length> {
        self.med_attr(Box::new(CssAttribute::Padding))
    }

    pub fn bottom(self) -> MedAttribute<Length> {
        self.med_attr(Box::new(CssAttribute::Bottom))
    }

    pub fn height(self) -> MedAttribute<Length> {
        self.med_attr(Box::new(CssAttribute::Height))
    }

    pub fn width(self) -> MedAttribute<Length> {
        self.med_attr(Box::new(CssAttribute::Width))
    }

    pub fn position(self) -> MedAttribute<CssPosition> {
        self.med_attr(Box::new(CssAttribute::Position))
    }
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
                CssAttribute::BackgroundImage(url) => format!("background-image:url({});", url),
                CssAttribute::BackgroundRepeat(repeat) => {
                    format!("background-repeat:{repeat};")
                }
                CssAttribute::BackgroundAttachment(attachment) => {
                    format!("background-attachment:{attachment};")
                }
                CssAttribute::Top(distance) => format!("top:{};", distance),
                CssAttribute::Bottom(distance) => format!("bottom:{};", distance),
                CssAttribute::Right(distance) => format!("right:{};", distance),
                CssAttribute::Left(distance) => format!("left:{};", distance),
                CssAttribute::Height(distance) => format!("height:{};", distance),
                CssAttribute::Width(distance) => format!("width:{};", distance),
                CssAttribute::Margin(distance) => format!("margin:{};", distance),
                CssAttribute::Padding(distance) => format!("padding:{};", distance),
                CssAttribute::BackgroundPosition(p) => format!("background-position:{p};"),
            })
            .fold(String::new(), move |acc, x| acc + &x);
        write!(f, "{}", result)
    }
}

impl<T> MedAttribute<T> {
    fn inner(self, position: T) -> Style {
        let Self { mut core, fun } = self;
        let attr = fun(position);
        core.0.insert(attr);
        //TODO : add warn at compile time when attr is added twice
        core
    }
}
