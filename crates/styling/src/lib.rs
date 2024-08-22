use color::Color;
use length::Length;
use position::CssPosition;
use std::{collections::HashSet, fmt::Display};

mod background;
mod color;
mod length;
mod position;

pub trait StyleState {}
pub trait PreState<T, R: StyleState + Default>: Sized {
    fn destruct(self) -> (Attributs, AttributeGetter<T>);
    fn base(self, position: T) -> Style<R> {
        let (mut core, fun) = self.destruct();
        let attr = fun(position);
        core.insert(attr);
        Style(core, R::default())
    }
}

pub type AttributeGetter<T> = Box<dyn FnOnce(T) -> Attribute>;
pub type Attributs = HashSet<Attribute>;

#[derive(Default)]
pub struct StyleBaseState;
pub struct PreStyleBase<T>(AttributeGetter<T>);

impl<T> StyleState for PreStyleBase<T> {}

impl StyleState for StyleBaseState {}

impl Default for Style<StyleBaseState> {
    fn default() -> Self {
        Self(HashSet::new(), Default::default())
    }
}

pub struct Style<T: StyleState>(Attributs, T);

impl<T> PreState<T, StyleBaseState> for Style<PreStyleBase<T>> {
    fn destruct(self) -> (Attributs, AttributeGetter<T>) {
        let Self(attrs, PreStyleBase(fun)) = self;
        (attrs, fun)
    }
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
    BackgroundPosition(background::XYPosition),
    BackgroundOrigin(background::Origin),
    BackgroundClip(background::Origin),
    BackgroundBlendMode(background::BlendMode),
    BackgroundSize(background::Size),
}

impl Style<StyleBaseState> {
    fn med_attr<T>(self, fun: AttributeGetter<T>) -> Style<PreStyleBase<T>> {
        let Self(core, _) = self;
        Style(core, PreStyleBase(fun))
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashSet::with_capacity(capacity), Default::default())
    }

    pub fn background(self) -> Style<background::BackgroundBaseState> {
        let Self(style, _) = self;
        Style(style, background::BackgroundBaseState)
    }

    pub fn fontsize(self) -> Style<PreStyleBase<Length>> {
        self.med_attr(Box::new(Attribute::FontSize))
    }

    pub fn margin(self) -> Style<PreStyleBase<Length>> {
        self.med_attr(Box::new(Attribute::Margin))
    }

    pub fn padding(self) -> Style<PreStyleBase<Length>> {
        self.med_attr(Box::new(Attribute::Padding))
    }

    pub fn bottom(self) -> Style<PreStyleBase<Length>> {
        self.med_attr(Box::new(Attribute::Bottom))
    }

    pub fn height(self) -> Style<PreStyleBase<Length>> {
        self.med_attr(Box::new(Attribute::Height))
    }

    pub fn width(self) -> Style<PreStyleBase<Length>> {
        self.med_attr(Box::new(Attribute::Width))
    }

    pub fn position(self) -> Style<PreStyleBase<CssPosition>> {
        self.med_attr(Box::new(Attribute::Position))
    }
}

impl Display for Style<StyleBaseState> {
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
                Attribute::BackgroundClip(origin) => format!("background-clip:{origin};"),
                Attribute::BackgroundBlendMode(blend) => {
                    format!("background-blend-mode:{blend};")
                }
                Attribute::BackgroundSize(size) => format!("background-size:{size};"),
            })
            .fold(String::new(), move |acc, x| acc + &x);
        write!(f, "{}", result)
    }
}
