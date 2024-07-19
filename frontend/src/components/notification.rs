use leptos::{
    either::Either,
    ev,
    html::{button, div, i, p, span},
    prelude::*,
    spawn::spawn_local,
};

use std::time::Duration;

use gloo_timers::future::sleep;

pub fn notification(theme: impl Fn() -> &'static str + 'static) -> impl IntoView {
    let messages = RwSignal::new(Vec::new());
    let close = move || {
        messages.update(|xs| {
            xs.pop();
        });
    };

    Effect::new(move |_| {
        messages.update(|xs| xs.insert(0, theme()));
        spawn_local(async move {
            sleep(Duration::from_secs(1)).await;
            close();
        });
    });

    move || {
        if !messages.get().is_empty() {
            Either::Left(  button()
        .on(ev::click, move |_| close())
        .class("fixed right-4 top-4 z-50 rounded-md bg-green-500 px-4 py-2 text-white transition hover:bg-green-600")
        .child(div()
            .class("flex items-center space-x-2")
            .child((span()
                .class("text-3xl")
                .child(i()
                    .class("bx bx-check")),
                p()
                    .class("font-bold")
                    .child(move || messages.get().first().unwrap().to_string())))))
        } else {
            Either::Right(())
        }
    }
}
