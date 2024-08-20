use background::MedBackground;
use color::Color;
use length::Length;
use position::CssPosition;
use std::{collections::HashSet, fmt::Display};

mod background;
mod color;
mod length;
mod position;

#[derive(Default)]
pub struct Style(HashSet<Attribute>);

pub struct MedAttribute<T> {
    core: Style,
    fun: Box<dyn FnOnce(T) -> Attribute>,
}

#[derive(Hash, Eq, PartialEq)]
pub enum Attribute {
    FontSize(Length),
    Position(CssPosition),
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
    BackgroundPosition(background::DuetPosition),
    BackgroundOrigin(background::Origin),
}

impl Style {
    fn med_attr<T>(self, fun: Box<dyn FnOnce(T) -> Attribute>) -> MedAttribute<T> {
        MedAttribute { core: self, fun }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashSet::with_capacity(capacity))
    }

    pub fn background(self) -> MedBackground {
        MedBackground { style: self }
    }

    pub fn fontsize(self) -> MedAttribute<Length> {
        self.med_attr(Box::new(Attribute::FontSize))
    }

    pub fn margin(self) -> MedAttribute<Length> {
        self.med_attr(Box::new(Attribute::Margin))
    }

    pub fn padding(self) -> MedAttribute<Length> {
        self.med_attr(Box::new(Attribute::Padding))
    }

    pub fn bottom(self) -> MedAttribute<Length> {
        self.med_attr(Box::new(Attribute::Bottom))
    }

    pub fn height(self) -> MedAttribute<Length> {
        self.med_attr(Box::new(Attribute::Height))
    }

    pub fn width(self) -> MedAttribute<Length> {
        self.med_attr(Box::new(Attribute::Width))
    }

    pub fn position(self) -> MedAttribute<CssPosition> {
        self.med_attr(Box::new(Attribute::Position))
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self
            .0
            .iter()
            .map(|x| match x {
                Attribute::FontSize(distance) => format!("font-size:{};", distance),
                Attribute::Position(position) => position.to_string(),
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
