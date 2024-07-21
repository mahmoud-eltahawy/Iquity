use leptos::html::{div, p};
use leptos::prelude::*;

use crate::local_config::{Config, THEMES, THEMES_SIZE};

pub(crate) fn help() -> impl IntoView {
    let conf = use_context::<Config>().unwrap();
    let theme = move || THEMES[conf.theme_index.get() % THEMES_SIZE];
    div()
        .attr("data-theme", theme)
        .class("grid grid-cols-1 gap-2 absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 z-50 w-9/12 h-5/6 text-center border-2 rounded-lg p-5 text-2xl")
        .child((
            p().child("p => print to pdf"),
            p().child("j => next theme"),
            p().child("k => previous theme"),
            p().child("L => next slide"),
            p().child("H => previous slide"),
            p().child("= or + => increase font size"),
            p().child("- or _ => decrease font size"),
            p().child("? or / => show this help message"),
            p().child("esc => to hide this message"),
        ))
}
