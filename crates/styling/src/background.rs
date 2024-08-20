use super::{CssAttribute, MedAttribute, Style};
use crate::color::Color;
use std::fmt::Display;

pub struct MedBackground {
    pub(crate) style: Style,
}

impl MedBackground {
    pub(crate) fn med_attr<T>(self, fun: Box<dyn FnOnce(T) -> CssAttribute>) -> MedAttribute<T> {
        let Self { style } = self;
        MedAttribute { core: style, fun }
    }

    pub fn color(self) -> MedAttribute<Color> {
        self.med_attr(Box::new(CssAttribute::BackgroundColor))
    }

    pub fn image(self, source: String) -> Style {
        let Self { mut style } = self;
        style.0.insert(CssAttribute::BackgroundImage(source));
        style
    }

    pub fn repeat(self) -> MedAttribute<Repeat> {
        self.med_attr(Box::new(CssAttribute::BackgroundRepeat))
    }

    pub fn attachment(self) -> MedAttribute<Attachment> {
        self.med_attr(Box::new(CssAttribute::BackgroundAttachment))
    }
}

#[derive(Hash, Eq, PartialEq)]
pub enum Repeat {
    X,
    Y,
    None,
}

#[derive(Hash, Eq, PartialEq)]
pub enum Attachment {
    Fixed,
    Scroll,
}

impl MedAttribute<Attachment> {
    pub fn scroll(self) -> Style {
        self.inner(Attachment::Scroll)
    }

    pub fn fixed(self) -> Style {
        self.inner(Attachment::Fixed)
    }
}

impl Display for Attachment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Attachment::Fixed => "fixed",
            Attachment::Scroll => "scroll",
        };
        write!(f, "{result}")
    }
}

impl MedAttribute<Repeat> {
    pub fn x(self) -> Style {
        self.inner(Repeat::X)
    }
    pub fn y(self) -> Style {
        self.inner(Repeat::Y)
    }
    pub fn none(self) -> Style {
        self.inner(Repeat::None)
    }
}

impl Display for Repeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Repeat::X => "repeat-x",
            Repeat::Y => "repeat-y",
            Repeat::None => "no-repeat",
        };
        write!(f, "{result}")
    }
}
