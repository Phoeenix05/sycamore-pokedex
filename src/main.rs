use reqwasm::http::Request;
use sycamore::futures::create_resource;
use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use sycamore_router::{HistoryIntegration, Route, Router, RouterProps};

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    Home,
    #[to("/pokemon/<id>")]
    Pokemon { id: String },
    #[not_found]
    NotFound,
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Router(
            integration=HistoryIntegration::new(),
            view=|cx, route: &ReadSignal<AppRoutes>| view! { cx, (
                (match route.get().as_ref() {
                    AppRoutes::Home => view! { cx, div { "Home" } },
                    AppRoutes::Pokemon { id } => view! { cx, div { "Pokemon" } },
                    AppRoutes::NotFound => view! { cx, div { "Not Found" } },
                })
            ) }
        )
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(App);
}
