use sycamore::prelude::*;

#[component(inline_props)]
pub fn Pokemon<G: Html>(cx: Scope, id: String) -> View<G> {
    view! { cx, div { "Pokemon : " (id) } }
}
