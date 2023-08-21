use sycamore::prelude::*;

#[component]
pub fn Home<G: Html>(cx: Scope) -> View<G> {
    let query = create_signal(cx, String::new());
    let path = || -> String { format!("/pokemon/{}", &query.get()) };

    view! { cx, div {
        input(bind:value=query, placeholder="Pokemon name")
        a(href=path()) { "Open pokemon info" }
        p { (format!("{}", query)) }
    } }
}
