mod index;
mod not_found;
mod pokemon;

use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

use crate::pages::{index::Home, not_found::NotFound, pokemon::Pokemon};

#[derive(Route, Debug)]
enum AppRoutes {
    #[to("/")]
    Home,
    #[to("/pokemon/<id>")]
    Pokemon { id: String },
    #[not_found]
    NotFound,
}

#[component]
pub fn RouterView<G: Html>(cx: Scope) -> View<G> {
    view! { cx, Router(
        integration=HistoryIntegration::new(),
        view=|cx, route: &ReadSignal<AppRoutes>| view! { cx, (
            {
                let route = route.get();
                match route.as_ref() {
                    AppRoutes::Home => view! { cx, Home },
                    AppRoutes::Pokemon { id } => view! { cx, Pokemon(id=id.clone()) },
                    AppRoutes::NotFound => view! { cx, NotFound },
                }
            }
        ) }
    ) }
}
