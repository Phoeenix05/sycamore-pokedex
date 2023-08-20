use sycamore::prelude::*;

#[component]
pub fn NotFound<G: Html>(cx: Scope) -> View<G> {
    view! { cx, div { "Not Found" } }
}
