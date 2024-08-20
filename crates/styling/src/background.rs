use super::{Attribute, MedAttribute, Style};
use crate::color::Color;
use std::fmt::Display;

pub struct MedBackground {
    pub(crate) style: Style,
}

pub struct MedPosition {
    style: Style,
}

pub struct MedPositionX {
    style: Style,
    x: PositionX,
}

#[derive(Hash, Eq, PartialEq)]
pub struct DuetPosition(PositionX, PositionY);

#[derive(Hash, Eq, PartialEq)]
pub enum PositionY {
    Top,
    Bottom,
    Center,
}

#[derive(Hash, Eq, PartialEq)]
pub enum PositionX {
    Left,
    Right,
    Center,
}

#[derive(Hash, Eq, PartialEq)]
pub enum Repeat {
    X,
    Y,
    None,
}

#[derive(Hash, Eq, PartialEq)]
pub enum Origin {
    PaddingBox,
    BorderBox,
    ContentBox,
    Initial,
    Inherit,
}

#[derive(Hash, Eq, PartialEq)]
pub enum Attachment {
    Fixed,
    Scroll,
}

impl MedBackground {
    pub fn med_attr<T>(self, fun: Box<dyn FnOnce(T) -> Attribute>) -> MedAttribute<T> {
        let Self { style } = self;
        MedAttribute { core: style, fun }
    }

    pub fn color(self) -> MedAttribute<Color> {
        self.med_attr(Box::new(Attribute::BackgroundColor))
    }

    pub fn image(self, source: &str) -> Style {
        let Self { mut style } = self;
        style
            .0
            .insert(Attribute::BackgroundImage(source.to_string()));
        style
    }

    pub fn repeat(self) -> MedAttribute<Repeat> {
        self.med_attr(Box::new(Attribute::BackgroundRepeat))
    }

    pub fn origin(self) -> MedAttribute<Origin> {
        self.med_attr(Box::new(Attribute::BackgroundOrigin))
    }

    pub fn attachment(self) -> MedAttribute<Attachment> {
        self.med_attr(Box::new(Attribute::BackgroundAttachment))
    }

    pub fn position(self) -> MedPosition {
        let Self { style } = self;
        MedPosition { style }
    }

    pub fn position_x(self) -> MedAttribute<PositionX> {
        let Self { style } = self;
        MedAttribute {
            core: style,
            fun: Box::new(Attribute::BackgroundPositionX),
        }
    }

    pub fn position_y(self) -> MedAttribute<PositionY> {
        let Self { style } = self;
        MedAttribute {
            core: style,
            fun: Box::new(Attribute::BackgroundPositionY),
        }
    }
}

impl MedAttribute<PositionX> {
    pub fn left(self) -> Style {
        self.inner(PositionX::Left)
    }

    pub fn right(self) -> Style {
        self.inner(PositionX::Right)
    }

    pub fn center(self) -> Style {
        self.inner(PositionX::Center)
    }
}

impl MedAttribute<PositionY> {
    pub fn top(self) -> Style {
        self.inner(PositionY::Top)
    }

    pub fn bottom(self) -> Style {
        self.inner(PositionY::Bottom)
    }

    pub fn center(self) -> Style {
        self.inner(PositionY::Center)
    }
}

impl MedAttribute<Origin> {
    pub fn padding_box(self) -> Style {
        self.inner(Origin::PaddingBox)
    }

    pub fn border_box(self) -> Style {
        self.inner(Origin::BorderBox)
    }

    pub fn content_box(self) -> Style {
        self.inner(Origin::ContentBox)
    }

    pub fn initial(self) -> Style {
        self.inner(Origin::Initial)
    }

    pub fn inherit(self) -> Style {
        self.inner(Origin::Inherit)
    }
}

impl MedPosition {
    fn inner(self, x: PositionX) -> MedPositionX {
        let Self { style } = self;
        MedPositionX { style, x }
    }

    pub fn left(self) -> MedPositionX {
        self.inner(PositionX::Left)
    }

    pub fn right(self) -> MedPositionX {
        self.inner(PositionX::Right)
    }

    pub fn center(self) -> MedPositionX {
        self.inner(PositionX::Center)
    }
}

impl MedPositionX {
    fn inner(self, position: PositionY) -> Style {
        let Self { mut style, x: left } = self;
        style
            .0
            .insert(Attribute::BackgroundPosition(DuetPosition(left, position)));
        style
    }

    pub fn top(self) -> Style {
        self.inner(PositionY::Top)
    }

    pub fn bottom(self) -> Style {
        self.inner(PositionY::Bottom)
    }

    pub fn center(self) -> Style {
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
impl Display for DuetPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let DuetPosition(x, y) = self;
        write!(f, "{x} {y}",)
    }
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

impl Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Origin::PaddingBox => "padding-box",
            Origin::BorderBox => "border-box",
            Origin::ContentBox => "content-box",
            Origin::Initial => "initial",
            Origin::Inherit => "inherit",
        };
        write!(f, "{result}")
    }
}
