use std::time::Duration;

use gloo_timers::future::sleep;
use leptos::{
    either::Either,
    html::{div, span},
    prelude::*,
    spawn::spawn_local,
};

use crate::Markdown;

pub fn _which_slide() -> impl IntoView {
    let show = RwSignal::new(false);
    let markdown = use_context::<Markdown>().unwrap();
    Effect::new(move |_| {
        markdown.current.track();
        markdown.len.track();
        show.set(true);
        spawn_local(async move {
            sleep(Duration::from_secs(1)).await;
            show.set(false);
        });
    });
    move || {
        if show.get() {
            Either::Left(div()
            .class("bg-transparent absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 z-30 w-3/12 text-lime-700 text-center p-5 text-3xl")
            .child((
                "{ ",
                span().child(move || markdown.current.get()),
                " / ",
                span().child(move || markdown.len.get()),
                " }",
            )))
        } else {
            Either::Right(())
        }
    }
}
