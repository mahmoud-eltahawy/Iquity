use std::collections::HashSet;

mod attribute;
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

pub type AttributeGetter<T> = Box<dyn FnOnce(T) -> attribute::Attribute>;
pub type Attributs = HashSet<attribute::Attribute>;

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

impl<T: StyleState> Style<T> {
    fn get_attributes(self) -> Attributs {
        let Self(attrs, _) = self;
        attrs
    }
}
