use std::collections::HashSet;

mod attribute;
mod background;
mod color;
mod length;
mod simple_props;

pub type AttributeGetter<T> = Box<dyn FnOnce(T) -> attribute::Attribute>;
pub type Attributs = HashSet<attribute::Attribute>;

pub trait StyleState {}
pub trait PreBaseState<T, R: StyleState + Default>: Sized {
    fn destruct(self) -> (Attributs, AttributeGetter<T>);
    fn base(self, position: T) -> Style<R> {
        let (mut core, fun) = self.destruct();
        let attr = fun(position);
        core.insert(attr);
        Style(core, R::default())
    }
}

#[derive(Default)]
pub struct StyleBaseState<T>(T);
pub struct Style<T: StyleState>(Attributs, T);

impl StyleState for StyleBaseState<()> {}
impl<T> StyleState for StyleBaseState<AttributeGetter<T>> {}
impl<T> PreBaseState<T, StyleBaseState<()>> for Style<StyleBaseState<AttributeGetter<T>>> {
    fn destruct(self) -> (Attributs, AttributeGetter<T>) {
        let Self(attrs, StyleBaseState(fun)) = self;
        (attrs, fun)
    }
}

impl<T: StyleState> Style<T> {
    fn get_attributes(self) -> Attributs {
        let Self(attrs, _) = self;
        attrs
    }
}

impl Default for Style<StyleBaseState<()>> {
    fn default() -> Self {
        Self(HashSet::new(), Default::default())
    }
}
